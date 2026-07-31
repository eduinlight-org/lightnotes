use dioxus::prelude::*;
use dioxus_i18n::t;

const SETUP_GUIDE_URL: &str = "https://github.com/eduinlight/lightnotes#getting-started";

const COMMANDS: [(&str, &str); 4] = [
  ("cp .env.dist .env", ""),
  ("make docker-up", "selfhost-comment-docker"),
  ("make api-dev", "selfhost-comment-api"),
  ("make app-web-dev", "selfhost-comment-app"),
];

#[component]
pub fn SelfHostSection() -> Element {
  rsx! {
      section {
          id: "selfhost",
          class: "wrap grid items-start gap-10 pt-[72px] pb-10 min-[720px]:grid-cols-2 min-[720px]:gap-x-[clamp(28px,5vw,88px)]",
          div {
              span { class: "kicker", {t!("selfhost-kicker")} }
              h2 {
                  class: "text-[clamp(28px,3vw,36px)] leading-[1.16] tracking-[-0.012em]",
                  {t!("selfhost-title")}
              }
              p {
                  class: "mt-[22px] max-w-[48ch] text-[16px] leading-[1.68] text-copy",
                  {t!("selfhost-description-before")}
                  " "
                  code { class: "font-mono text-[14px] text-accent-300", "dx build" }
                  {t!("selfhost-description-after")}
              }
              div {
                  class: "mt-[26px] flex flex-wrap gap-3",
                  a { class: "btn btn-primary btn-md", href: SETUP_GUIDE_URL, {t!("selfhost-setup-guide")} }
              }
          }
          div {
              class: "grid gap-0.5 rounded-md bg-surface px-6 py-[22px] font-mono text-[13.5px] leading-[2] text-neutral-300 shadow-sm",
              for (command , comment_key) in COMMANDS {
                  div {
                      span { class: "text-neutral-600", "$ " }
                      "{command} "
                      if !comment_key.is_empty() {
                          span { class: "text-neutral-600", "# " {t!(comment_key)} }
                      }
                  }
              }
          }
      }
  }
}
