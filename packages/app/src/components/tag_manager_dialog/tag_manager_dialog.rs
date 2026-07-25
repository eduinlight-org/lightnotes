use crate::state::{use_notes, use_ui, NoteFilter};
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Tag as TagIcon, Trash2, X};
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::dialog::{Dialog, DialogDescription, DialogTitle};
use ui::components::input::Input;
use ui::components::sidebar::use_is_mobile;

#[component]
pub fn TagManagerPanel(on_select: Option<EventHandler<()>>) -> Element {
  let mut store = use_notes();
  let mut draft = use_signal(String::new);
  let tags = store.tags();
  let is_mobile = use_is_mobile();

  let mut submit = move || {
    let name = draft();
    if name.trim().is_empty() {
      return;
    }
    store.create_tag(name);
    draft.set(String::new());
  };

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
                  "Create a tag, tap one to filter, or delete it everywhere. {tags.len()} tags in use."
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
                  placeholder: "New tag name…",
                  value: draft(),
                  oninput: move |event: FormEvent| draft.set(event.value()),
                  onkeydown: move |event: KeyboardEvent| {
                      if event.key() == Key::Enter {
                          submit();
                      }
                  },
              }
              Button {
                  variant: ButtonVariant::Primary,
                  size: ButtonSize::Sm,
                  class: "flex-none border border-[var(--accent)] bg-transparent text-[var(--accent)] hover:bg-[color-mix(in_srgb,var(--accent)_12%,transparent)]",
                  onclick: move |_| submit(),
                  "Add"
              }
          }
          div { class: list_class,
              if tags.is_empty() {
                  p { class: "px-2 py-6 text-center text-sm text-[var(--secondary-color-5)]", "No tags yet." }
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
                                  move |_| {
                                      store.set_filter(NoteFilter::Tag(tag_id.clone()));
                                      navigator().push(Route::Notes {});
                                      if let Some(handler) = &on_select {
                                          handler.call(());
                                      }
                                  }
                              },
                              span { class: "h-2 w-2 flex-none rounded-full bg-[var(--accent)]" }
                              span {
                                  class: "flex-1 text-sm font-medium text-[var(--secondary-color)]",
                                  "#{tag.name}"
                              }
                              span { class: "text-xs text-[var(--secondary-color-5)]",
                                  if is_mobile() { "{count} notes" } else { "{count}" }
                              }
                              button {
                                  class: "flex-none text-[var(--secondary-color-5)] hover:text-[#ec6a5e]",
                                  "aria-label": "Delete tag",
                                  onclick: move |event: MouseEvent| {
                                      event.stop_propagation();
                                      store.delete_tag(&tag_id_for_delete);
                                  },
                                  Trash2 { size: if is_mobile() { "17px" } else { "15px" } }
                              }
                          }
                      }
                  }
              }
          }
      }
  }
}

#[component]
pub fn TagManagerDialog() -> Element {
  let mut ui = use_ui();
  let open = (ui.tags_open)();

  rsx! {
      Dialog {
          open,
          on_open_change: move |value| ui.tags_open.set(value),
          class: "max-h-[calc(100vh-32px)] flex flex-col overflow-hidden",
          div { class: "flex flex-none items-center justify-between",
              DialogTitle { class: "text-[var(--secondary-color)] font-medium!", "Manage tags" }
              button {
                  "aria-label": "Close",
                  onclick: move |_| ui.tags_open.set(false),
                  X { size: "18px", stroke: "var(--secondary-color-5)" }
              }
          }
          DialogDescription { class: "sr-only", "Create, filter by, or delete tags" }
          TagManagerPanel { on_select: move |_| ui.tags_open.set(false) }
          div { class: "flex flex-none justify-end pt-2",
              Button {
                  variant: ButtonVariant::Secondary,
                  size: ButtonSize::Sm,
                  class: "border border-[var(--primary-color-6)] bg-transparent hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]",
                  onclick: move |_| ui.tags_open.set(false),
                  "Close"
              }
          }
      }
  }
}
