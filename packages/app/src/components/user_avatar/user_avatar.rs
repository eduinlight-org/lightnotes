use crate::state::use_auth;
use dioxus::prelude::*;
use ui::components::avatar::{Avatar, AvatarFallback, AvatarImageSize, AvatarShape, ImageAvatar};

fn initials_from(name: &str) -> String {
  let words: Vec<&str> = name.split_whitespace().filter(|word| !word.is_empty()).collect();

  let letters: String = words
    .iter()
    .take(2)
    .filter_map(|word| word.chars().next())
    .flat_map(|letter| letter.to_uppercase())
    .collect();

  if letters.is_empty() {
    "?".to_string()
  } else {
    letters
  }
}

fn initials_from_email(email: &str) -> String {
  let local = email.split('@').next().unwrap_or(email);
  let normalized = local.replace(['.', '_', '-', '+'], " ");

  initials_from(&normalized)
}

#[component]
pub fn UserAvatar() -> Element {
  let auth = use_auth();
  let Some(user) = auth.user() else {
    return rsx! {};
  };

  let label = user.name.clone().unwrap_or_else(|| user.email.clone());
  let initials = match user.name.as_deref() {
    Some(name) if !name.trim().is_empty() => initials_from(name),
    _ => initials_from_email(&user.email),
  };

  rsx! {
      if let Some(picture) = user.picture {
          ImageAvatar {
              src: "{picture}",
              alt: "{label}",
              size: AvatarImageSize::Small,
              shape: AvatarShape::Circle,
              "{initials}"
          }
      } else {
          Avatar {
              size: AvatarImageSize::Small,
              shape: AvatarShape::Circle,
              AvatarFallback { "{initials}" }
          }
      }
  }
}
