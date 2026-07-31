use super::use_note_editor::NoteEditorState;
use dioxus::prelude::*;
use dioxus_i18n::t;
use ui::components::button::{Button, ButtonSize, ButtonVariant};
use ui::components::dialog::{Dialog, DialogDescription, DialogTitle};
use ui::components::input::Input;

#[derive(PartialEq, Clone, Props)]
pub(super) struct LinkDialogProps {
  pub editor: NoteEditorState,
}

#[component]
pub(super) fn LinkDialog(props: LinkDialogProps) -> Element {
  let LinkDialogProps { mut editor } = props;

  rsx! {
    Dialog {
      open: (editor.link_dialog_open)(),
      on_open_change: move |value| editor.link_dialog_open.set(value),
      class: "w-[min(90vw,26rem)] flex flex-col gap-3",
      DialogTitle { class: "text-[var(--secondary-color)] font-medium!", {t!("link-dialog-title")} }
      DialogDescription { class: "sr-only", {t!("link-dialog-description")} }
      label { class: "flex flex-col gap-1 text-xs text-[var(--secondary-color-5)]",
        {t!("link-dialog-text")}
        Input {
          class: "border border-[var(--primary-color-6)] bg-[var(--primary-color-1)] text-sm text-[var(--secondary-color)]",
          placeholder: t!("link-dialog-text-placeholder"),
          value: (editor.link_text_draft)(),
          oninput: move |event: FormEvent| editor.link_text_draft.set(event.value()),
        }
      }
      label { class: "flex flex-col gap-1 text-xs text-[var(--secondary-color-5)]",
        {t!("link-dialog-url")}
        Input {
          class: "border border-[var(--primary-color-6)] bg-[var(--primary-color-1)] text-sm text-[var(--secondary-color)]",
          placeholder: "https://…",
          value: (editor.link_url_draft)(),
          oninput: move |event: FormEvent| editor.link_url_draft.set(event.value()),
          onkeydown: move |event: KeyboardEvent| {
            if event.key() == Key::Enter {
              editor.submit_link_dialog();
            }
          },
        }
      }
      div { class: "flex justify-end gap-2 pt-1",
        Button {
          variant: ButtonVariant::Secondary,
          size: ButtonSize::Sm,
          onclick: move |_| editor.close_link_dialog(),
          {t!("action-cancel")}
        }
        Button {
          variant: ButtonVariant::Primary,
          size: ButtonSize::Sm,
          onclick: move |_| editor.submit_link_dialog(),
          {t!("link-dialog-submit")}
        }
      }
    }
  }
}
