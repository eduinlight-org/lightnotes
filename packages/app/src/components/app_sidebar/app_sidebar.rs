use super::use_app_sidebar::use_app_sidebar;
use crate::state::{FolderIcon, NoteFilter, SyncStatus};
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_icons::lucide::{
  Archive, Bookmark, BookOpen, Briefcase, Calendar, Camera, CirclePlus, CloudCheck, CloudOff,
  Code, Gift, Globe, Heart, House, Inbox, Layers, Lock, Music, Notebook, Palette, Pin, Rocket,
  Settings, SlidersHorizontal, Star, User,
};
use ui::components::button::{Button, ButtonVariant};
use ui::components::sidebar::{
  Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel,
};

fn folder_icon(icon: FolderIcon) -> Element {
  match icon {
    FolderIcon::Inbox => rsx! { Inbox { size: "16px" } },
    FolderIcon::Briefcase => rsx! { Briefcase { size: "16px" } },
    FolderIcon::User => rsx! { User { size: "16px" } },
    FolderIcon::BookOpen => rsx! { BookOpen { size: "16px" } },
    FolderIcon::Notebook => rsx! { Notebook { size: "16px" } },
    FolderIcon::Archive => rsx! { Archive { size: "16px" } },
    FolderIcon::House => rsx! { House { size: "16px" } },
    FolderIcon::Star => rsx! { Star { size: "16px" } },
    FolderIcon::Heart => rsx! { Heart { size: "16px" } },
    FolderIcon::Settings => rsx! { Settings { size: "16px" } },
    FolderIcon::Calendar => rsx! { Calendar { size: "16px" } },
    FolderIcon::Camera => rsx! { Camera { size: "16px" } },
    FolderIcon::Music => rsx! { Music { size: "16px" } },
    FolderIcon::Code => rsx! { Code { size: "16px" } },
    FolderIcon::Palette => rsx! { Palette { size: "16px" } },
    FolderIcon::Gift => rsx! { Gift { size: "16px" } },
    FolderIcon::Globe => rsx! { Globe { size: "16px" } },
    FolderIcon::Lock => rsx! { Lock { size: "16px" } },
    FolderIcon::Rocket => rsx! { Rocket { size: "16px" } },
    FolderIcon::Bookmark => rsx! { Bookmark { size: "16px" } },
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
  let mut sidebar = use_app_sidebar();
  let is_mobile = sidebar.is_mobile;
  let filter = sidebar.store.filter();
  let all_active = filter == NoteFilter::All;
  let starred_active = filter == NoteFilter::Starred;
  let pinned_active = filter == NoteFilter::Pinned;
  let sync = sidebar.store.sync();

  rsx! {
      Sidebar {
          label: t!("sidebar-label"),
          description: t!("sidebar-description"),
          SidebarContent { class: "gap-0 p-2",
              Button {
                  variant: ButtonVariant::Primary,
                  class: "w-full justify-start gap-2 border border-[var(--accent)] bg-transparent text-[var(--accent)] hover:bg-[color-mix(in_srgb,var(--accent)_12%,transparent)]",
                  onclick: move |_| sidebar.create_note(),
                  CirclePlus { size: "16px" }
                  {t!("action-new-note")}
              }
              div { class: "mt-3 flex flex-col gap-1",
                  div {
                      class: nav_row_class(all_active),
                      onclick: move |_| sidebar.select_filter(NoteFilter::All),
                      Layers { size: "16px" }
                      span { class: "flex-1", {t!("filter-all-notes")} }
                      span { class: "text-xs opacity-60", "{sidebar.store.note_count()}" }
                  }
                  div {
                      class: nav_row_class(starred_active),
                      onclick: move |_| sidebar.select_filter(NoteFilter::Starred),
                      Star { size: "16px" }
                      span { class: "flex-1", {t!("filter-starred")} }
                      span { class: "text-xs opacity-60", "{sidebar.store.starred_count()}" }
                  }
                  div {
                      class: nav_row_class(pinned_active),
                      onclick: move |_| sidebar.select_filter(NoteFilter::Pinned),
                      Pin { size: "16px" }
                      span { class: "flex-1", {t!("filter-pinned")} }
                      span { class: "text-xs opacity-60", "{sidebar.store.pinned_count()}" }
                  }
              }
              SidebarGroup { class: "px-0",
                  div { class: "flex items-center justify-between px-2",
                      SidebarGroupLabel { {t!("folders-title")} }
                      button {
                          class: if is_mobile() { "flex items-center gap-1 text-xs text-[var(--accent)]" } else { "flex items-center text-[var(--accent)]" },
                          "aria-label": t!("folders-manage-title"),
                          title: t!("folders-manage-title"),
                          onclick: move |_| sidebar.open_folders_manager(),
                          SlidersHorizontal { size: "13px" }
                          if is_mobile() {
                              {t!("action-manage")}
                          }
                      }
                  }
                  div { class: "flex flex-col gap-1",
                      for folder in sidebar.store.folders() {
                          {
                              let folder_id = folder.id.clone();
                              let is_active = filter == NoteFilter::Folder(folder.id.clone());
                              let count = sidebar.store.folder_note_count(&folder.id);
                              rsx! {
                                  div {
                                      key: "{folder.id}",
                                      class: nav_row_class(is_active),
                                      onclick: move |_| sidebar.select_filter(NoteFilter::Folder(folder_id.clone())),
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
                      SidebarGroupLabel { {t!("tags-title")} }
                      button {
                          class: if is_mobile() { "flex items-center gap-1 text-xs text-[var(--accent)]" } else { "flex items-center text-[var(--accent)]" },
                          "aria-label": t!("tags-manage-title"),
                          title: t!("tags-manage-title"),
                          onclick: move |_| sidebar.open_tags_manager(),
                          SlidersHorizontal { size: "13px" }
                          if is_mobile() {
                              {t!("action-manage")}
                          }
                      }
                  }
                  div { class: "flex flex-wrap gap-1.5 px-2",
                      for tag in sidebar.store.tags() {
                          {
                              let tag_id = tag.id.clone();
                              let is_active = filter == NoteFilter::Tag(tag.id.clone());
                              let count = sidebar.store.tag_note_count(&tag.id);
                              let class = if is_active {
                                  "inline-flex items-center gap-1 rounded-md border border-[var(--accent)] px-2 py-1 text-xs cursor-pointer text-[var(--accent)]"
                              } else {
                                  "inline-flex items-center gap-1 rounded-md border border-[var(--primary-color-6)] px-2 py-1 text-xs cursor-pointer text-[var(--secondary-color)]"
                              };
                              rsx! {
                                  span {
                                      key: "{tag.id}",
                                      class,
                                      onclick: move |_| sidebar.select_filter(NoteFilter::Tag(tag_id.clone())),
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
                  title: t!("sync-toggle-hint"),
                  onclick: move |_| sidebar.toggle_sync(),
                  if sync == SyncStatus::Synced {
                      CloudCheck { size: "15px" }
                      {t!("sync-saved")}
                  } else {
                      CloudOff { size: "15px" }
                      {t!("sync-offline")}
                  }
              }
          }
      }
  }
}
