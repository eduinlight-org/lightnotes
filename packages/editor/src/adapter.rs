use std::cell::{Cell, RefCell};
use std::rc::Rc;

use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use taino_edit_core::{EditorState, KeyPress, Keymap, Transaction, Transform};
pub use taino_edit_dom::ViewPlugin;
use taino_edit_dom::{EditorView, ViewAction};

#[derive(Clone, Default)]
pub struct ViewPlugins(PluginCell);

type PluginCell = Rc<RefCell<Option<Vec<Box<dyn ViewPlugin>>>>>;

impl ViewPlugins {
  pub fn new(plugins: Vec<Box<dyn ViewPlugin>>) -> Self {
    Self(Rc::new(RefCell::new(Some(plugins))))
  }

  fn take(&self) -> Vec<Box<dyn ViewPlugin>> {
    self.0.borrow_mut().take().unwrap_or_default()
  }
}

impl PartialEq for ViewPlugins {
  fn eq(&self, _other: &Self) -> bool {
    true
  }
}

impl std::fmt::Debug for ViewPlugins {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let n = self.0.borrow().as_ref().map_or(0, Vec::len);
    f.debug_struct("ViewPlugins").field("pending", &n).finish()
  }
}

#[derive(Clone, Default)]
pub struct KeymapProp(Rc<RefCell<Option<Keymap>>>);

impl KeymapProp {
  pub fn new(keymap: Keymap) -> Self {
    Self(Rc::new(RefCell::new(Some(keymap))))
  }

  fn take(&self) -> Option<Keymap> {
    self.0.borrow_mut().take()
  }
}

impl PartialEq for KeymapProp {
  fn eq(&self, _other: &Self) -> bool {
    true
  }
}

impl std::fmt::Debug for KeymapProp {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("KeymapProp").finish_non_exhaustive()
  }
}

#[derive(PartialEq, Clone, Props)]
pub struct TainoEditorProps {
  pub state: Signal<EditorState>,
  #[props(default)]
  pub plugins: ViewPlugins,
  #[props(default)]
  pub keymap: KeymapProp,
}

#[component]
pub fn TainoEditor(props: TainoEditorProps) -> Element {
  let TainoEditorProps { state, plugins, keymap } = props;

  let mut runtime: Signal<Option<EditorRuntime>> = use_signal(|| None);

  use_effect(move || {
    let snapshot = state.read().clone();
    if let Some(rt) = runtime.write().as_mut() {
      rt.view.update(snapshot.doc().clone());
      let mirrored_from_dom = rt.selection_from_dom.replace(false);
      if !mirrored_from_dom && rt.view.has_focus() && rt.view.read_selection() != Some(snapshot.selection()) {
        rt.applying_selection.set(true);
        let _ = rt.view.set_selection(snapshot.selection());
        rt.applying_selection.set(false);
      }
      rt.view.refresh_view_decorations(Some(snapshot.selection()));
    }
  });

  let on_mounted = move |evt: Event<MountedData>| {
    let Some(element) = evt.data().downcast::<web_sys::Element>().cloned() else {
      return;
    };
    let snapshot = state.read().clone();
    let mut view = EditorView::mount(snapshot.doc().clone(), snapshot.schema().clone(), element.clone());
    view.set_view_plugins(plugins.take());
    view.refresh_view_decorations(Some(snapshot.selection()));
    let applying = Rc::new(Cell::new(false));
    let from_dom = Rc::new(Cell::new(false));
    let keymap_cell: Rc<RefCell<Option<Keymap>>> = Rc::new(RefCell::new(keymap.take()));
    let closures = wire_events(&element, runtime, state, applying.clone(), from_dom.clone(), keymap_cell.clone());
    runtime.set(Some(EditorRuntime {
      view,
      closures,
      applying_selection: applying,
      selection_from_dom: from_dom,
      keymap: keymap_cell,
    }));
  };

  rsx! {
    div { class: "taino-editor", onmounted: on_mounted }
  }
}

struct EditorRuntime {
  view: EditorView,
  #[allow(dead_code)]
  closures: Vec<EventCloser>,
  applying_selection: Rc<Cell<bool>>,
  selection_from_dom: Rc<Cell<bool>>,
  #[allow(dead_code)]
  keymap: Rc<RefCell<Option<Keymap>>>,
}

