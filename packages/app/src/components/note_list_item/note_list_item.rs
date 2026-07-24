use crate::state::{use_notes, Note};
use crate::Route;
use dioxus::prelude::*;
use ui::components::badge::{Badge, BadgeVariant};
use ui::components::card::{Card, CardContent, CardHeader, CardTitle};

fn snippet(markdown: &str) -> String {
  let plain: String = markdown
    .lines()
    .map(|line| line.trim_start_matches(['#', '-', '*', ' ', '\t']))
    .collect::<Vec<_>>()
    .join(" ");
  plain.chars().take(90).collect()
}

#[component]
pub fn NoteListItem(note: Note, is_active: bool) -> Element {
  let store = use_notes();
  let note_id = note.id.clone();

  rsx! {
      div {
          class: if is_active {
              "cursor-pointer rounded-lg border border-amber-400/40 bg-white/10"
          } else {
              "cursor-pointer rounded-lg border border-transparent hover:bg-white/5"
          },
          onclick: move |_| {
              navigator().push(Route::NoteEditor { note_id: note_id.clone() });
          },
          Card { class: "border-none bg-transparent shadow-none",
              CardHeader {
                  CardTitle { class: "text-white", "{note.title}" }
              }
              CardContent {
                  p { class: "line-clamp-2 text-sm text-[#a1a1a1]", "{snippet(&note.content)}" }
                  div { class: "mt-2 flex flex-wrap items-center gap-2",
                      span { class: "text-xs text-[#5d5d5d]", "{note.updated_at}" }
                      for tag_id in note.tag_ids.iter() {
                          if let Some(name) = store.tag_name(tag_id) {
                              Badge { key: "{tag_id}", variant: BadgeVariant::Outline, "#{name}" }
                          }
                      }
                  }
              }
          }
      }
  }
}
