use super::link_dialog::LinkDialog;
use super::use_note_editor::use_note_editor;
use crate::components::{
  ResponsivePopoverContent, ResponsivePopoverRoot, ResponsivePopoverTrigger,
};
use crate::state::{format_relative_time, FolderIcon, Note};
use crate::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::{
  Archive, ArrowLeft, Bold, BookOpen, Bookmark, Briefcase, Calendar, Camera, CaseLower, CaseUpper,
  ChevronDown, Code, FileText, Gift, Globe, Heading1, Heading2, Heading3, Heart, House,
  Inbox, Italic, Link as LinkIcon, List, ListIndentDecrease, ListOrdered, Lock,
  Merge, Music, Notebook, Palette, Pilcrow, Pin, Quote, Redo, Rocket, Settings,
  SquareDashedMousePointer, Star, Table as TableIcon, TableCellsMerge, TableCellsSplit,
  TextAlignCenter, TextAlignEnd, TextAlignJustify, TextAlignStart, Trash2, Undo, Unlink, User, X,
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

fn word_count(content: &str) -> usize {
  content.split_whitespace().count()
}

const TABLE_BUTTON_CLASS: &str = "rounded-md border border-[var(--primary-color-6)] px-2 py-1 text-xs text-[var(--secondary-color)] hover:bg-[color-mix(in_srgb,var(--secondary-color)_6%,transparent)]";

fn toolbar_button(
  label: &'static str,
  tooltip: &'static str,
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
                "aria-label": label,
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
  label: &'static str,
  tooltip: &'static str,
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
                {label}
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
}

#[component]
pub fn NoteEditorPanel(props: NoteEditorPanelProps) -> Element {
  let NoteEditorPanelProps { note } = props;
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

  let folder_name = store
    .folders()
    .into_iter()
    .find(|folder| Some(&folder.id) == note.folder_id.as_ref())
    .map(|folder| folder.name)
    .unwrap_or_else(|| "\u{2014}".to_string());
  let words = word_count(&note.content);
  let addable_tags: Vec<_> = store
    .tags()
    .into_iter()
    .filter(|tag| !note.tag_ids.contains(&tag.id))
    .collect();

  rsx! {
      div { class: "flex h-full flex-1 flex-col overflow-hidden",
          div { class: "flex flex-none flex-col border-b border-[var(--primary-color-6)] p-3 md:px-6 md:py-4",
          div { class: "flex items-center gap-2",
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
                  class: "h-auto flex-1 border-none bg-transparent px-0 py-1 text-xl font-medium shadow-none [outline:none] hover:bg-transparent focus:bg-transparent focus:shadow-none md:text-[28px]",
                  placeholder: "Untitled note",
                  value: note.title.clone(),
                  oninput: {
                      let note_id = note_id.clone();
                      move |event: FormEvent| store.set_note_title(&note_id, event.value())
                  },
              }
              button {
                  "aria-label": if note.starred { "Remove from Starred" } else { "Add to Starred" },
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
              button {
                  "aria-label": if note.pinned { "Unpin from top" } else { "Pin to top of list" },
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
                  "aria-label": "Delete note",
                  onclick: {
                      let note_id = note_id.clone();
                      move |_| {
                          store.delete_note(&note_id);
                          navigator().push(Route::Notes {});
                      }
                  },
                  Trash2 { size: "16px", stroke: "var(--secondary-color-5)" }
              }
          }
          div { class: "mt-2 flex flex-wrap items-center gap-2 text-xs text-[var(--secondary-color-5)]",
              ResponsivePopoverRoot {
                  open: (editor.folder_picker_open)(),
                  on_open_change: move |value| editor.folder_picker_open.set(value),
                  ResponsivePopoverTrigger {
                      class: "inline-flex items-center gap-[5px] rounded-md border-none bg-transparent px-[6px] py-[2px] -m-[6px] text-[var(--secondary-color-5)] hover:bg-[color-mix(in_srgb,var(--secondary-color)_8%,transparent)] hover:text-[var(--accent)]",
                      title: "Move to folder",
                      FileText { size: "13px" }
                      "{folder_name}"
                      ChevronDown { size: "11px" }
                  }
                  ResponsivePopoverContent {
                      title: "Move to folder",
                      align: ContentAlign::Start,
                      class: "w-52 items-stretch gap-1 p-1.5 text-left",
                      if !is_mobile() {
                          div { class: "px-2 py-1 text-[10px] font-medium uppercase tracking-wider text-[var(--secondary-color-5)]",
                              "Move to folder"
                          }
                      }
                      div {
                          class: if is_mobile() {
                              if note.folder_id.is_none() { "flex cursor-pointer items-center gap-2 rounded-[11px] px-[14px] py-[13px] text-sm bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] text-[var(--accent)]" } else { "flex cursor-pointer items-center gap-2 rounded-[11px] px-[14px] py-[13px] text-sm text-[var(--secondary-color)] bg-[var(--primary-color-2)]" }
                          } else if note.folder_id.is_none() {
                              "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1.5 text-sm bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] text-[var(--accent)]"
                          } else {
                              "flex cursor-pointer items-center gap-2 rounded-md px-2 py-1.5 text-sm text-[var(--secondary-color)] hover:bg-[var(--primary-color-4)]"
                          },
                          onclick: {
                              let note_id = note_id.clone();
                              move |_| {
                                  store.set_note_folder(&note_id, None);
                                  editor.folder_picker_open.set(false);
                              }
                          },
                          FileText { size: "15px" }
                          "No folder"
                      }
                      for folder in store.folders() {
                          {
                              let folder_id = folder.id.clone();
                              let is_active = note.folder_id.as_deref() == Some(folder.id.as_str());
                              let class = if is_mobile() {
                                  if is_active {
                                      "flex cursor-pointer items-center gap-2 rounded-[11px] px-[14px] py-[13px] text-sm bg-[color-mix(in_srgb,var(--accent)_15%,transparent)] text-[var(--accent)]"
                                  } else {
                                      "flex cursor-pointer items-center gap-2 rounded-[11px] px-[14px] py-[13px] text-sm text-[var(--secondary-color)] bg-[var(--primary-color-2)]"
                                  }
                              } else if is_active {
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
                                          move |_| {
                                              store.set_note_folder(&note_id, Some(folder_id.clone()));
                                              editor.folder_picker_open.set(false);
                                          }
                                      },
                                      {folder_icon(folder.icon)}
                                      "{folder.name}"
                                  }
                              }
                          }
                      }
                  }
              }
              span { "\u{b7}" }
              span { "Edited {format_relative_time(note.updated_at_ms)}" }
              span { "\u{b7}" }
              span { "{words} words" }
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
                              "aria-label": "Remove tag",
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
                      "+ tag"
                  }
                  ResponsivePopoverContent {
                      title: "Add tag",
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
                              placeholder: "New tag, then Enter",
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
          div { class: "mt-3 flex flex-nowrap items-center gap-1 overflow-x-auto overflow-y-hidden [scrollbar-width:none] [&::-webkit-scrollbar]:hidden md:flex-wrap md:overflow-visible",
              {toolbar_button("Bold", "Bold", rsx! { Bold { size: "14px" } }, move || editor.markdown_editor.toggle_bold())}
              {toolbar_button("Italic", "Italic", rsx! { Italic { size: "14px" } }, move || editor.markdown_editor.toggle_italic())}
              {toolbar_button("Code", "Inline code", rsx! { Code { size: "14px" } }, move || editor.markdown_editor.toggle_code())}
              {toolbar_button("Link", "Add link…", rsx! { LinkIcon { size: "14px" } }, move || editor.open_link_dialog())}
              {toolbar_button("Remove link", "Remove link", rsx! { Unlink { size: "14px" } }, move || editor.markdown_editor.remove_link())}
              span { class: "mx-1 h-4 w-px shrink-0 bg-[var(--primary-color-6)]" }
              {toolbar_button("Paragraph", "Convert to plain paragraph", rsx! { Pilcrow { size: "14px" } }, move || editor.markdown_editor.set_paragraph())}
              {toolbar_button("Heading 1", "Heading 1", rsx! { Heading1 { size: "14px" } }, move || editor.markdown_editor.set_heading(1))}
              {toolbar_button("Heading 2", "Heading 2", rsx! { Heading2 { size: "14px" } }, move || editor.markdown_editor.set_heading(2))}
              {toolbar_button("Heading 3", "Heading 3", rsx! { Heading3 { size: "14px" } }, move || editor.markdown_editor.set_heading(3))}
              {toolbar_button("Quote", "Toggle blockquote", rsx! { Quote { size: "14px" } }, move || editor.markdown_editor.toggle_blockquote())}
              {toolbar_button("Code block", "Code block", rsx! { Code { size: "14px" } }, move || editor.markdown_editor.set_code_block())}
              span { class: "mx-1 h-4 w-px shrink-0 bg-[var(--primary-color-6)]" }
              {toolbar_button("Bulleted list", "Bulleted list", rsx! { List { size: "14px" } }, move || editor.markdown_editor.toggle_bullet_list())}
              {toolbar_button("Numbered list", "Numbered list", rsx! { ListOrdered { size: "14px" } }, move || editor.markdown_editor.toggle_ordered_list())}
              {toolbar_button("Lift out of list", "Lift out of list", rsx! { ListIndentDecrease { size: "14px" } }, move || editor.markdown_editor.lift_list_item())}
              span { class: "mx-1 h-4 w-px shrink-0 bg-[var(--primary-color-6)]" }
              {toolbar_button("Align left", "Align left", rsx! { TextAlignStart { size: "14px" } }, move || editor.markdown_editor.align_left())}
              {toolbar_button("Align center", "Align center", rsx! { TextAlignCenter { size: "14px" } }, move || editor.markdown_editor.align_center())}
              {toolbar_button("Align right", "Align right", rsx! { TextAlignEnd { size: "14px" } }, move || editor.markdown_editor.align_right())}
              {toolbar_button("Justify", "Justify text", rsx! { TextAlignJustify { size: "14px" } }, move || editor.markdown_editor.align_justify())}
              span { class: "mx-1 h-4 w-px shrink-0 bg-[var(--primary-color-6)]" }
              {toolbar_button("Uppercase", "Convert selection to UPPERCASE", rsx! { CaseUpper { size: "14px" } }, move || editor.markdown_editor.to_uppercase())}
              {toolbar_button("Lowercase", "Convert selection to lowercase", rsx! { CaseLower { size: "14px" } }, move || editor.markdown_editor.to_lowercase())}
              span { class: "mx-1 h-4 w-px shrink-0 bg-[var(--primary-color-6)]" }
              {toolbar_button("Undo", "Undo", rsx! { Undo { size: "14px" } }, move || editor.markdown_editor.undo())}
              {toolbar_button("Redo", "Redo", rsx! { Redo { size: "14px" } }, move || editor.markdown_editor.redo())}
              {toolbar_button("Select all", "Select all", rsx! { SquareDashedMousePointer { size: "14px" } }, move || editor.markdown_editor.select_all())}
          }
          div { class: "mt-1.5 flex flex-nowrap items-center gap-1.5 overflow-x-auto overflow-y-hidden [scrollbar-width:none] [&::-webkit-scrollbar]:hidden md:flex-wrap md:overflow-visible",
              span { class: "shrink-0 text-[10px] font-medium uppercase tracking-wider text-[var(--secondary-color-5)]", "Table" }
              {toolbar_button("Insert 3x3 table", "Insert a 3x3 table at the cursor", rsx! { TableIcon { size: "14px" } }, move || editor.markdown_editor.insert_table(3, 3))}
              {table_button("+ Row", "Insert a row after the current one", move || editor.markdown_editor.add_row())}
              {table_button("+ Col", "Insert a column after the current one", move || editor.markdown_editor.add_column())}
              {table_button("− Row", "Delete the current row", move || editor.markdown_editor.delete_row())}
              {table_button("− Col", "Delete the current column", move || editor.markdown_editor.delete_column())}
              {table_button("Header row", "Toggle the first row as a header", move || editor.markdown_editor.toggle_header_row())}
              {toolbar_button("Merge row", "Merge the selected cells across the current row", rsx! { Merge { size: "14px" } }, move || editor.markdown_editor.merge_row())}
              {toolbar_button("Merge column", "Merge the selected cells down the current column", rsx! { TableCellsMerge { size: "14px" } }, move || editor.markdown_editor.merge_column())}
              {toolbar_button("Split cell", "Split a previously merged cell", rsx! { TableCellsSplit { size: "14px" } }, move || editor.markdown_editor.split_cell())}
              {table_button("Delete table", "Delete the whole table", move || editor.markdown_editor.delete_table())}
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
  }
}