struct EventCloser {
  event: &'static str,
  target: web_sys::EventTarget,
  closure: Closure<dyn FnMut(web_sys::Event)>,
  capture: bool,
}

impl Drop for EventCloser {
  fn drop(&mut self) {
    let _ =
      self.target.remove_event_listener_with_callback_and_bool(self.event, self.closure.as_ref().unchecked_ref(), self.capture);
  }
}

fn push_listener(closers: &mut Vec<EventCloser>, target: web_sys::EventTarget, event: &'static str, closure: Closure<dyn FnMut(web_sys::Event)>) {
  push_listener_capture(closers, target, event, closure, false);
}

fn push_listener_capture(
  closers: &mut Vec<EventCloser>,
  target: web_sys::EventTarget,
  event: &'static str,
  closure: Closure<dyn FnMut(web_sys::Event)>,
  capture: bool,
) {
  if target.add_event_listener_with_callback_and_bool(event, closure.as_ref().unchecked_ref(), capture).is_ok() {
    closers.push(EventCloser { event, target, closure, capture });
  }
}

fn wire_events(
  el: &web_sys::Element,
  mut runtime: Signal<Option<EditorRuntime>>,
  mut state: Signal<EditorState>,
  applying_selection: Rc<Cell<bool>>,
  selection_from_dom: Rc<Cell<bool>>,
  keymap_cell: Rc<RefCell<Option<Keymap>>>,
) -> Vec<EventCloser> {
  let target: web_sys::EventTarget = el.clone().into();
  let mut closers: Vec<EventCloser> = Vec::new();

  let cb = Closure::<dyn FnMut(web_sys::Event)>::new(move |_ev: web_sys::Event| {
    if let Some(Some(t)) = with_view(runtime, |v| v.read_dom_changes()) {
      apply_transform(state, &t);
    }
  });
  push_listener(&mut closers, target.clone(), "input", cb);

  let km_for_keydown = keymap_cell;
  let cb = Closure::<dyn FnMut(web_sys::Event)>::new(move |ev: web_sys::Event| {
    let Ok(kev) = ev.dyn_into::<web_sys::KeyboardEvent>() else {
      return;
    };
    let key = KeyPress { key: kev.key(), ctrl: kev.ctrl_key(), alt: kev.alt_key(), shift: kev.shift_key(), meta: kev.meta_key() };
    let mut cur = state.peek().clone();
    if let Some(Some(live)) = with_view(runtime, |v| v.read_selection()) {
      if live != cur.selection() {
        let mut tx = cur.tr();
        tx.set_selection(live);
        tx.no_history();
        cur = cur.apply(tx);
      }
    }
    let mut next = None;
    let handled = match km_for_keydown.borrow().as_ref() {
      Some(km) => {
        let mut d = |t: Transaction| next = Some(cur.apply(t));
        km.handle(&cur, &key, Some(&mut d))
      }
      None => false,
    };
    if let Some(n) = next {
      if let Some(rt) = runtime.write().as_mut() {
        rt.view.update(n.doc().clone());
        rt.applying_selection.set(true);
        let _ = rt.view.set_selection(n.selection());
        rt.applying_selection.set(false);
        rt.view.refresh_view_decorations(Some(n.selection()));
      }
      state.set(n);
    }
    let structural = matches!(key.key.as_str(), "Enter" | "Backspace" | "Delete");
    if handled || structural {
      kev.prevent_default();
    }
  });
  push_listener(&mut closers, target.clone(), "keydown", cb);

  let cb = Closure::<dyn FnMut(web_sys::Event)>::new(move |_ev: web_sys::Event| {
    with_view(runtime, |v| v.composition_start());
  });
  push_listener(&mut closers, target.clone(), "compositionstart", cb);

  let cb = Closure::<dyn FnMut(web_sys::Event)>::new(move |_ev: web_sys::Event| {
    let t = with_view(runtime, |v| {
      v.composition_end();
      v.read_dom_changes()
    })
    .flatten();
    if let Some(t) = t {
      apply_transform(state, &t);
    }
  });
  push_listener(&mut closers, target.clone(), "compositionend", cb);

  let cb = Closure::<dyn FnMut(web_sys::Event)>::new(move |ev: web_sys::Event| {
    let Ok(clip) = ev.dyn_into::<web_sys::ClipboardEvent>() else {
      return;
    };
    clip.prevent_default();
    let Some(data) = clip.clipboard_data() else {
      return;
    };
    let md = data.get_data("text/markdown").unwrap_or_default();
    let html = data.get_data("text/html").unwrap_or_default();
    let text = data.get_data("text/plain").unwrap_or_default();
    let t = with_view(runtime, |v| {
      if !md.is_empty() {
        v.paste_markdown(&md)
      } else if !html.is_empty() {
        v.paste_html(&html)
      } else if !text.is_empty() {
        v.paste_text(&text)
      } else {
        None
      }
    })
    .flatten();
    if let Some(t) = t {
      apply_transform(state, &t);
    }
  });
  push_listener(&mut closers, target.clone(), "paste", cb);

  for kind in ["mousedown", "mousemove", "mouseup"] {
    let cb = Closure::<dyn FnMut(web_sys::Event)>::new(move |ev: web_sys::Event| {
      if let Some(Some(action)) = with_view(runtime, |v| v.handle_view_event(&ev)) {
        apply_view_action(state, action);
      }
    });
    push_listener(&mut closers, target.clone(), kind, cb);
  }

  if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
    let doc_target: web_sys::EventTarget = doc.into();
    let applying = applying_selection;
    let from_dom = selection_from_dom;
    let cb = Closure::<dyn FnMut(web_sys::Event)>::new(move |_ev: web_sys::Event| {
      if applying.get() {
        return;
      }
      let Some(Some(sel)) = with_view(runtime, |v| v.read_selection()) else {
        return;
      };
      let cur = state.peek().selection();
      if sel == cur {
        return;
      }
      from_dom.set(true);
      let mut s = state;
      let next = {
        let snap = s.peek();
        let mut tx = snap.tr();
        tx.set_selection(sel);
        tx.no_history();
        snap.apply(tx)
      };
      s.set(next);
    });
    push_listener(&mut closers, doc_target, "selectionchange", cb);
  }

  if let Some(window) = web_sys::window() {
    let win_target: web_sys::EventTarget = window.unchecked_into();
    let cb = Closure::<dyn FnMut(web_sys::Event)>::new(move |_ev: web_sys::Event| {
      with_view(runtime, |v| v.reposition_inline_decorations());
    });
    push_listener_capture(&mut closers, win_target.clone(), "scroll", cb, true);
    let cb = Closure::<dyn FnMut(web_sys::Event)>::new(move |_ev: web_sys::Event| {
      with_view(runtime, |v| v.reposition_inline_decorations());
    });
    push_listener(&mut closers, win_target, "resize", cb);
  }

  closers
}

