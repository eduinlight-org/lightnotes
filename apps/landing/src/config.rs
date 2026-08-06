pub const APP_URL: &str = match option_env!("LANDING_APP_URL") {
  Some(url) => url,
  None => "https://app-lightnotes.eduindev.com",
};

pub const GITHUB_URL: &str = match option_env!("LANDING_GITHUB_URL") {
  Some(url) => url,
  None => "https://github.com/eduinlight-org/lightnotes",
};

pub const CONTACT_EMAIL: &str = match option_env!("LANDING_CONTACT_EMAIL") {
  Some(email) => email,
  None => "eduinlight@gmail.com",
};

pub const DIOXUS_URL: &str = "https://dioxuslabs.com/";

pub fn app_host() -> &'static str {
  APP_URL.trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/')
}

pub fn github_setup_url() -> String {
  format!("{GITHUB_URL}#getting-started")
}

pub fn github_build_url() -> String {
  format!("{GITHUB_URL}#building-for-release")
}

pub fn github_license_url() -> String {
  format!("{GITHUB_URL}/blob/main/LICENSE")
}

pub fn contact_mailto() -> String {
  format!("mailto:{CONTACT_EMAIL}")
}
