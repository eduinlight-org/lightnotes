use dioxus::prelude::*;
use taino_edit_core::markdown::{parse_markdown, to_markdown};
use taino_edit_extensions::{
  Align, Blockquote, Bold, Code, CodeBlock, Extension, Heading, History, Image, Italic, Link, Lists, Paragraph, Table,
  TransformCase,
};

pub use taino_edit_core::{Attrs, Node, NodeSpec, Schema, SchemaBuilder};

pub const EDITOR_ID: &str = "lightnotes-editor";

fn default_extensions() -> [&'static dyn Extension; 14] {
  [
    &Paragraph, &Heading, &Bold, &Italic, &Code, &Link, &Blockquote, &CodeBlock, &Lists, &History, &Image, &Align,
    &Table, &TransformCase,
  ]
}

fn build_default_schema() -> Schema {
  let base = SchemaBuilder::new()
    .node("doc", NodeSpec { content: Some("block*".into()), ..Default::default() })
    .node("text", NodeSpec { group: Some("inline".into()), ..Default::default() });
  taino_edit_extensions::build_schema_with(base, &default_extensions(), "doc").expect("default schema builds")
}

fn empty_doc(schema: &Schema) -> Node {
  let paragraph = schema
    .node("paragraph", Attrs::new(), vec![], vec![])
    .expect("paragraph is always schema-valid empty");
  schema
    .node(schema.top_node_type().name(), Attrs::new(), vec![paragraph], vec![])
    .expect("doc with a single empty paragraph is always schema-valid")
}

const BLANK_PARAGRAPH_MARKER: &str = "\u{200B}";

fn expand_blank_paragraph_markers(markdown: &str) -> String {
  fn flush_blank_run(out_lines: &mut Vec<String>, blank_run: &mut usize) {
    if *blank_run == 0 {
      return;
    }
    out_lines.push(String::new());
    for _ in 0..(*blank_run - 1) / 2 {
      out_lines.push(BLANK_PARAGRAPH_MARKER.to_string());
      out_lines.push(String::new());
    }
    *blank_run = 0;
  }

  let mut out_lines: Vec<String> = Vec::new();
  let mut in_fence = false;
  let mut blank_run = 0usize;

  for raw_line in markdown.lines() {
    let trimmed = raw_line.trim();
    let is_fence_delim = trimmed.starts_with("```") || trimmed.starts_with("~~~");

    if in_fence {
      out_lines.push(raw_line.to_string());
      if is_fence_delim {
        in_fence = false;
      }
      continue;
    }

    if is_fence_delim {
      flush_blank_run(&mut out_lines, &mut blank_run);
      out_lines.push(raw_line.to_string());
      in_fence = true;
      continue;
    }

    if trimmed.is_empty() {
      blank_run += 1;
      continue;
    }

    flush_blank_run(&mut out_lines, &mut blank_run);
    out_lines.push(raw_line.to_string());
  }
  flush_blank_run(&mut out_lines, &mut blank_run);

  out_lines.join("\n")
}

fn strip_blank_paragraph_markers(schema: &Schema, node: &Node) -> Node {
  if node.is_leaf() {
    return node.clone();
  }

  if node.node_type().name() == "paragraph" && node.text_content() == BLANK_PARAGRAPH_MARKER {
    return schema
      .node("paragraph", node.attrs().clone(), vec![], vec![])
      .expect("empty paragraph is always schema-valid");
  }

  let children: Vec<Node> = node.content().iter().map(|child| strip_blank_paragraph_markers(schema, child)).collect();

  schema
    .node(node.node_type().name(), node.attrs().clone(), children, node.marks().to_vec())
    .unwrap_or_else(|_| node.clone())
}

fn doc_from_markdown(schema: &Schema, markdown: &str) -> Node {
  let expanded = expand_blank_paragraph_markers(markdown);
  let doc = parse_markdown(schema, &expanded).unwrap_or_else(|_| empty_doc(schema));
  let doc = strip_blank_paragraph_markers(schema, &doc);
  if doc.child_count() == 0 {
    empty_doc(schema)
  } else {
    doc
  }
}

const BLOCK_TAGS_NEEDING_PLACEHOLDER: [&str; 9] = ["p", "h1", "h2", "h3", "h4", "h5", "h6", "li", "blockquote"];

fn render_html(doc: &Node) -> String {
  let mut html = doc.to_html();
  for tag in BLOCK_TAGS_NEEDING_PLACEHOLDER {
    let empty = format!("<{tag}></{tag}>");
    let filled = format!("<{tag}><br></{tag}>");
    html = html.replace(&empty, &filled);
  }
  html
}

