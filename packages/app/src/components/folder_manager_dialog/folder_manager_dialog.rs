use crate::state::{use_notes, use_ui, FolderIcon, NoteFilter};
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::{
  Archive, BookOpen, Briefcase, ChevronDown, Inbox, Notebook, Trash2, User, X,
};
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::dialog::{Dialog, DialogDescription, DialogTitle};
use ui::components::input::Input;
use ui::components::popover::{PopoverContent, PopoverRoot, PopoverTrigger};
use ui::components::sidebar::use_is_mobile;

const ALL_FOLDER_ICONS: [FolderIcon; 6] = [
  FolderIcon::Inbox,
  FolderIcon::Briefcase,
  FolderIcon::User,
  FolderIcon::BookOpen,
  FolderIcon::Notebook,
  FolderIcon::Archive,
];

fn folder_icon(icon: FolderIcon, size: &'static str) -> Element {
  match icon {
    FolderIcon::Inbox => rsx! { Inbox { size } },
    FolderIcon::Briefcase => rsx! { Briefcase { size } },
    FolderIcon::User => rsx! { User { size } },
    FolderIcon::BookOpen => rsx! { BookOpen { size } },
    FolderIcon::Notebook => rsx! { Notebook { size } },
    FolderIcon::Archive => rsx! { Archive { size } },
  }
}

#[component]
fn IconPicker(current: FolderIcon, onselect: EventHandler<FolderIcon>) -> Element {
  rsx! {
      PopoverRoot {
          PopoverTrigger {
              class: "flex flex-none items-center gap-0.5 border-none bg-transparent p-0 text-[var(--accent)]",
              title: "Change icon",
              {folder_icon(current, "16px")}
              ChevronDown { size: "10px" }
          }
          PopoverContent { class: "w-44 items-stretch p-2",
              div { class: "grid grid-cols-3 gap-1",
                  for icon in ALL_FOLDER_ICONS {
                      button {
                          key: "{icon:?}",
                          class: "flex h-9 items-center justify-center rounded-md text-[var(--secondary-color)] hover:bg-[var(--primary-color-4)]",
                          onclick: move |_| onselect.call(icon),
                          {folder_icon(icon, "17px")}
                      }
                  }
              }
          }
      }
  }
}

#[component]
pub fn FolderManagerPanel(on_select: Option<EventHandler<()>>) -> Element {
  let mut store = use_notes();
  let mut draft = use_signal(String::new);
  let mut draft_icon = use_signal(|| FolderIcon::Notebook);
  let folders = store.folders();
  let is_mobile = use_is_mobile();

  let mut submit = move || {
    let name = draft();
    if name.trim().is_empty() {
      return;
    }
    store.create_folder_with_icon(name, draft_icon());
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
                  "Rename in place, or delete a folder to move its notes to no folder. {folders.len()} folders."
              }
          }
          div { class: input_row_class,
              IconPicker { current: draft_icon(), onselect: move |icon| draft_icon.set(icon) }
              Input {
                  class: "h-full flex-1 border-none bg-transparent p-0 text-[13px] shadow-none [outline:none] hover:bg-transparent focus:bg-transparent focus:shadow-none",
                  placeholder: "New folder name…",
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
              if folders.is_empty() {
                  p { class: "px-2 py-6 text-center text-sm text-[var(--secondary-color-5)]", "No folders yet." }
              }
              for folder in folders {
                  {
                      let folder_id = folder.id.clone();
                      let folder_id_for_delete = folder.id.clone();
                      let folder_id_for_select = folder.id.clone();
                      let count = store.folder_note_count(&folder.id);
                      let row_class = if is_mobile() {
                          "flex items-center gap-[11px] rounded-[11px] bg-[var(--primary-color-2)] px-[14px] py-[13px]"
                      } else {
                          "flex items-center gap-3 rounded-lg px-2 py-2 hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]"
                      };
                      rsx! {
                          div {
                              key: "{folder.id}",
                              class: row_class,
                              IconPicker {
                                  current: folder.icon,
                                  onselect: {
                                      let folder_id = folder_id.clone();
                                      move |icon| store.set_folder_icon(&folder_id, icon)
                                  },
                              }
                              Input {
                                  class: "h-auto flex-1 border-none bg-transparent p-0 text-sm font-medium shadow-none [outline:none] hover:bg-transparent focus:bg-transparent focus:shadow-none",
                                  value: folder.name.clone(),
                                  oninput: {
                                      let folder_id = folder_id.clone();
                                      move |event: FormEvent| store.rename_folder(&folder_id, event.value())
                                  },
                              }
                              span {
                                  class: "flex-none cursor-pointer text-xs text-[var(--secondary-color-5)]",
                                  onclick: move |_| {
                                      store.set_filter(NoteFilter::Folder(folder_id_for_select.clone()));
                                      navigator().push(Route::Notes {});
                                      if let Some(handler) = &on_select {
                                          handler.call(());
                                      }
                                  },
                                  "{count}"
                              }
                              button {
                                  class: "flex-none text-[var(--secondary-color-5)] hover:text-[#ec6a5e]",
                                  "aria-label": "Delete folder",
                                  onclick: move |_| store.delete_folder(&folder_id_for_delete),
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
pub fn FolderManagerDialog() -> Element {
  let mut ui = use_ui();
  let open = (ui.folders_open)();

  rsx! {
      Dialog {
          open,
          on_open_change: move |value| ui.folders_open.set(value),
          class: "max-h-[calc(100vh-32px)] flex flex-col overflow-hidden",
          div { class: "flex flex-none items-center justify-between",
              DialogTitle { class: "text-[var(--secondary-color)] font-medium!", "Manage folders" }
              button {
                  "aria-label": "Close",
                  onclick: move |_| ui.folders_open.set(false),
                  X { size: "18px", stroke: "var(--secondary-color-5)" }
              }
          }
          DialogDescription { class: "sr-only", "Create, rename, or delete folders" }
          FolderManagerPanel { on_select: move |_| ui.folders_open.set(false) }
          div { class: "flex flex-none justify-end pt-2",
              Button {
                  variant: ButtonVariant::Secondary,
                  size: ButtonSize::Sm,
                  class: "border border-[var(--primary-color-6)] bg-transparent hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]",
                  onclick: move |_| ui.folders_open.set(false),
                  "Close"
              }
          }
      }
  }
}
