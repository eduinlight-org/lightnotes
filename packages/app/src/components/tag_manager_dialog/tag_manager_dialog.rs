use super::use_tag_manager_dialog::{use_tag_manager_dialog, use_tag_manager_panel};
use crate::components::{ConfirmDialog, ManagerPanelSkeleton};
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_icons::lucide::{Tag as TagIcon, Trash2, X};
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::dialog::{Dialog, DialogDescription, DialogTitle};
use ui::components::input::Input;

#[derive(PartialEq, Clone, Props)]
pub struct TagManagerPanelProps {
  pub on_select: Option<EventHandler<()>>,
}

#[component]
pub fn TagManagerPanel(props: TagManagerPanelProps) -> Element {
  let TagManagerPanelProps { on_select } = props;
  let mut panel = use_tag_manager_panel(on_select);

  if !panel.ready {
    return rsx! {
        ManagerPanelSkeleton {}
    };
  }

  let store = panel.store;
  let mut draft = panel.draft;
  let is_mobile = panel.is_mobile;
  let tags = store.tags();
  let pending_delete_id = (panel.pending_delete)();
  let pending_delete_name = pending_delete_id
    .as_ref()
    .and_then(|id| tags.iter().find(|tag| &tag.id == id))
    .map(|tag| format!("#{}", tag.name))
    .unwrap_or_else(|| t!("tags-delete-fallback-name"));

  let input_row_class = if is_mobile() {
    "flex h-11 items-center gap-2 rounded-[11px] border border-[var(--primary-color-6)] bg-[var(--primary-color-2)] px-[13px] focus-within:border-[var(--accent)]"
  } else {
    "flex h-[38px] items-center gap-2 rounded-lg border border-[var(--primary-color-6)] bg-[var(--primary-color-2)] px-[11px] focus-within:border-[var(--accent)]"
  };

  let panel_class = "flex min-h-0 flex-1 flex-col gap-3";
  let list_class = "flex min-h-0 flex-1 flex-col gap-1.5 overflow-y-auto";

  rsx! {
      div { class: panel_class,
          if !is_mobile() {
              p { class: "text-sm text-[var(--secondary-color-5)]",
                  {t!("tags-hint", count: tags.len() as i64)}
              }
          }
          div { class: input_row_class,
              if is_mobile() {
                  span { class: "flex-none text-[var(--secondary-color-5)]", "#" }
              } else {
                  TagIcon { class: "flex-none text-[var(--secondary-color-5)]", size: "14px" }
              }
              Input {
                  class: "h-full flex-1 border-none bg-transparent p-0 text-[13px] shadow-none [outline:none] hover:bg-transparent focus:bg-transparent focus:shadow-none",
                  placeholder: t!("tags-new-placeholder"),
                  value: draft(),
                  oninput: move |event: FormEvent| draft.set(event.value()),
                  onkeydown: move |event: KeyboardEvent| {
                      if event.key() == Key::Enter {
                          panel.submit();
                      }
                  },
              }
              Button {
                  variant: ButtonVariant::Primary,
                  size: ButtonSize::Sm,
                  class: "flex-none border border-[var(--accent)] bg-transparent text-[var(--accent)] hover:bg-[color-mix(in_srgb,var(--accent)_12%,transparent)]",
                  onclick: move |_| panel.submit(),
                  {t!("action-add")}
              }
          }
          div { class: list_class,
              if tags.is_empty() {
                  p { class: "px-2 py-6 text-center text-sm text-[var(--secondary-color-5)]", {t!("tags-empty")} }
              }
              for tag in tags {
                  {
                      let tag_id = tag.id.clone();
                      let tag_id_for_delete = tag.id.clone();
                      let count = store.tag_note_count(&tag.id);
                      let row_class = if is_mobile() {
                          "flex cursor-pointer items-center gap-[11px] rounded-[11px] bg-[var(--primary-color-2)] px-[14px] py-[13px]"
                      } else {
                          "flex cursor-pointer items-center gap-3 rounded-lg px-2 py-2 hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]"
                      };
                      rsx! {
                          div {
                              key: "{tag.id}",
                              class: row_class,
                              onclick: {
                                  let tag_id = tag_id.clone();
                                  move |_| panel.select_tag(tag_id.clone())
                              },
                              span { class: "h-2 w-2 flex-none rounded-full bg-[var(--accent)]" }
                              span {
                                  class: "flex-1 text-sm font-medium text-[var(--secondary-color)]",
                                  "#{tag.name}"
                              }
                              span { class: "text-xs text-[var(--secondary-color-5)]",
                                  if is_mobile() { {t!("notes-count", count: count as i64)} } else { "{count}" }
                              }
                              button {
                                  class: "flex-none text-[var(--secondary-color-5)] hover:text-[#ec6a5e]",
                                  "aria-label": t!("tags-delete"),
                                  onclick: move |event: MouseEvent| {
                                      event.stop_propagation();
                                      panel.request_delete(&tag_id_for_delete);
                                  },
                                  Trash2 { size: if is_mobile() { "17px" } else { "15px" } }
                              }
                          }
                      }
                  }
              }
          }
          ConfirmDialog {
              open: pending_delete_id.is_some(),
              on_open_change: move |_| panel.cancel_delete(),
              icon: rsx! { TagIcon { size: "20px", stroke: "var(--primary-error-color)" } },
              title: t!("tags-delete-title"),
              description: rsx! {
                  span {
                      strong { "{pending_delete_name}" }
                      " — "
                      {t!("tags-delete-description")}
                  }
              },
              on_confirm: move |_| {
                  if let Some(tag_id) = pending_delete_id.clone() {
                      panel.confirm_delete(&tag_id);
                  }
              },
          }
      }
  }
}

#[component]
pub fn TagManagerDialog() -> Element {
  let mut dialog = use_tag_manager_dialog();

  rsx! {
      Dialog {
          open: dialog.open(),
          on_open_change: move |value| dialog.set_open(value),
          class: "max-h-[calc(100vh-32px)] flex flex-col overflow-hidden",
          div { class: "flex flex-none items-center justify-between",
              DialogTitle { class: "text-[var(--secondary-color)] font-medium!", {t!("tags-manage-title")} }
              button {
                  "aria-label": t!("action-close"),
                  onclick: move |_| dialog.close(),
                  X { size: "18px", stroke: "var(--secondary-color-5)" }
              }
          }
          DialogDescription { class: "sr-only", {t!("tags-manage-description")} }
          TagManagerPanel { on_select: move |_| dialog.close() }
          div { class: "flex flex-none justify-end pt-2",
              Button {
                  variant: ButtonVariant::Secondary,
                  size: ButtonSize::Sm,
                  class: "border border-[var(--primary-color-6)] bg-transparent hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]",
                  onclick: move |_| dialog.close(),
                  {t!("action-close")}
              }
          }
      }
  }
}
