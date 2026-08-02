const INDEX_TEMPLATE: &str = include_str!("index.html");
const SPLASH_STYLE: &str = include_str!("splash.css");
const SPLASH_SCRIPT: &str = include_str!("splash.js");
const SPLASH_BODY: &str = include_str!("splash.html");

pub const DISMISS_SPLASH_JS: &str = r#"
let splash = document.getElementById('app-splash');
if (splash) {
  splash.setAttribute('data-hidden', '');
  setTimeout(function () { splash.remove(); }, 240);
}
"#;

pub const SPLASH_BACKGROUND_RGBA: (u8, u8, u8, u8) = (22, 24, 38, 255);

pub fn custom_index(stylesheets: &[String]) -> String {
  let links = stylesheets
    .iter()
    .map(|href| format!("<link rel=\"stylesheet\" href=\"{href}\" type=\"text/css\" />"))
    .collect::<Vec<_>>()
    .join("\n    ");

  INDEX_TEMPLATE
    .replace("{splash_script}", SPLASH_SCRIPT)
    .replace("{stylesheets}", &links)
    .replace("{splash_style}", SPLASH_STYLE)
    .replace("{splash_body}", SPLASH_BODY)
}

#[cfg(test)]
mod tests {
  use super::*;

  const WEB_INDEX: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../apps/web/index.html"
  ));

  #[test]
  fn custom_index_keeps_the_anchors_dioxus_desktop_requires() {
    let index = custom_index(&["/assets/main.css".to_string()]);

    assert!(index.contains("</head>"));
    assert!(index.contains("</body>"));
    assert!(index.contains("<div id=\"main\"></div>"));
    assert!(index.contains("<link rel=\"stylesheet\" href=\"/assets/main.css\""));
    assert!(index.contains("id=\"app-splash\""));
    assert!(!index.contains("{splash_script}"));
    assert!(!index.contains("{splash_style}"));
    assert!(!index.contains("{splash_body}"));
    assert!(!index.contains("{stylesheets}"));
  }

  #[test]
  fn web_index_carries_the_same_splash_as_the_native_shell() {
    assert!(
      WEB_INDEX.contains(SPLASH_STYLE.trim_end()),
      "apps/web/index.html has drifted from packages/app/src/boot/splash.css"
    );
    assert!(
      WEB_INDEX.contains(SPLASH_SCRIPT.trim_end()),
      "apps/web/index.html has drifted from packages/app/src/boot/splash.js"
    );
    assert!(
      WEB_INDEX.contains(SPLASH_BODY.trim_end()),
      "apps/web/index.html has drifted from packages/app/src/boot/splash.html"
    );
  }
}
