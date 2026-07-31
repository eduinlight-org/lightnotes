use super::link_dialog::LinkDialog;
use super::use_note_editor::use_note_editor;
use crate::components::{
  ConfirmDialog, FolderPicker, ResponsivePopoverContent, ResponsivePopoverRoot,
  ResponsivePopoverTrigger,
};
use crate::state::{format_relative_time, Note};
use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_icons::lucide::{
  ArrowLeft, Bold, CaseLower, CaseUpper, Code, FileText, Heading1, Heading2, Heading3, Italic,
  Link as LinkIcon, List, ListIndentDecrease, ListOrdered, Merge, Pilcrow, Pin, Quote, Redo,
  SquareDashedMousePointer, Star, Table as TableIcon, TableCellsMerge, TableCellsSplit,
  TextAlignCenter, TextAlignEnd, TextAlignJustify, TextAlignStart, Trash2, Undo, Unlink, X,
};
use editor::MarkdownEditorView;
use std::cell::RefCell;
use std::rc::Rc;
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::input::Input;
use ui::components::popover::{ContentAlign, ContentSide};
use ui::components::sidebar::use_is_mobile;
use ui::components::tooltip::{Tooltip, TooltipContent, TooltipTrigger};

const TOOLBAR_BUTTON_CLASS: &str = "flex flex-justify items-center text-[var(--accent)] hover:bg-[color-mix(in_srgb,var(--accent)_10%,transparent)] hover:text-[var(--accent)]";

const EDITOR_CONTENT_CLASS: &str ="text-[var(--secondary-color)] [&_h1]:mb-3 [&_h1]:mt-4 [&_h1]:text-2xl [&_h1]:font-medium [&_h2]:mb-2 [&_h2]:mt-4 [&_h2]:text-xl [&_h2]:font-medium [&_h3]:mb-2 [&_h3]:mt-3 [&_h3]:text-lg [&_h3]:font-medium [&_p]:mb-3 [&_ul]:mb-3 [&_ul]:list-disc [&_ul]:pl-6 [&_ol]:mb-3 [&_ol]:list-decimal [&_ol]:pl-6 [&_li]:mb-1 [&_strong]:font-semibold [&_em]:italic [&_code]:rounded [&_code]:bg-[var(--primary-color-5)] [&_code]:px-1 [&_code]:py-0.5 [&_code]:text-[13px] [&_pre]:mb-3 [&_pre]:overflow-x-auto [&_pre]:rounded-lg [&_pre]:bg-[var(--primary-color-5)] [&_pre]:p-3 [&_pre_code]:bg-transparent [&_pre_code]:p-0 [&_a]:text-[var(--accent)] [&_a]:underline [&_blockquote]:mb-3 [&_blockquote]:border-l-2 [&_blockquote]:border-[var(--accent)] [&_blockquote]:pl-3 [&_blockquote]:italic [&_blockquote]:text-[var(--secondary-color-5)] [&_table]:mb-3 [&_table]:w-full [&_table]:border-collapse [&_th]:border [&_th]:border-[var(--primary-color-6)] [&_th]:p-2 [&_th]:text-left [&_th]:font-medium [&_td]:border [&_td]:border-[var(--primary-color-6)] [&_td]:p-2 [&_img]:max-w-full [&_img]:rounded-lg [&_.taino-cell-selected]:bg-[color-mix(in_srgb,var(--accent)_18%,transparent)] [&_.taino-editor]:h-full [&_.taino-editor]:min-h-full [&_.taino-editor]:outline-none";

fn word_count(content: &str) -> usize {
  content.split_whitespace().count()
}

const TABLE_BUTTON_CLASS: &str = "rounded-md border border-[var(--primary-color-6)] px-2 py-1 text-xs text-[var(--secondary-color)] hover:bg-[color-mix(in_srgb,var(--secondary-color)_6%,transparent)]";