fn with_view<R>(runtime: Signal<Option<EditorRuntime>>, f: impl FnOnce(&EditorView) -> R) -> Option<R> {
  runtime.peek().as_ref().map(|rt| f(&rt.view))
}

fn apply_view_action(mut state: Signal<EditorState>, action: ViewAction) {
  match action {
    ViewAction::Select(sel) => {
      let next = {
        let snap = state.peek();
        let mut tx = snap.tr();
        tx.set_selection(sel);
        tx.no_history();
        snap.apply(tx)
      };
      state.set(next);
    }
    ViewAction::Command(cmd) => {
      let snapshot = state.peek().clone();
      let mut next = None;
      {
        let mut d = |tx: Transaction| next = Some(snapshot.apply(tx));
        cmd(&snapshot, Some(&mut d));
      }
      if let Some(n) = next {
        state.set(n);
      }
    }
  }
}

fn apply_transform(mut state: Signal<EditorState>, tr: &Transform) {
  let next = {
    let snap = state.peek();
    let mut tx = snap.tr();
    let mut ok = true;
    for step in tr.steps() {
      if tx.transform().step(step.clone(), snap.schema()).is_err() {
        ok = false;
        break;
      }
    }
    if !ok {
      return;
    }
    snap.apply(tx)
  };
  state.set(next);
}