fn doc_from_html(schema: &Schema, html: &str) -> Node {
  let doc = schema.parse_html(html).unwrap_or_else(|_| empty_doc(schema));
  if doc.child_count() == 0 {
    empty_doc(schema)
  } else {
    doc
  }
}

pub async fn prompt(message: &'static str) -> Option<String> {
  let mut eval = document::eval(
    r#"
    const [message] = await dioxus.recv();
    dioxus.send(window.prompt(message));
    "#,
  );
  let _ = eval.send((message,));
  let result = eval.recv::<Option<String>>().await.ok().flatten();
  result.filter(|value| !value.is_empty())
}

pub async fn selected_text() -> String {
  let mut eval = document::eval(
    r#"
    const sel = window.getSelection();
    dioxus.send(sel ? sel.toString() : "");
    "#,
  );
  eval.recv::<String>().await.unwrap_or_default()
}

pub async fn save_selection_range() {
  let mut eval = document::eval(
    r#"
    const sel = window.getSelection();
    window.__noteEditorSavedRange = sel && sel.rangeCount > 0 && !sel.isCollapsed
      ? sel.getRangeAt(0).cloneRange()
      : null;
    dioxus.send(true);
    "#,
  );
  let _ = eval.recv::<bool>().await;
}

pub async fn current_link() -> Option<(String, String)> {
  let mut eval = document::eval(
    r#"
    const sel = window.getSelection();
    let node = sel && sel.anchorNode;
    if (node && node.nodeType === Node.TEXT_NODE) node = node.parentElement;
    let a = node ? node.closest('a') : null;
    if (!a && node && typeof sel.anchorOffset === 'number') {
      const before = node.childNodes[sel.anchorOffset - 1];
      const after = node.childNodes[sel.anchorOffset];
      const fromElement = (n) => n && n.nodeType === Node.ELEMENT_NODE ? n.closest('a') : null;
      a = fromElement(before) || fromElement(after);
    }
    dioxus.send(a ? [a.textContent, a.getAttribute('href') || ''] : null);
    "#,
  );
  eval.recv::<Option<(String, String)>>().await.ok().flatten()
}

async fn read_inner_html() -> Option<String> {
  let mut eval = document::eval(
    r#"
    const [id] = await dioxus.recv();
    const el = document.getElementById(id);
    dioxus.send(el ? el.innerHTML : null);
    "#,
  );
  let _ = eval.send((EDITOR_ID,));
  eval.recv::<Option<String>>().await.ok().flatten()
}

async fn run_exec_command(command: &'static str) {
  let eval = document::eval(
    r#"
    const [id, command] = await dioxus.recv();
    const el = document.getElementById(id);
    if (el) { el.focus(); document.execCommand(command); }
    "#,
  );
  let _ = eval.send((EDITOR_ID, command));
}

async fn run_exec_command_with_value(command: &'static str, value: String) {
  let eval = document::eval(
    r#"
    const [id, command, value] = await dioxus.recv();
    const el = document.getElementById(id);
    if (el) { el.focus(); document.execCommand(command, false, value); }
    "#,
  );
  let _ = eval.send((EDITOR_ID, command, value));
}

async fn toggle_format_block(tag: &'static str) {
  let eval = document::eval(
    r#"
    const [id, tag] = await dioxus.recv();
    const el = document.getElementById(id);
    if (el) {
      el.focus();
      const current = (document.queryCommandValue('formatBlock') || '').toLowerCase();
      document.execCommand('formatBlock', false, current === tag.toLowerCase() ? 'p' : tag);
    }
    "#,
  );
  let _ = eval.send((EDITOR_ID, tag));
}

async fn transform_case(uppercase: bool) {
  let eval = document::eval(
    r#"
    const [id, uppercase] = await dioxus.recv();
    const el = document.getElementById(id);
    if (el) {
      el.focus();
      const sel = window.getSelection();
      if (sel && sel.rangeCount && !sel.isCollapsed) {
        const text = sel.toString();
        const transformed = uppercase ? text.toUpperCase() : text.toLowerCase();
        document.execCommand('insertText', false, transformed);
      }
    }
    "#,
  );
  let _ = eval.send((EDITOR_ID, uppercase));
}

async fn insert_html(html: String) {
  let eval = document::eval(
    r#"
    const [id, html] = await dioxus.recv();
    const el = document.getElementById(id);
    if (el) { el.focus(); document.execCommand('insertHTML', false, html); }
    "#,
  );
  let _ = eval.send((EDITOR_ID, html));
}

