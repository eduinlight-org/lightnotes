use dioxus::prelude::*;
use ui::components::sidebar::use_is_mobile;

#[derive(Clone, Copy)]
pub struct ResponsivePopoverCtx {
  pub is_mobile: Signal<bool>,
  pub open: Memo<bool>,
  pub set_open: Callback<bool>,
}

fn use_controlled_open(
  open: ReadSignal<Option<bool>>,
  default_open: bool,
  on_open_change: Callback<bool>,
) -> (Memo<bool>, Callback<bool>) {
  let mut internal_open = use_signal(|| open.cloned().unwrap_or(default_open));
  let value = use_memo(move || open.cloned().unwrap_or_else(&*internal_open));

  let set_value = use_callback(move |value: bool| {
    internal_open.set(value);
    on_open_change.call(value);
  });

  (value, set_value)
}

pub fn use_responsive_popover(
  open: ReadSignal<Option<bool>>,
  default_open: bool,
  on_open_change: Callback<bool>,
) -> ResponsivePopoverCtx {
  let is_mobile = use_is_mobile();
  let (open, set_open) = use_controlled_open(open, default_open, on_open_change);

  ResponsivePopoverCtx { is_mobile, open, set_open }
}
