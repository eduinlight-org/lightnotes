use std::sync::Arc;

use crate::domain::ports::{
  AuthTicketRepository, GoogleCodeExchanger, GoogleIdentityVerifier, RefreshTokenRepository, TokenIssuer, UserRepository,
};
use crate::domain::user::{AuthError, NativeAuthTicket, TicketStatus, User};
use crate::infrastructure::auth::pkce::challenge_for;
use crate::infrastructure::auth::secrets::generate_secret;

use super::google_sign_in::{issue_session, IssuedSession};

const TICKET_TTL_MS: i64 = 10 * 60 * 1000;

pub struct StartedNativeAuth {
  pub ticket: String,
  pub authorize_url: String,
}

pub enum PollOutcome {
  Pending,
  Complete(Box<NativeSession>),
  Failed,
}

pub struct NativeSession {
  pub user: User,
  pub access_token: String,
  pub refresh_token: String,
  pub expires_in_secs: i64,
}

pub struct NativeAuthHandler {
  pub ticket_repo: Arc<dyn AuthTicketRepository>,
  pub exchanger: Arc<dyn GoogleCodeExchanger>,
  pub verifier: Arc<dyn GoogleIdentityVerifier>,
  pub user_repo: Arc<dyn UserRepository>,
  pub refresh_repo: Arc<dyn RefreshTokenRepository>,
  pub token_issuer: Arc<dyn TokenIssuer>,
  pub refresh_ttl_secs: i64,
}

impl NativeAuthHandler {
  pub async fn start(&self, now_ms: i64) -> Result<StartedNativeAuth, AuthError> {
    let ticket_id = generate_secret();
    let state = generate_secret();
    let pkce_verifier = generate_secret();

    let ticket = NativeAuthTicket {
      id: ticket_id.clone(),
      state: state.clone(),
      pkce_verifier: pkce_verifier.clone(),
      status: TicketStatus::Pending,
      user_id: None,
      access_token: None,
      refresh_token: None,
      expires_in_secs: None,
      created_at_ms: now_ms,
    };

    self.ticket_repo.insert(&ticket).await?;

    let authorize_url = self.exchanger.authorize_url(&state, &challenge_for(&pkce_verifier)).await;

    Ok(StartedNativeAuth {
      ticket: ticket_id,
      authorize_url,
    })
  }

  pub async fn complete(&self, code: &str, state: &str, now_ms: i64) -> Result<(), AuthError> {
    let ticket = self.ticket_repo.find_by_state(state).await?.ok_or(AuthError::InvalidToken)?;

    if ticket.status != TicketStatus::Pending {
      return Err(AuthError::InvalidToken);
    }

    if now_ms - ticket.created_at_ms > TICKET_TTL_MS {
      self.ticket_repo.fail(&ticket.id).await?;
      return Err(AuthError::Expired);
    }

    let outcome = self.issue_for(code, &ticket.pkce_verifier, now_ms).await;

    match outcome {
      Ok(session) => {
        self
          .ticket_repo
          .complete(
            &ticket.id,
            &session.user.id,
            &session.access_token,
            &session.refresh_token,
            session.expires_in_secs,
          )
          .await?;

        Ok(())
      }
      Err(err) => {
        self.ticket_repo.fail(&ticket.id).await?;
        Err(err)
      }
    }
  }

  async fn issue_for(&self, code: &str, pkce_verifier: &str, now_ms: i64) -> Result<IssuedSession, AuthError> {
    let id_token = self.exchanger.exchange_code(code, pkce_verifier).await?;
    let identity = self.verifier.verify(&id_token).await?;
    let user = self.user_repo.upsert_by_google_sub(&identity, now_ms).await?;

    issue_session(
      self.refresh_repo.as_ref(),
      self.token_issuer.as_ref(),
      user,
      None,
      self.refresh_ttl_secs,
      now_ms,
    )
    .await
  }

  pub async fn poll(&self, ticket_id: &str) -> Result<PollOutcome, AuthError> {
    let Some(ticket) = self.ticket_repo.take(ticket_id).await? else {
      return Err(AuthError::InvalidToken);
    };

    match ticket.status {
      TicketStatus::Pending => Ok(PollOutcome::Pending),
      TicketStatus::Failed => Ok(PollOutcome::Failed),
      TicketStatus::Complete => {
        let user_id = ticket.user_id.ok_or(AuthError::InvalidToken)?;
        let user = self.user_repo.find_by_id(&user_id).await?.ok_or(AuthError::InvalidToken)?;

        Ok(PollOutcome::Complete(Box::new(NativeSession {
          user,
          access_token: ticket.access_token.ok_or(AuthError::InvalidToken)?,
          refresh_token: ticket.refresh_token.ok_or(AuthError::InvalidToken)?,
          expires_in_secs: ticket.expires_in_secs.unwrap_or_default(),
        })))
      }
    }
  }
}