async fn insert_link_html(html: String) {
  let eval = document::eval(
    r#"
    const [id, html] = await dioxus.recv();
    const el = document.getElementById(id);
    if (!el) { return; }
    el.focus();
    const sel = window.getSelection();
    let node = sel && sel.anchorNode;
    if (node && node.nodeType === Node.TEXT_NODE) node = node.parentElement;
    let existing = node ? node.closest('a') : null;
    if (!existing && node && typeof sel.anchorOffset === 'number') {
      const before = node.childNodes[sel.anchorOffset - 1];
      const after = node.childNodes[sel.anchorOffset];
      const fromElement = (n) => n && n.nodeType === Node.ELEMENT_NODE ? n.closest('a') : null;
      existing = fromElement(before) || fromElement(after);
    }
    if (existing) {
      const range = document.createRange();
      range.selectNode(existing);
      sel.removeAllRanges();
      sel.addRange(range);
    } else if (window.__noteEditorSavedRange) {
      sel.removeAllRanges();
      sel.addRange(window.__noteEditorSavedRange);
    }
    document.execCommand('insertHTML', false, html);
    window.__noteEditorSavedRange = null;
    "#,
  );
  let _ = eval.send((EDITOR_ID, html));
}

async fn run_table_action(action: &'static str) {
  let eval = document::eval(
    r#"
    const [id, action] = await dioxus.recv();
    const el = document.getElementById(id);
    if (!el) { return; }
    const sel = window.getSelection();
    if (!sel || !sel.anchorNode) { return; }
    let node = sel.anchorNode;
    if (node.nodeType === Node.TEXT_NODE) node = node.parentElement;
    const cell = node.closest('td, th');
    const row = node.closest('tr');
    const table = node.closest('table');
    if (!table) { return; }
    if (action === 'addRow') {
      const newRow = table.insertRow(row ? row.rowIndex + 1 : -1);
      const cols = row ? row.cells.length : (table.rows[0] ? table.rows[0].cells.length : 1);
      for (let c = 0; c < cols; c++) { const td = newRow.insertCell(); td.innerHTML = '<br>'; }
    } else if (action === 'deleteRow') {
      if (row) {
        if (table.rows.length <= 1) { table.remove(); }
        else { table.deleteRow(row.rowIndex); }
      }
    } else if (action === 'addCol') {
      const colIndex = cell ? cell.cellIndex + 1 : 0;
      for (const r of table.rows) { const td = r.insertCell(colIndex); td.innerHTML = '<br>'; }
    } else if (action === 'deleteCol') {
      const colIndex = cell ? cell.cellIndex : 0;
      if (table.rows[0] && table.rows[0].cells.length <= 1) { table.remove(); }
      else { for (const r of table.rows) { if (r.cells[colIndex]) r.deleteCell(colIndex); } }
    } else if (action === 'deleteTable') {
      table.remove();
    } else if (action === 'toggleHeaderRow') {
      const firstRow = table.rows[0];
      if (firstRow) {
        const isHeader = firstRow.cells[0] && firstRow.cells[0].tagName === 'TH';
        const newRow = document.createElement('tr');
        for (const c of Array.from(firstRow.cells)) {
          const newCell = document.createElement(isHeader ? 'td' : 'th');
          newCell.innerHTML = c.innerHTML;
          newRow.appendChild(newCell);
        }
        firstRow.replaceWith(newRow);
      }
    } else if (action === 'mergeRow') {
      if (cell && cell.nextElementSibling && (cell.nextElementSibling.tagName === 'TD' || cell.nextElementSibling.tagName === 'TH')) {
        const next = cell.nextElementSibling;
        cell.colSpan = (cell.colSpan || 1) + (next.colSpan || 1);
        cell.innerHTML = cell.innerHTML + ' ' + next.innerHTML;
        next.remove();
      }
    } else if (action === 'mergeColumn') {
      if (cell && row) {
        const nextRow = row.nextElementSibling;
        const colIndex = cell.cellIndex;
        if (nextRow && nextRow.cells[colIndex]) {
          const below = nextRow.cells[colIndex];
          cell.rowSpan = (cell.rowSpan || 1) + (below.rowSpan || 1);
          cell.innerHTML = cell.innerHTML + ' ' + below.innerHTML;
          below.remove();
        }
      }
    } else if (action === 'splitCell') {
      if (cell) {
        if ((cell.colSpan || 1) > 1) {
          const span = cell.colSpan;
          cell.colSpan = 1;
          for (let i = 1; i < span; i++) {
            const td = document.createElement(cell.tagName.toLowerCase());
            td.innerHTML = '<br>';
            cell.after(td);
          }
        }
        if ((cell.rowSpan || 1) > 1) {
          cell.rowSpan = 1;
        }
      }
    }
    document.getElementById(id).dispatchEvent(new Event('input', { bubbles: true }));
    "#,
  );
  let _ = eval.send((EDITOR_ID, action));
}

