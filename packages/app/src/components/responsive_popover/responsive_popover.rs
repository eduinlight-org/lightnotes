use super::use_responsive_popover::{use_responsive_popover, ResponsivePopoverCtx};
use dioxus::prelude::*;
use ui::components::popover::{ContentAlign, PopoverContent, PopoverRoot, PopoverTrigger};
use ui::components::sheet::{Sheet, SheetDescription, SheetHeader, SheetTitle};

#[derive(PartialEq, Clone, Props)]
pub struct ResponsivePopoverRootProps {
  pub open: ReadSignal<Option<bool>>,
  #[props(default)]
  pub default_open: bool,
  #[props(default)]
  pub on_open_change: Callback<bool>,
  pub children: Element,
}

#[component]
pub fn ResponsivePopoverRoot(props: ResponsivePopoverRootProps) -> Element {
  let ResponsivePopoverRootProps { open, default_open, on_open_change, children } = props;
  let ctx = use_responsive_popover(open, default_open, on_open_change);
  use_context_provider(|| ctx);

  if (ctx.is_mobile)() {
    rsx! {
      {children}
    }
  } else {
    rsx! {
      PopoverRoot {
        open: (ctx.open)(),
        on_open_change: move |value| ctx.set_open.call(value),
        {children}
      }
    }
  }
}

#[derive(PartialEq, Clone, Props)]
pub struct ResponsivePopoverTriggerProps {
  #[props(extends = GlobalAttributes)]
  pub attributes: Vec<Attribute>,
  pub children: Element,
}

#[component]
pub fn ResponsivePopoverTrigger(props: ResponsivePopoverTriggerProps) -> Element {
  let ResponsivePopoverTriggerProps { attributes, children } = props;
  let ctx: ResponsivePopoverCtx = use_context();

  if (ctx.is_mobile)() {
    rsx! {
      button { onclick: move |_| ctx.set_open.call(true), ..attributes, {children} }
    }
  } else {
    rsx! {
      PopoverTrigger { attributes, {children} }
    }
  }
}

#[derive(PartialEq, Clone, Props)]
pub struct ResponsivePopoverContentProps {
  pub title: String,
  #[props(default)]
  pub description: Option<String>,
  #[props(default = ContentAlign::Center)]
  pub align: ContentAlign,
  #[props(default)]
  pub class: Option<String>,
  pub children: Element,
}

#[component]
pub fn ResponsivePopoverContent(props: ResponsivePopoverContentProps) -> Element {
  let ResponsivePopoverContentProps { title, description, align, class, children } = props;
  let ctx: ResponsivePopoverCtx = use_context();

  if (ctx.is_mobile)() {
    rsx! {
      Sheet {
        open: (ctx.open)(),
        on_open_change: move |value| ctx.set_open.call(value),
        "data-side": "bottom",
        class: "max-h-[80vh] overflow-y-auto",
        SheetHeader { class: "sr-only",
          SheetTitle { "{title}" }
          SheetDescription { "{description.clone().unwrap_or_else(|| title.clone())}" }
        }
        div { class: "flex flex-col gap-1.5 p-3 pb-[calc(1rem+env(safe-area-inset-bottom))]", {children} }
      }
    }
  } else {
    rsx! {
      PopoverContent { align, class, {children} }
    }
  }
}
