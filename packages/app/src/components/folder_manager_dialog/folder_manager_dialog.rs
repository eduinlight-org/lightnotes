use super::use_folder_manager_dialog::{use_folder_manager_dialog, use_folder_manager_panel};
use crate::components::{ConfirmDialog, ResponsivePopoverContent, ResponsivePopoverRoot, ResponsivePopoverTrigger};
use crate::state::FolderIcon;
use dioxus::prelude::*;
use dioxus_icons::lucide::{
  Archive, Bookmark, BookOpen, Briefcase, Calendar, Camera, ChevronDown, Code, Folder as FolderGlyph,
  Gift, Globe, Heart, House, Inbox, Lock, Music, Notebook, Palette, Rocket, Settings, Star, Trash2,
  User, X,
};
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::dialog::{Dialog, DialogDescription, DialogTitle};
use ui::components::input::Input;
use ui::components::sidebar::use_is_mobile;

const ALL_FOLDER_ICONS: [FolderIcon; 20] = [
  FolderIcon::Inbox,
  FolderIcon::Briefcase,
  FolderIcon::User,
  FolderIcon::BookOpen,
  FolderIcon::Notebook,
  FolderIcon::Archive,
  FolderIcon::House,
  FolderIcon::Star,
  FolderIcon::Heart,
  FolderIcon::Settings,
  FolderIcon::Calendar,
  FolderIcon::Camera,
  FolderIcon::Music,
  FolderIcon::Code,
  FolderIcon::Palette,
  FolderIcon::Gift,
  FolderIcon::Globe,
  FolderIcon::Lock,
  FolderIcon::Rocket,
  FolderIcon::Bookmark,
];

fn folder_icon(icon: FolderIcon, size: &'static str) -> Element {
  match icon {
    FolderIcon::Inbox => rsx! { Inbox { size } },
    FolderIcon::Briefcase => rsx! { Briefcase { size } },
    FolderIcon::User => rsx! { User { size } },
    FolderIcon::BookOpen => rsx! { BookOpen { size } },
    FolderIcon::Notebook => rsx! { Notebook { size } },
    FolderIcon::Archive => rsx! { Archive { size } },
    FolderIcon::House => rsx! { House { size } },
    FolderIcon::Star => rsx! { Star { size } },
    FolderIcon::Heart => rsx! { Heart { size } },
    FolderIcon::Settings => rsx! { Settings { size } },
    FolderIcon::Calendar => rsx! { Calendar { size } },
    FolderIcon::Camera => rsx! { Camera { size } },
    FolderIcon::Music => rsx! { Music { size } },
    FolderIcon::Code => rsx! { Code { size } },
    FolderIcon::Palette => rsx! { Palette { size } },
    FolderIcon::Gift => rsx! { Gift { size } },
    FolderIcon::Globe => rsx! { Globe { size } },
    FolderIcon::Lock => rsx! { Lock { size } },
    FolderIcon::Rocket => rsx! { Rocket { size } },
    FolderIcon::Bookmark => rsx! { Bookmark { size } },
  }
}

#[derive(PartialEq, Clone, Props)]
pub struct IconPickerProps {
  pub current: FolderIcon,
  pub onselect: EventHandler<FolderIcon>,
}

#[component]
fn IconPicker(props: IconPickerProps) -> Element {
  let IconPickerProps { current, onselect } = props;
  let is_mobile = use_is_mobile();

  rsx! {
      ResponsivePopoverRoot {
          ResponsivePopoverTrigger {
              class: "flex flex-none items-center gap-0.5 border-none bg-transparent p-0 text-[var(--accent)]",
              title: "Change icon",
              {folder_icon(current, "16px")}
              ChevronDown { size: "10px" }
          }
          ResponsivePopoverContent { title: "Change icon", class: "w-52 items-stretch p-2",
              div { class: if is_mobile() { "grid grid-cols-4 gap-2" } else { "grid grid-cols-4 gap-1" },
                  for icon in ALL_FOLDER_ICONS {
                      button {
                          key: "{icon:?}",
                          class: if is_mobile() {
                              "flex h-12 items-center justify-center rounded-[11px] bg-[var(--primary-color-2)] text-[var(--secondary-color)]"
                          } else {
                              "flex h-9 items-center justify-center rounded-md text-[var(--secondary-color)] hover:bg-[var(--primary-color-4)]"
                          },
                          onclick: move |_| onselect.call(icon),
                          {folder_icon(icon, if is_mobile() { "20px" } else { "17px" })}
                      }
                  }
              }
          }
      }
  }
}