#[derive(Clone, Copy, PartialEq)]
pub struct MarkdownEditorHandle {
  markdown: Signal<String>,
  html: Signal<String>,
  schema: Signal<Schema>,
}

impl MarkdownEditorHandle {
  pub fn markdown(&self) -> String {
    self.markdown.read().clone()
  }

  pub fn load(&mut self, markdown: &str) -> String {
    let schema = self.schema.peek().clone();
    let doc = doc_from_markdown(&schema, markdown);
    let normalized = to_markdown(&doc);
    self.html.set(render_html(&doc));
    self.markdown.set(normalized.clone());
    normalized
  }

  pub fn sync_from_dom(&mut self) {
    let mut markdown = self.markdown;
    let schema = self.schema.peek().clone();
    spawn(async move {
      if let Some(html) = read_inner_html().await {
        let doc = doc_from_html(&schema, &html);
        markdown.set(to_markdown(&doc));
      }
    });
  }

  fn exec(&mut self, command: &'static str) {
    self.sync_after(async move { run_exec_command(command).await });
  }

  fn exec_with_value(&mut self, command: &'static str, value: String) {
    self.sync_after(async move { run_exec_command_with_value(command, value).await });
  }

  fn sync_after(&mut self, action: impl std::future::Future<Output = ()> + 'static) {
    let mut markdown = self.markdown;
    let schema = self.schema.peek().clone();
    spawn(async move {
      action.await;
      if let Some(html) = read_inner_html().await {
        let doc = doc_from_html(&schema, &html);
        markdown.set(to_markdown(&doc));
      }
    });
  }

  pub fn insert_link(&mut self, text: String, href: String) {
    let html = format!(
      "<a href=\"{}\">{}</a>",
      href.replace('&', "&amp;").replace('"', "&quot;"),
      text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
    );
    self.sync_after(async move { insert_link_html(html).await });
  }

  pub fn remove_link(&mut self) {
    self.exec("unlink");
  }

  pub fn toggle_bold(&mut self) {
    self.exec("bold");
  }

  pub fn toggle_italic(&mut self) {
    self.exec("italic");
  }

  pub fn toggle_code(&mut self) {
    self.sync_after(async move {
      let eval = document::eval(
        r#"
        const [id] = await dioxus.recv();
        const el = document.getElementById(id);
        if (el) {
          el.focus();
          const sel = window.getSelection();
          const text = sel && sel.rangeCount ? sel.toString() : '';
          document.execCommand('insertHTML', false, '<code>' + (text || 'code').replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;') + '</code>');
        }
        "#,
      );
      let _ = eval.send((EDITOR_ID,));
    });
  }

  pub fn set_heading(&mut self, level: u64) {
    let tag: &'static str = match level {
      1 => "h1",
      2 => "h2",
      _ => "h3",
    };
    self.sync_after(async move { toggle_format_block(tag).await });
  }

  pub fn set_paragraph(&mut self) {
    self.exec_with_value("formatBlock", "p".to_string());
  }

  pub fn toggle_blockquote(&mut self) {
    self.sync_after(async move { toggle_format_block("blockquote").await });
  }

  pub fn set_code_block(&mut self) {
    self.sync_after(async move { toggle_format_block("pre").await });
  }

  pub fn toggle_bullet_list(&mut self) {
    self.exec("insertUnorderedList");
  }

  pub fn toggle_ordered_list(&mut self) {
    self.exec("insertOrderedList");
  }

  pub fn lift_list_item(&mut self) {
    self.exec("outdent");
  }

  pub fn undo(&mut self) {
    self.exec("undo");
  }

  pub fn redo(&mut self) {
    self.exec("redo");
  }

  pub fn select_all(&mut self) {
    self.exec("selectAll");
  }

  pub fn insert_image(&mut self, src: String, _alt: Option<String>) {
    let html = format!("<img src=\"{}\">", src.replace('&', "&amp;").replace('"', "&quot;"));
    self.sync_after(async move { insert_html(html).await });
  }

