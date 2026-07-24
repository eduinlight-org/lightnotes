use super::use_note_editor::{use_note_editor, EditorView};
use crate::state::Note;
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::{
  ArrowLeft, Bold, Columns2, Eye, Heading1, Italic, List, Pencil, Trash2,
};
use pulldown_cmark::{html, Parser};
use ui::components::badge::{Badge, BadgeVariant};
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::input::Input;
use ui::components::textarea::Textarea;

// Tailwind's typography plugin isn't wired into this workspace's CSS-only
// (no npm) build, so the markdown preview is styled directly via descendant
// selectors instead of the `prose` classes.
const MARKDOWN_PREVIEW_CLASS: &str = "text-[#e5e5e5] [&_h1]:mb-3 [&_h1]:mt-4 [&_h1]:text-2xl [&_h1]:font-bold [&_h1]:text-white [&_h2]:mb-2 [&_h2]:mt-4 [&_h2]:text-xl [&_h2]:font-bold [&_h2]:text-white [&_h3]:mb-2 [&_h3]:mt-3 [&_h3]:text-lg [&_h3]:font-bold [&_h3]:text-white [&_p]:mb-3 [&_ul]:mb-3 [&_ul]:list-disc [&_ul]:pl-6 [&_ol]:mb-3 [&_ol]:list-decimal [&_ol]:pl-6 [&_li]:mb-1 [&_strong]:font-semibold [&_strong]:text-white [&_em]:italic [&_code]:rounded [&_code]:bg-white/10 [&_code]:px-1 [&_code]:py-0.5 [&_a]:text-amber-400 [&_a]:underline";

fn render_markdown(markdown: &str) -> String {
  let parser = Parser::new(markdown);
  let mut html_output = String::new();
  html::push_html(&mut html_output, parser);
  html_output
}

#[component]
pub fn NoteEditorPanel(note: Note) -> Element {
  let mut editor = use_note_editor();
  let mut store = editor.store;
  let note_id = note.id.clone();
  let view = (editor.view)();
  let preview_pane_class = format!(
    "{MARKDOWN_PREVIEW_CLASS} {}",
    if view == EditorView::Split {
      "h-full w-1/2 overflow-y-auto border-l border-white/10 p-4"
    } else {
      "h-full w-full overflow-y-auto p-4"
    }
  );

  rsx! {
      div { class: "flex h-full flex-1 flex-col",
          div { class: "flex items-center gap-2 border-b border-white/10 p-3",
              Button {
                  class: "md:hidden",
                  variant: ButtonVariant::Ghost,
                  size: ButtonSize::IconSm,
                  "aria-label": "Back to notes",
                  onclick: move |_| {
                      navigator().push(Route::Notes {});
                  },
                  ArrowLeft { size: "16px" }
              }
              Input {
                  class: "flex-1 text-lg font-semibold",
                  placeholder: "Untitled note",
                  value: note.title.clone(),
                  oninput: {
                      let note_id = note_id.clone();
                      move |event: FormEvent| store.set_note_title(&note_id, event.value())
                  },
              }
              Button {
                  variant: ButtonVariant::Destructive,
                  size: ButtonSize::IconSm,
                  "aria-label": "Delete note",
                  onclick: {
                      let note_id = note_id.clone();
                      move |_| {
                          store.delete_note(&note_id);
                          navigator().push(Route::Notes {});
                      }
                  },
                  Trash2 { size: "16px" }
              }
          }
          div { class: "flex items-center gap-1 border-b border-white/10 p-2",
              Button {
                  variant: ButtonVariant::Ghost,
                  size: ButtonSize::IconXs,
                  "aria-label": "Bold",
                  onclick: {
                      let note_id = note_id.clone();
                      let content = note.content.clone();
                      move |_| editor.append_snippet(&note_id, &content, "**bold** ")
                  },
                  Bold { size: "14px" }
              }
              Button {
                  variant: ButtonVariant::Ghost,
                  size: ButtonSize::IconXs,
                  "aria-label": "Italic",
                  onclick: {
                      let note_id = note_id.clone();
                      let content = note.content.clone();
                      move |_| editor.append_snippet(&note_id, &content, "_italic_ ")
                  },
                  Italic { size: "14px" }
              }
              Button {
                  variant: ButtonVariant::Ghost,
                  size: ButtonSize::IconXs,
                  "aria-label": "Heading",
                  onclick: {
                      let note_id = note_id.clone();
                      let content = note.content.clone();
                      move |_| editor.append_snippet(&note_id, &content, "\n# Heading\n")
                  },
                  Heading1 { size: "14px" }
              }
              Button {
                  variant: ButtonVariant::Ghost,
                  size: ButtonSize::IconXs,
                  "aria-label": "Bulleted list",
                  onclick: {
                      let note_id = note_id.clone();
                      let content = note.content.clone();
                      move |_| editor.append_snippet(&note_id, &content, "\n- item\n")
                  },
                  List { size: "14px" }
              }
              div { class: "ml-auto flex items-center gap-1",
                  Button {
                      variant: if view == EditorView::Edit { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
                      size: ButtonSize::IconXs,
                      "aria-label": "Edit",
                      onclick: move |_| editor.view.set(EditorView::Edit),
                      Pencil { size: "14px" }
                  }
                  Button {
                      variant: if view == EditorView::Preview { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
                      size: ButtonSize::IconXs,
                      "aria-label": "Preview",
                      onclick: move |_| editor.view.set(EditorView::Preview),
                      Eye { size: "14px" }
                  }
                  Button {
                      variant: if view == EditorView::Split { ButtonVariant::Secondary } else { ButtonVariant::Ghost },
                      size: ButtonSize::IconXs,
                      "aria-label": "Split view",
                      onclick: move |_| editor.view.set(EditorView::Split),
                      Columns2 { size: "14px" }
                  }
              }
          }
          div { class: "flex flex-1 overflow-hidden",
              if matches!(view, EditorView::Edit | EditorView::Split) {
                  Textarea {
                      class: if view == EditorView::Split { "h-full w-1/2 resize-none border-r border-white/10 p-4" } else { "h-full w-full resize-none p-4" },
                      placeholder: "Write in Markdown...",
                      value: note.content.clone(),
                      oninput: {
                          let note_id = note_id.clone();
                          move |event: FormEvent| store.set_note_content(&note_id, event.value())
                      },
                  }
              }
              if matches!(view, EditorView::Preview | EditorView::Split) {
                  div {
                      class: preview_pane_class,
                      dangerous_inner_html: "{render_markdown(&note.content)}",
                  }
              }
          }
          div { class: "flex flex-wrap items-center gap-2 border-t border-white/10 p-3",
              for tag_id in note.tag_ids.iter() {
                  if let Some(name) = store.tag_name(tag_id) {
                      Badge { key: "{tag_id}", variant: BadgeVariant::Outline,
                          "#{name}"
                          button {
                              class: "ml-1",
                              onclick: {
                                  let note_id = note_id.clone();
                                  let tag_id = tag_id.clone();
                                  move |_| store.remove_note_tag(&note_id, &tag_id)
                              },
                              "x"
                          }
                      }
                  }
              }
              Input {
                  class: "w-32",
                  placeholder: "Add tag...",
                  value: (editor.tag_draft)(),
                  oninput: move |event: FormEvent| editor.tag_draft.set(event.value()),
                  onkeydown: {
                      let note_id = note_id.clone();
                      move |event: KeyboardEvent| {
                          if event.key() == Key::Enter {
                              editor.commit_tag_draft(&note_id);
                          }
                      }
                  },
              }
          }
      }
  }
}
