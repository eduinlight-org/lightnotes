use dioxus::prelude::*;
use ui::components::alert_dialog::{
  AlertDialog, AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogDescription,
  AlertDialogTitle,
};

#[derive(PartialEq, Clone, Props)]
pub struct ConfirmDialogProps {
  pub open: ReadSignal<Option<bool>>,
  #[props(default)]
  pub on_open_change: Callback<bool>,
  pub icon: Element,
  pub title: String,
  pub description: Element,
  pub on_confirm: EventHandler<MouseEvent>,
}

#[component]
pub fn ConfirmDialog(props: ConfirmDialogProps) -> Element {
  let ConfirmDialogProps { open, on_open_change, icon, title, description, on_confirm } = props;

  rsx! {
      AlertDialog { open, on_open_change,
          div { class: "flex items-center gap-2.5",
              {icon}
              AlertDialogTitle { "{title}" }
          }
          AlertDialogDescription { {description} }
          AlertDialogActions {
              AlertDialogCancel { "Cancel" }
              AlertDialogAction { on_click: on_confirm, "Delete" }
          }
      }
  }
}
