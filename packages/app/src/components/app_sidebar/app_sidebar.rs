use crate::state::{use_notes, NoteFilter};
use dioxus::prelude::*;
use dioxus_icons::lucide::{FolderPlus, NotebookText, Tag as TagIcon};
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::sidebar::{
  Sidebar, SidebarContent, SidebarGroup, SidebarGroupLabel, SidebarHeader, SidebarMenu,
  SidebarMenuItem,
};

#[component]
pub fn AppSidebar() -> Element {
  let mut store = use_notes();
  let filter = store.filter();

  rsx! {
      Sidebar {
          SidebarHeader {
              div { class: "flex items-center gap-2 px-2 py-1 text-white",
                  NotebookText { size: "18px" }
                  span { class: "font-semibold", "Notes" }
              }
          }
          SidebarContent {
              SidebarGroup {
                  SidebarMenu {
                      SidebarMenuItem {
                          Button {
                              variant: if filter == NoteFilter::All { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
                              class: "w-full justify-start",
                              onclick: move |_| store.set_filter(NoteFilter::All),
                              "All notes"
                          }
                      }
                  }
              }
              SidebarGroup {
                  div { class: "flex items-center justify-between px-2",
                      SidebarGroupLabel { "Folders" }
                      Button {
                          variant: ButtonVariant::Ghost,
                          size: ButtonSize::IconXs,
                          "aria-label": "New folder",
                          onclick: move |_| {
                              store.create_folder("New folder".to_string());
                          },
                          FolderPlus { size: "14px" }
                      }
                  }
                  SidebarMenu {
                      for folder in store.folders() {
                          {
                              let folder_id = folder.id.clone();
                              let is_active = filter == NoteFilter::Folder(folder.id.clone());
                              rsx! {
                                  SidebarMenuItem { key: "{folder.id}",
                                      Button {
                                          variant: if is_active { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
                                          class: "w-full justify-start",
                                          onclick: move |_| store.set_filter(NoteFilter::Folder(folder_id.clone())),
                                          "{folder.name}"
                                      }
                                  }
                              }
                          }
                      }
                  }
              }
              SidebarGroup {
                  div { class: "flex items-center justify-between px-2",
                      SidebarGroupLabel { "Tags" }
                      Button {
                          variant: ButtonVariant::Ghost,
                          size: ButtonSize::IconXs,
                          "aria-label": "New tag",
                          onclick: move |_| {
                              store.create_tag("new-tag".to_string());
                          },
                          TagIcon { size: "14px" }
                      }
                  }
                  SidebarMenu {
                      for tag in store.tags() {
                          {
                              let tag_id = tag.id.clone();
                              let is_active = filter == NoteFilter::Tag(tag.id.clone());
                              rsx! {
                                  SidebarMenuItem { key: "{tag.id}",
                                      Button {
                                          variant: if is_active { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
                                          class: "w-full justify-start",
                                          onclick: move |_| store.set_filter(NoteFilter::Tag(tag_id.clone())),
                                          "#{tag.name}"
                                      }
                                  }
                              }
                          }
                      }
                  }
              }
          }
      }
  }
}