fn toolbar_button(
  label: String,
  tooltip: String,
  icon: Element,
  onclick: impl FnMut() + 'static,
) -> Element {
  let onclick = Rc::new(RefCell::new(onclick));
  rsx! {
    div { class: "shrink-0",
      Tooltip {
        TooltipTrigger {
          as: move |trigger_attrs: Vec<Attribute>| {
            let icon = icon.clone();
            let onclick = onclick.clone();
            rsx! {
              Button {
                variant: ButtonVariant::Ghost,
                size: ButtonSize::IconXs,
                class: TOOLBAR_BUTTON_CLASS,
                "aria-label": label.clone(),
                attributes: trigger_attrs,
                onmousedown: move |event: MouseEvent| event.prevent_default(),
                onclick: move |_| (onclick.borrow_mut())(),
                {icon}
              }
            }
          },
        }
        TooltipContent { side: ContentSide::Bottom, {tooltip} }
      }
    }
  }
}

fn table_button(
  label: String,
  tooltip: String,
  onclick: impl FnMut() + 'static,
) -> Element {
  let onclick = Rc::new(RefCell::new(onclick));
  rsx! {
    div { class: "shrink-0",
      Tooltip {
        TooltipTrigger {
          as: move |trigger_attrs: Vec<Attribute>| {
            let onclick = onclick.clone();
            rsx! {
              button {
                class: TABLE_BUTTON_CLASS,
                onmousedown: move |event: MouseEvent| event.prevent_default(),
                onclick: move |_| (onclick.borrow_mut())(),
                ..trigger_attrs,
                {label.clone()}
              }
            }
          },
        }
        TooltipContent { side: ContentSide::Bottom, {tooltip} }
      }
    }
  }
}

#[derive(PartialEq, Clone, Props)]
pub struct NoteEditorPanelProps {
  pub note: Note,
  #[props(default)]
  pub extra_header: Option<Element>,
  #[props(default = Route::Notes {})]
  pub back_route: Route,
}

