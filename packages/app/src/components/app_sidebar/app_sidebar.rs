use crate::state::{use_notes, use_ui, FolderIcon, NoteFilter, SyncStatus};
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::{
  Archive, BookOpen, Briefcase, CirclePlus, CloudCheck, CloudOff, Inbox, Layers, Notebook, Pin,
  SlidersHorizontal, Star, User,
};
use ui::components::button::{Button, ButtonVariant};
use ui::components::sidebar::{
  use_is_mobile, use_sidebar, Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel,
};

fn folder_icon(icon: FolderIcon) -> Element {
  match icon {
    FolderIcon::Inbox => rsx! { Inbox { size: "16px" } },
    FolderIcon::Briefcase => rsx! { Briefcase { size: "16px" } },
    FolderIcon::User => rsx! { User { size: "16px" } },
    FolderIcon::BookOpen => rsx! { BookOpen { size: "16px" } },
    FolderIcon::Notebook => rsx! { Notebook { size: "16px" } },
    FolderIcon::Archive => rsx! { Archive { size: "16px" } },
  }
}

fn nav_row_class(active: bool) -> &'static str {
  if active {
    "flex items-center gap-2 rounded-lg px-2 py-1.5 text-sm cursor-pointer bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] text-[var(--accent)]"
  } else {
    "flex items-center gap-2 rounded-lg px-2 py-1.5 text-sm cursor-pointer text-[var(--secondary-color)] hover:bg-[var(--primary-color-4)]"
  }
}

#[component]
pub fn AppSidebar() -> Element {
  let mut store = use_notes();
  let mut ui = use_ui();
  let sidebar_ctx = use_sidebar();
  let is_mobile = use_is_mobile();
  let filter = store.filter();
  let all_active = filter == NoteFilter::All;
  let starred_active = filter == NoteFilter::Starred;
  let pinned_active = filter == NoteFilter::Pinned;
  let sync = store.sync();

  let select_filter = move |filter: NoteFilter| {
    let mut store = store;
    store.set_filter(filter);
    sidebar_ctx.set_open_mobile(false);
    navigator().push(Route::Notes {});
  };

  rsx! {
      Sidebar {
          SidebarContent { class: "gap-0 p-2",
              Button {
                  variant: ButtonVariant::Primary,
                  class: "w-full justify-start gap-2 border border-[var(--accent)] bg-transparent text-[var(--accent)] hover:bg-[color-mix(in_srgb,var(--accent)_12%,transparent)]",
                  onclick: move |_| {
                      let note_id = store.create_note();
                      sidebar_ctx.set_open_mobile(false);
                      navigator().push(Route::NoteEditor { note_id });
                  },
                  CirclePlus { size: "16px" }
                  "New note"
              }
              div { class: "mt-3 flex flex-col gap-1",
                  div {
                      class: nav_row_class(all_active),
                      onclick: move |_| select_filter(NoteFilter::All),
                      Layers { size: "16px" }
                      span { class: "flex-1", "All Notes" }
                      span { class: "text-xs opacity-60", "{store.note_count()}" }
                  }
                  div {
                      class: nav_row_class(starred_active),
                      onclick: move |_| select_filter(NoteFilter::Starred),
                      Star { size: "16px" }
                      span { class: "flex-1", "Starred" }
                      span { class: "text-xs opacity-60", "{store.starred_count()}" }
                  }
                  div {
                      class: nav_row_class(pinned_active),
                      onclick: move |_| select_filter(NoteFilter::Pinned),
                      Pin { size: "16px" }
                      span { class: "flex-1", "Pinned" }
                      span { class: "text-xs opacity-60", "{store.pinned_count()}" }
                  }
              }
              SidebarGroup { class: "px-0",
                  div { class: "flex items-center justify-between px-2",
                      SidebarGroupLabel { "Folders" }
                      button {
                          class: if is_mobile() { "flex items-center gap-1 text-xs text-[var(--accent)]" } else { "flex items-center text-[var(--accent)]" },
                          "aria-label": "Manage folders",
                          title: "Manage folders",
                          onclick: move |_| {
                              sidebar_ctx.set_open_mobile(false);
                              if is_mobile() {
                                  navigator().push(Route::FoldersScreen {});
                              } else {
                                  ui.folders_open.set(true);
                              }
                          },
                          SlidersHorizontal { size: "13px" }
                          if is_mobile() {
                              "Manage"
                          }
                      }
                  }
                  div { class: "flex flex-col gap-1",
                      for folder in store.folders() {
                          {
                              let folder_id = folder.id.clone();
                              let is_active = filter == NoteFilter::Folder(folder.id.clone());
                              let count = store.folder_note_count(&folder.id);
                              rsx! {
                                  div {
                                      key: "{folder.id}",
                                      class: nav_row_class(is_active),
                                      onclick: move |_| select_filter(NoteFilter::Folder(folder_id.clone())),
                                      {folder_icon(folder.icon)}
                                      span { class: "flex-1 truncate", "{folder.name}" }
                                      span { class: "text-xs opacity-60", "{count}" }
                                  }
                              }
                          }
                      }
                  }
              }
              SidebarGroup { class: "px-0",
                  div { class: "flex items-center justify-between px-2",
                      SidebarGroupLabel { "Tags" }
                      button {
                          class: if is_mobile() { "flex items-center gap-1 text-xs text-[var(--accent)]" } else { "flex items-center text-[var(--accent)]" },
                          "aria-label": "Manage tags",
                          title: "Manage tags",
                          onclick: move |_| {
                              sidebar_ctx.set_open_mobile(false);
                              if is_mobile() {
                                  navigator().push(Route::TagsScreen {});
                              } else {
                                  ui.tags_open.set(true);
                              }
                          },
                          SlidersHorizontal { size: "13px" }
                          if is_mobile() {
                              "Manage"
                          }
                      }
                  }
                  div { class: "flex flex-wrap gap-1.5 px-2",
                      for tag in store.tags() {
                          {
                              let tag_id = tag.id.clone();
                              let is_active = filter == NoteFilter::Tag(tag.id.clone());
                              let count = store.tag_note_count(&tag.id);
                              let class = if is_active {
                                  "inline-flex items-center gap-1 rounded-md border border-[var(--accent)] px-2 py-1 text-xs cursor-pointer text-[var(--accent)]"
                              } else {
                                  "inline-flex items-center gap-1 rounded-md border border-[var(--primary-color-6)] px-2 py-1 text-xs cursor-pointer text-[var(--secondary-color)]"
                              };
                              rsx! {
                                  span {
                                      key: "{tag.id}",
                                      class,
                                      onclick: move |_| select_filter(NoteFilter::Tag(tag_id.clone())),
                                      "#{tag.name}"
                                      span { class: "opacity-55", "{count}" }
                                  }
                              }
                          }
                      }
                  }
              }
          }
          SidebarFooter { class: "gap-1 border-t border-[var(--primary-color-6)] pt-2",
              div {
                  class: "flex cursor-pointer items-center gap-2 rounded-lg px-2 py-1.5 text-xs text-[var(--secondary-color-5)] hover:bg-[var(--primary-color-4)]",
                  title: "Click to toggle offline",
                  onclick: move |_| store.toggle_sync(),
                  if sync == SyncStatus::Synced {
                      CloudCheck { size: "15px" }
                      "All changes saved"
                  } else {
                      CloudOff { size: "15px" }
                      "Offline — saved locally"
                  }
              }
          }
      }
  }
}
