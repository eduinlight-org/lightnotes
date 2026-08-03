mod login_button;
mod use_login_button;

#[cfg(not(target_arch = "wasm32"))]
mod use_login_button_native;
#[cfg(target_arch = "wasm32")]
mod use_login_button_web;

pub use login_button::LoginButton;
