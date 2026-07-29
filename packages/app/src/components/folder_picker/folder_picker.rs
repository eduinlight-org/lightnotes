use super::use_folder_picker::use_folder_picker;
use crate::components::{ResponsivePopoverContent, ResponsivePopoverRoot, ResponsivePopoverTrigger};
use crate::state::{FolderIcon, Note};
use dioxus::prelude::*;
use dioxus_icons::lucide::{
  Archive, Bookmark, BookOpen, Briefcase, Calendar, Camera, ChevronDown, Code, FileText, Gift,
  Globe, Heart, House, Inbox, Lock, Music, Notebook, Palette, Rocket, Settings, Star, User,
};
use ui::components::popover::ContentAlign;

fn folder_icon(icon: FolderIcon) -> Element {
  match icon {
    FolderIcon::Inbox => rsx! { Inbox { size: "14px" } },
    FolderIcon::Briefcase => rsx! { Briefcase { size: "14px" } },
    FolderIcon::User => rsx! { User { size: "14px" } },
    FolderIcon::BookOpen => rsx! { BookOpen { size: "14px" } },
    FolderIcon::Notebook => rsx! { Notebook { size: "14px" } },
    FolderIcon::Archive => rsx! { Archive { size: "14px" } },
    FolderIcon::House => rsx! { House { size: "14px" } },
    FolderIcon::Star => rsx! { Star { size: "14px" } },
    FolderIcon::Heart => rsx! { Heart { size: "14px" } },
    FolderIcon::Settings => rsx! { Settings { size: "14px" } },
    FolderIcon::Calendar => rsx! { Calendar { size: "14px" } },
    FolderIcon::Camera => rsx! { Camera { size: "14px" } },
    FolderIcon::Music => rsx! { Music { size: "14px" } },
    FolderIcon::Code => rsx! { Code { size: "14px" } },
    FolderIcon::Palette => rsx! { Palette { size: "14px" } },
    FolderIcon::Gift => rsx! { Gift { size: "14px" } },
    FolderIcon::Globe => rsx! { Globe { size: "14px" } },
    FolderIcon::Lock => rsx! { Lock { size: "14px" } },
    FolderIcon::Rocket => rsx! { Rocket { size: "14px" } },
    FolderIcon::Bookmark => rsx! { Bookmark { size: "14px" } },
  }
}

#[derive(PartialEq, Clone, Props)]
pub struct FolderPickerProps {
  pub note: Note,
}

#[component]
pub fn FolderPicker(props: FolderPickerProps) -> Element {
  let FolderPickerProps { note } = props;
  let mut picker = use_folder_picker();
  let note_id = note.id.clone();
  let folders = picker.store.folders();
  let current = folders.iter().find(|folder| Some(&folder.id) == note.folder_id.as_ref()).cloned();
  let folder_name = current.as_ref().map(|folder| folder.name.clone()).unwrap_or_else(|| "No folder".to_string());

  rsx! {
      ResponsivePopoverRoot {
          open: (picker.open)(),
          on_open_change: move |value| picker.open.set(value),
          ResponsivePopoverTrigger {
              class: "flex h-8 items-center gap-1.5 rounded-md border border-[var(--primary-color-6)] px-2.5 text-xs text-[var(--secondary-color)]",
              title: "Move to folder",
              span { style: "display:flex;color:var(--accent)",
                  if let Some(folder) = current.as_ref() {
                      {folder_icon(folder.icon)}
                  } else {
                      FileText { size: "14px" }
                  }
              }
              "{folder_name}"
              ChevronDown { size: "11px" }
          }
          ResponsivePopoverContent {
              title: "Move to folder",
              align: ContentAlign::Start,
              class: "w-52 items-stretch gap-1 p-1.5 text-left",
              div {
                  class: if note.folder_id.is_none() {
                      "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1.5 text-sm bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] text-[var(--accent)]"
                  } else {
                      "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1.5 text-sm text-[var(--secondary-color)] hover:bg-[var(--primary-color-4)]"
                  },
                  onclick: {
                      let note_id = note_id.clone();
                      move |_| picker.move_to_folder(&note_id, None)
                  },
                  FileText { size: "15px" }
                  "No folder"
              }
              for folder in folders {
                  {
                      let folder_id = folder.id.clone();
                      let is_active = note.folder_id.as_deref() == Some(folder.id.as_str());
                      let class = if is_active {
                          "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1.5 text-sm bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] text-[var(--accent)]"
                      } else {
                          "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1.5 text-sm text-[var(--secondary-color)] hover:bg-[var(--primary-color-4)]"
                      };
                      rsx! {
                          div {
                              key: "{folder.id}",
                              class,
                              onclick: {
                                  let note_id = note_id.clone();
                                  let folder_id = folder_id.clone();
                                  move |_| picker.move_to_folder(&note_id, Some(folder_id.clone()))
                              },
                              {folder_icon(folder.icon)}
                              "{folder.name}"
                          }
                      }
                  }
              }
          }
      }
  }
}