#[component]
pub fn NoteEditorPanel(props: NoteEditorPanelProps) -> Element {
  let NoteEditorPanelProps { note, extra_header, back_route } = props;
  let mut editor = use_note_editor();
  let mut store = editor.store;
  let is_mobile = use_is_mobile();
  let note_id = note.id.clone();

  editor.sync_note(&note);

  {
    let mut editor = editor;
    let note_id = note_id.clone();
    use_effect(move || editor.save_if_changed(&note_id));
  }

  let words = word_count(&note.content);
  let addable_tags: Vec<_> = store
    .tags()
    .into_iter()
    .filter(|tag| !note.tag_ids.contains(&tag.id))
    .collect();
  let display_title = if note.title.is_empty() { t!("notes-untitled-note") } else { note.title.clone() };

  rsx! {
      div { class: "flex h-full flex-1 flex-col overflow-hidden",
          div { class: "flex flex-none flex-col border-b border-[var(--primary-color-6)] p-3 md:px-6 md:py-4",
          div { class: "flex items-center gap-2",
              Button {
                  class: "md:hidden",
                  variant: ButtonVariant::Ghost,
                  size: ButtonSize::IconSm,
                  "aria-label": t!("note-back-to-notes"),
                  onclick: {
                      let back_route = back_route.clone();
                      move |_| {
                          navigator().push(back_route.clone());
                      }
                  },
                  ArrowLeft { size: "16px" }
              }
              Input {
                  class: "h-auto min-w-0 flex-1 border-none bg-transparent px-0 py-1 text-xl font-medium shadow-none [outline:none] hover:bg-transparent focus:bg-transparent focus:shadow-none md:text-[28px]",
                  placeholder: t!("notes-untitled-note"),
                  value: note.title.clone(),
                  oninput: {
                      let note_id = note_id.clone();
                      move |event: FormEvent| store.set_note_title(&note_id, event.value())
                  },
              }
              Button {
                  variant: ButtonVariant::Ghost,
                  size: ButtonSize::IconSm,
                  "aria-label": if note.starred { t!("note-remove-from-starred") } else { t!("note-add-to-starred") },
                  onclick: {
                      let note_id = note_id.clone();
                      move |_| store.toggle_note_star(&note_id)
                  },
                  Star {
                      size: "18px",
                      fill: if note.starred { "#d9b84b" } else { "none" },
                      stroke: if note.starred { "#d9b84b" } else { "var(--secondary-color-5)" },
                  }
              }
              Button {
                  variant: ButtonVariant::Ghost,
                  size: ButtonSize::IconSm,
                  "aria-label": if note.pinned { t!("note-unpin-from-top") } else { t!("note-pin-to-top") },
                  onclick: {
                      let note_id = note_id.clone();
                      move |_| store.toggle_note_pin(&note_id)
                  },
                  Pin {
                      size: "18px",
                      fill: if note.pinned { "var(--accent)" } else { "none" },
                      stroke: if note.pinned { "var(--accent)" } else { "var(--secondary-color-5)" },
                  }
              }
              Button {
                  variant: ButtonVariant::Ghost,
                  size: ButtonSize::IconSm,
                  "aria-label": t!("note-delete"),
                  onclick: move |_| editor.confirm_delete_open.set(true),
                  Trash2 { size: "16px", stroke: "var(--secondary-color-5)" }
              }
          }
          div { class: "mt-2 flex flex-wrap items-center gap-2 text-xs text-[var(--secondary-color-5)]",
              FolderPicker { note: note.clone() }
              span { "\u{b7}" }
              span { {t!("note-edited", time: format_relative_time(note.updated_at_ms))} }
              span { "\u{b7}" }
              span { {t!("note-word-count", count: words as i64)} }
              span { class: "mx-1 h-3 w-px bg-[var(--primary-color-6)]" }
              for tag_id in note.tag_ids.iter() {
                  if let Some(name) = store.tag_name(tag_id) {
                      span {
                          key: "{tag_id}",
                          class: if is_mobile() {
                              "inline-flex items-center gap-1.5 rounded-full border border-[var(--primary-color-6)] px-3 py-1 text-[var(--secondary-color)]"
                          } else {
                              "inline-flex items-center gap-1 rounded-md border border-[var(--primary-color-6)] px-2 py-0.5 text-[var(--secondary-color)]"
                          },
                          "#{name}"
                          button {
                              "aria-label": t!("note-remove-tag"),
                              onclick: {
                                  let note_id = note_id.clone();
                                  let tag_id = tag_id.clone();
                                  move |_| store.remove_note_tag(&note_id, &tag_id)
                              },
                              X { size: if is_mobile() { "14px" } else { "11px" } }
                          }
                      }
                  }
              }
              ResponsivePopoverRoot {
                  open: (editor.tag_picker_open)(),
                  on_open_change: move |value| editor.tag_picker_open.set(value),
                  ResponsivePopoverTrigger {
                      class: "inline-flex items-center gap-[4px] rounded-md border-none bg-transparent px-[6px] py-[2px] text-[var(--accent)] hover:bg-[color-mix(in_srgb,var(--accent)_10%,transparent)]",
                      {t!("note-add-tag")}
                  }
                  ResponsivePopoverContent {
                      title: t!("note-add-tag-title"),
                      align: ContentAlign::Start,
                      class: "w-56 items-stretch gap-2 p-2 text-left",
                      div {
                          class: if is_mobile() {
                              "flex h-11 items-center gap-1.5 rounded-[11px] border border-[var(--primary-color-6)] bg-[var(--primary-color-2)] px-[13px] focus-within:border-[var(--accent)]"
                          } else {
                              "flex h-8 items-center gap-1.5 rounded-lg border border-[var(--primary-color-6)] bg-[var(--primary-color-1)] px-[9px] focus-within:border-[var(--accent)]"
                          },
                          span { class: "flex-none text-[var(--secondary-color-5)]", "#" }
                          Input {
                              class: "h-full flex-1 border-none bg-transparent p-0 text-[13px] shadow-none [outline:none] hover:bg-transparent focus:bg-transparent focus:shadow-none",
                              placeholder: t!("note-new-tag-placeholder"),
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
                      div { class: "flex flex-wrap gap-1.5",
                          for tag in addable_tags {
                              {
                                  let tag_id = tag.id.clone();
                                  let note_id = note_id.clone();
                                  let chip_class = if is_mobile() {
                                      "cursor-pointer rounded-full border border-[var(--primary-color-6)] px-3 py-1.5 text-sm text-[var(--secondary-color)]"
                                  } else {
                                      "cursor-pointer rounded-md border border-[var(--primary-color-6)] px-2 py-1 text-xs text-[var(--secondary-color)]"
                                  };
                                  rsx! {
                                      span {
                                          key: "{tag.id}",
                                          class: chip_class,
                                          onclick: move |_| store.add_note_tag(&note_id, tag_id.clone()),
                                          "#{tag.name}"
                                      }
                                  }
                              }
                          }
                      }
                  }
              }
          }
          if let Some(extra_header) = extra_header {
              div { class: "mt-2 flex flex-wrap items-center gap-2", {extra_header} }
          }
          div { class: "mt-3 flex flex-nowrap items-center gap-1 overflow-x-auto overflow-y-hidden [scrollbar-width:none] [&::-webkit-scrollbar]:hidden md:flex-wrap md:overflow-visible",
              {toolbar_button(t!("editor-bold"), t!("editor-bold"), rsx! { Bold { size: "14px" } }, move || editor.markdown_editor.toggle_bold())}
              {toolbar_button(t!("editor-italic"), t!("editor-italic"), rsx! { Italic { size: "14px" } }, move || editor.markdown_editor.toggle_italic())}
              {toolbar_button(t!("editor-code"), t!("editor-inline-code"), rsx! { Code { size: "14px" } }, move || editor.markdown_editor.toggle_code())}
              {toolbar_button(t!("editor-link"), t!("editor-add-link-prompt"), rsx! { LinkIcon { size: "14px" } }, move || editor.open_link_dialog())}
              {toolbar_button(t!("editor-remove-link"), t!("editor-remove-link"), rsx! { Unlink { size: "14px" } }, move || editor.markdown_editor.remove_link())}
              span { class: "mx-1 h-4 w-px shrink-0 bg-[var(--primary-color-6)]" }
              {toolbar_button(t!("editor-paragraph"), t!("editor-paragraph-tooltip"), rsx! { Pilcrow { size: "14px" } }, move || editor.markdown_editor.set_paragraph())}
              {toolbar_button(t!("editor-heading-1"), t!("editor-heading-1"), rsx! { Heading1 { size: "14px" } }, move || editor.markdown_editor.set_heading(1))}
              {toolbar_button(t!("editor-heading-2"), t!("editor-heading-2"), rsx! { Heading2 { size: "14px" } }, move || editor.markdown_editor.set_heading(2))}
              {toolbar_button(t!("editor-heading-3"), t!("editor-heading-3"), rsx! { Heading3 { size: "14px" } }, move || editor.markdown_editor.set_heading(3))}
              {toolbar_button(t!("editor-quote"), t!("editor-quote-tooltip"), rsx! { Quote { size: "14px" } }, move || editor.markdown_editor.toggle_blockquote())}
              {toolbar_button(t!("editor-code-block"), t!("editor-code-block"), rsx! { Code { size: "14px" } }, move || editor.markdown_editor.set_code_block())}
              span { class: "mx-1 h-4 w-px shrink-0 bg-[var(--primary-color-6)]" }
              {toolbar_button(t!("editor-bulleted-list"), t!("editor-bulleted-list"), rsx! { List { size: "14px" } }, move || editor.markdown_editor.toggle_bullet_list())}
              {toolbar_button(t!("editor-numbered-list"), t!("editor-numbered-list"), rsx! { ListOrdered { size: "14px" } }, move || editor.markdown_editor.toggle_ordered_list())}
              {toolbar_button(t!("editor-lift-list"), t!("editor-lift-list"), rsx! { ListIndentDecrease { size: "14px" } }, move || editor.markdown_editor.lift_list_item())}
              span { class: "mx-1 h-4 w-px shrink-0 bg-[var(--primary-color-6)]" }
              {toolbar_button(t!("editor-align-left"), t!("editor-align-left"), rsx! { TextAlignStart { size: "14px" } }, move || editor.markdown_editor.align_left())}
              {toolbar_button(t!("editor-align-center"), t!("editor-align-center"), rsx! { TextAlignCenter { size: "14px" } }, move || editor.markdown_editor.align_center())}
              {toolbar_button(t!("editor-align-right"), t!("editor-align-right"), rsx! { TextAlignEnd { size: "14px" } }, move || editor.markdown_editor.align_right())}
              {toolbar_button(t!("editor-justify"), t!("editor-justify-tooltip"), rsx! { TextAlignJustify { size: "14px" } }, move || editor.markdown_editor.align_justify())}
              span { class: "mx-1 h-4 w-px shrink-0 bg-[var(--primary-color-6)]" }
              {toolbar_button(t!("editor-uppercase"), t!("editor-uppercase-tooltip"), rsx! { CaseUpper { size: "14px" } }, move || editor.markdown_editor.to_uppercase())}
              {toolbar_button(t!("editor-lowercase"), t!("editor-lowercase-tooltip"), rsx! { CaseLower { size: "14px" } }, move || editor.markdown_editor.to_lowercase())}
              span { class: "mx-1 h-4 w-px shrink-0 bg-[var(--primary-color-6)]" }
              {toolbar_button(t!("editor-undo"), t!("editor-undo"), rsx! { Undo { size: "14px" } }, move || editor.markdown_editor.undo())}
              {toolbar_button(t!("editor-redo"), t!("editor-redo"), rsx! { Redo { size: "14px" } }, move || editor.markdown_editor.redo())}
              {toolbar_button(t!("editor-select-all"), t!("editor-select-all"), rsx! { SquareDashedMousePointer { size: "14px" } }, move || editor.markdown_editor.select_all())}
          }
          div { class: "mt-1.5 flex flex-nowrap items-center gap-1.5 overflow-x-auto overflow-y-hidden [scrollbar-width:none] [&::-webkit-scrollbar]:hidden md:flex-wrap md:overflow-visible",
              span { class: "shrink-0 text-[10px] font-medium uppercase tracking-wider text-[var(--secondary-color-5)]", {t!("editor-table")} }
              {toolbar_button(t!("editor-insert-table"), t!("editor-insert-table-tooltip"), rsx! { TableIcon { size: "14px" } }, move || editor.markdown_editor.insert_table(3, 3))}
              {table_button(t!("editor-add-row"), t!("editor-add-row-tooltip"), move || editor.markdown_editor.add_row())}
              {table_button(t!("editor-add-column"), t!("editor-add-column-tooltip"), move || editor.markdown_editor.add_column())}
              {table_button(t!("editor-delete-row"), t!("editor-delete-row-tooltip"), move || editor.markdown_editor.delete_row())}
              {table_button(t!("editor-delete-column"), t!("editor-delete-column-tooltip"), move || editor.markdown_editor.delete_column())}
              {table_button(t!("editor-header-row"), t!("editor-header-row-tooltip"), move || editor.markdown_editor.toggle_header_row())}
              {toolbar_button(t!("editor-merge-row"), t!("editor-merge-row-tooltip"), rsx! { Merge { size: "14px" } }, move || editor.markdown_editor.merge_row())}
              {toolbar_button(t!("editor-merge-column"), t!("editor-merge-column-tooltip"), rsx! { TableCellsMerge { size: "14px" } }, move || editor.markdown_editor.merge_column())}
              {toolbar_button(t!("editor-split-cell"), t!("editor-split-cell-tooltip"), rsx! { TableCellsSplit { size: "14px" } }, move || editor.markdown_editor.split_cell())}
              {table_button(t!("editor-delete-table"), t!("editor-delete-table-tooltip"), move || editor.markdown_editor.delete_table())}
          }
          }
          div { class: "min-h-0 flex flex-1 overflow-hidden",
              MarkdownEditorView {
                  handle: editor.markdown_editor,
                  class: format!("{EDITOR_CONTENT_CLASS} h-full w-full overflow-y-auto p-4 md:p-6"),
              }
          }
      }
      LinkDialog { editor }
      ConfirmDialog {
          open: (editor.confirm_delete_open)(),
          on_open_change: move |_| editor.confirm_delete_open.set(false),
          icon: rsx! { FileText { size: "20px", stroke: "var(--primary-error-color)" } },
          title: t!("note-delete-title"),
          description: rsx! {
              span {
                  strong { "{display_title}" }
                  " — "
                  {t!("note-delete-description")}
              }
          },
          on_confirm: {
              let note_id = note_id.clone();
              let back_route = back_route.clone();
              move |_| {
                  store.delete_note(&note_id);
                  editor.confirm_delete_open.set(false);
                  navigator().push(back_route.clone());
              }
          },
      }
  }
}