#[derive(PartialEq, Clone, Props)]
pub struct FolderManagerPanelProps {
  pub on_select: Option<EventHandler<()>>,
}

#[component]
pub fn FolderManagerPanel(props: FolderManagerPanelProps) -> Element {
  let FolderManagerPanelProps { on_select } = props;
  let mut panel = use_folder_manager_panel(on_select);
  let mut store = panel.store;
  let mut draft = panel.draft;
  let mut draft_icon = panel.draft_icon;
  let is_mobile = panel.is_mobile;
  let folders = store.folders();
  let pending_delete_id = (panel.pending_delete)();
  let pending_delete_name = pending_delete_id
    .as_ref()
    .and_then(|id| folders.iter().find(|folder| &folder.id == id))
    .map(|folder| folder.name.clone())
    .unwrap_or_else(|| "This folder".to_string());

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
                          panel.submit();
                      }
                  },
              }
              Button {
                  variant: ButtonVariant::Primary,
                  size: ButtonSize::Sm,
                  class: "flex-none border border-[var(--accent)] bg-transparent text-[var(--accent)] hover:bg-[color-mix(in_srgb,var(--accent)_12%,transparent)]",
                  onclick: move |_| panel.submit(),
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
                                  onclick: move |_| panel.select_folder(folder_id_for_select.clone()),
                                  "{count}"
                              }
                              button {
                                  class: "flex-none text-[var(--secondary-color-5)] hover:text-[#ec6a5e]",
                                  "aria-label": "Delete folder",
                                  onclick: move |_| panel.request_delete(&folder_id_for_delete),
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
              icon: rsx! { FolderGlyph { size: "20px", stroke: "var(--primary-error-color)" } },
              title: "Delete folder?",
              description: rsx! {
                  span {
                      strong { "{pending_delete_name}" }
                      " — Its notes will move to no folder. This can't be undone."
                  }
              },
              on_confirm: move |_| {
                  if let Some(folder_id) = pending_delete_id.clone() {
                      panel.confirm_delete(&folder_id);
                  }
              },
          }
      }
  }
}

#[component]
pub fn FolderManagerDialog() -> Element {
  let mut dialog = use_folder_manager_dialog();

  rsx! {
      Dialog {
          open: dialog.open(),
          on_open_change: move |value| dialog.set_open(value),
          class: "max-h-[calc(100vh-32px)] flex flex-col overflow-hidden",
          div { class: "flex flex-none items-center justify-between",
              DialogTitle { class: "text-[var(--secondary-color)] font-medium!", "Manage folders" }
              button {
                  "aria-label": "Close",
                  onclick: move |_| dialog.close(),
                  X { size: "18px", stroke: "var(--secondary-color-5)" }
              }
          }
          DialogDescription { class: "sr-only", "Create, rename, or delete folders" }
          FolderManagerPanel { on_select: move |_| dialog.close() }
          div { class: "flex flex-none justify-end pt-2",
              Button {
                  variant: ButtonVariant::Secondary,
                  size: ButtonSize::Sm,
                  class: "border border-[var(--primary-color-6)] bg-transparent hover:bg-[color-mix(in_srgb,var(--secondary-color)_5%,transparent)]",
                  onclick: move |_| dialog.close(),
                  "Close"
              }
          }
      }
  }
}
