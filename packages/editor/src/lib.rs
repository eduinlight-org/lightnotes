mod adapter;

use dioxus::prelude::*;
use serde_json::json;
use taino_edit_core::markdown::{parse_markdown, to_markdown};
use taino_edit_extensions::{
  lift_list_item, redo_command, remove_link, set_link, undo_command, wrap_in_bullet_list, wrap_in_ordered_list,
  Blockquote, Bold, Code, CodeBlock, Extension, Heading, History, Italic, Link, Lists, Paragraph,
};

pub use adapter::{KeymapProp, TainoEditor, ViewPlugins};
pub use taino_edit_core::{
  base_keymap, lift, remove_mark, select_all, set_block_type, set_mark, split_block, toggle_mark, wrap_in, AttrSpec,
  AttrValue, Attrs, Command, Dispatch, EditorState, KeyPress, Keymap, Mark, MarkSpec, MarkType, Node, NodeSpec,
  NodeType, Plugin, PluginKey, PluginSet, ResolvedPos, Schema, SchemaBuilder, Selection, Slice, Transaction, Transform,
};
pub use taino_edit_dom::{Decoration, EditorView, ViewAction, ViewDesc, ViewPlugin};

fn default_extensions() -> [&'static dyn Extension; 10] {
  [&Paragraph, &Heading, &Bold, &Italic, &Code, &Link, &Blockquote, &CodeBlock, &Lists, &History]
}

fn build_default_schema() -> Schema {
  let base = SchemaBuilder::new()
    .node("doc", NodeSpec { content: Some("block*".into()), ..Default::default() })
    .node("text", NodeSpec { group: Some("inline".into()), ..Default::default() });
  taino_edit_extensions::build_schema_with(base, &default_extensions(), "doc").expect("default schema builds")
}

fn build_default_keymap(schema: &Schema) -> Keymap {
  taino_edit_extensions::build_keymap_with(&default_extensions(), schema, false)
}

pub fn prompt(message: &str) -> Option<String> {
  web_sys::window().and_then(|window| window.prompt_with_message(message).ok().flatten()).filter(|value| !value.is_empty())
}

fn empty_doc(schema: &Schema) -> Node {
  schema
    .node(schema.top_node_type().name(), Attrs::new(), vec![], vec![])
    .expect("empty doc is always schema-valid")
}

fn doc_from_markdown(schema: &Schema, markdown: &str) -> Node {
  parse_markdown(schema, markdown).unwrap_or_else(|_| empty_doc(schema))
}

#[derive(Clone, Copy, PartialEq)]
pub struct MarkdownEditorHandle {
  pub state: Signal<EditorState>,
}

impl MarkdownEditorHandle {
  pub fn markdown(&self) -> String {
    to_markdown(self.state.read().doc())
  }

  pub fn load(&mut self, markdown: &str) -> String {
    let schema = self.state.peek().schema().clone();
    let doc = doc_from_markdown(&schema, markdown);
    let normalized = to_markdown(&doc);
    self.state.set(EditorState::new(doc, schema));
    normalized
  }

  fn run(&mut self, cmd: Command) {
    let snapshot = self.state.peek().clone();
    let mut next = None;
    {
      let mut dispatch = |tx: Transaction| next = Some(snapshot.apply(tx));
      cmd(&snapshot, Some(&mut dispatch));
    }
    if let Some(n) = next {
      self.state.set(n);
    }
  }

  fn toggle_mark_named(&mut self, name: &str) {
    let schema = self.state.peek().schema().clone();
    if let Some(mark_type) = schema.mark_type(name) {
      self.run(toggle_mark(mark_type.clone()));
    }
  }

  pub fn toggle_bold(&mut self) {
    self.toggle_mark_named("strong");
  }

  pub fn toggle_italic(&mut self) {
    self.toggle_mark_named("em");
  }

  pub fn toggle_code(&mut self) {
    self.toggle_mark_named("code");
  }

  pub fn set_heading(&mut self, level: u64) {
    self.run(set_block_type("heading", Attrs::from_iter([("level".to_string(), json!(level))])));
  }

  pub fn set_paragraph(&mut self) {
    self.run(set_block_type("paragraph", Attrs::new()));
  }

  pub fn toggle_blockquote(&mut self) {
    self.run(wrap_in("blockquote", Attrs::new()));
  }

  pub fn set_code_block(&mut self) {
    self.run(set_block_type("code_block", Attrs::new()));
  }

  pub fn toggle_bullet_list(&mut self) {
    self.run(wrap_in_bullet_list());
  }

  pub fn toggle_ordered_list(&mut self) {
    self.run(wrap_in_ordered_list());
  }

  pub fn lift_list_item(&mut self) {
    self.run(lift_list_item());
  }

  pub fn set_link(&mut self, href: String) {
    self.run(set_link(href, None));
  }

  pub fn remove_link(&mut self) {
    self.run(remove_link());
  }

  pub fn undo(&mut self) {
    self.run(undo_command());
  }

  pub fn redo(&mut self) {
    self.run(redo_command());
  }
}

pub fn use_markdown_editor(initial_markdown: impl Into<String>) -> MarkdownEditorHandle {
  let initial_markdown = initial_markdown.into();
  let state = use_signal(move || {
    let schema = build_default_schema();
    let doc = doc_from_markdown(&schema, &initial_markdown);
    EditorState::new(doc, schema)
  });
  MarkdownEditorHandle { state }
}

#[derive(PartialEq, Clone, Props)]
pub struct MarkdownEditorViewProps {
  pub handle: MarkdownEditorHandle,
  #[props(default)]
  pub class: Option<String>,
}

#[component]
pub fn MarkdownEditorView(props: MarkdownEditorViewProps) -> Element {
  let MarkdownEditorViewProps { handle, class } = props;
  let keymap = use_hook(|| KeymapProp::new(build_default_keymap(handle.state.peek().schema())));

  rsx! {
    div {
      class: class.unwrap_or_default(),
      TainoEditor { state: handle.state, keymap: keymap.clone() }
    }
  }
}