  pub fn insert_image_via_prompt(&mut self) {
    let mut this = *self;
    spawn(async move {
      if let Some(src) = prompt("Image URL:").await {
        this.insert_image(src, None);
      }
    });
  }

  pub fn align_left(&mut self) {
    self.exec("justifyLeft");
  }

  pub fn align_center(&mut self) {
    self.exec("justifyCenter");
  }

  pub fn align_right(&mut self) {
    self.exec("justifyRight");
  }

  pub fn align_justify(&mut self) {
    self.exec("justifyFull");
  }

  pub fn to_uppercase(&mut self) {
    self.sync_after(async move { transform_case(true).await });
  }

  pub fn to_lowercase(&mut self) {
    self.sync_after(async move { transform_case(false).await });
  }

  pub fn insert_table(&mut self, rows: usize, cols: usize) {
    let mut html = String::from("<table>");
    for _ in 0..rows {
      html.push_str("<tr>");
      for _ in 0..cols {
        html.push_str("<td><br></td>");
      }
      html.push_str("</tr>");
    }
    html.push_str("</table><p><br></p>");
    self.sync_after(async move { insert_html(html).await });
  }

  fn table_action(&mut self, action: &'static str) {
    self.sync_after(async move { run_table_action(action).await });
  }

  pub fn add_row(&mut self) {
    self.table_action("addRow");
  }

  pub fn add_column(&mut self) {
    self.table_action("addCol");
  }

  pub fn delete_row(&mut self) {
    self.table_action("deleteRow");
  }

  pub fn delete_column(&mut self) {
    self.table_action("deleteCol");
  }

  pub fn delete_table(&mut self) {
    self.table_action("deleteTable");
  }

  pub fn toggle_header_row(&mut self) {
    self.table_action("toggleHeaderRow");
  }

  pub fn merge_row(&mut self) {
    self.table_action("mergeRow");
  }

  pub fn merge_column(&mut self) {
    self.table_action("mergeColumn");
  }

  pub fn split_cell(&mut self) {
    self.table_action("splitCell");
  }
}

pub fn use_markdown_editor(initial_markdown: impl Into<String>) -> MarkdownEditorHandle {
  let initial_markdown = initial_markdown.into();
  let schema = use_signal(build_default_schema);
  let (markdown, html) = {
    let schema = schema.peek().clone();
    let doc = doc_from_markdown(&schema, &initial_markdown);
    (to_markdown(&doc), render_html(&doc))
  };
  MarkdownEditorHandle { markdown: use_signal(|| markdown), html: use_signal(|| html), schema }
}

#[derive(PartialEq, Clone, Props)]
pub struct MarkdownEditorViewProps {
  pub handle: MarkdownEditorHandle,
  #[props(default)]
  pub class: Option<String>,
}

#[component]
pub fn MarkdownEditorView(props: MarkdownEditorViewProps) -> Element {
  let MarkdownEditorViewProps { mut handle, class } = props;
  let html = (handle.html)();

  use_effect(move || {
    spawn(async move {
      let mut eval = document::eval(
        r#"
        const [id] = await dioxus.recv();
        const el = document.getElementById(id);
        if (el) {
          el.addEventListener('input', () => dioxus.send(true));
        }
        "#,
      );
      let _ = eval.send((EDITOR_ID,));
      while eval.recv::<bool>().await.is_ok() {
        handle.sync_from_dom();
      }
    });
  });

  rsx! {
    div {
      class: class.unwrap_or_default(),
      id: EDITOR_ID,
      contenteditable: "true",
      dangerous_inner_html: "{html}",
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn blank_paragraph_survives_markdown_roundtrip() {
    let schema = build_default_schema();

    let html = "<p>line one</p><p><br></p><p>line two</p><p><br></p><p>line three</p>";
    let doc_from_dom = doc_from_html(&schema, html);
    let md_from_dom = to_markdown(&doc_from_dom);

    let doc_reparsed = doc_from_markdown(&schema, &md_from_dom);
    let md_after_reparse = to_markdown(&doc_reparsed);
    assert_eq!(md_from_dom, md_after_reparse);

    let html_after_reparse = render_html(&doc_reparsed);
    assert_eq!(html_after_reparse, html);
  }

  #[test]
  fn code_block_blank_lines_are_untouched() {
    let schema = build_default_schema();
    let md = "before\n\n```\nfn main() {\n\n\n}\n```\n\nafter\n";
    let doc = doc_from_markdown(&schema, md);
    assert_eq!(to_markdown(&doc), md);
  }
}
