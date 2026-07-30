use dioxus::prelude::*;

const FEATURES: [(&str, &str, &str); 4] = [
  (
    "01",
    "Notes & organization",
    "Star, pin and delete notes — pinned ones float to the top. Sort them into icon-tagged folders and freeform hashtags, filter by All, Starred, Pinned, folder or tag with live counts, and search titles and content instantly, locally.",
  ),
  (
    "02",
    "A diary over the same notes",
    "Every note carries its own date, independent of when it was written. Day, week and month calendars mark the days that have entries, and an optional reminder — at the time, or up to a week before — shows as a bell in both the diary and the notes list.",
  ),
  (
    "03",
    "Markdown without preview mode",
    "WYSIWYG editing that round-trips cleanly to Markdown. Headings, quotes, code blocks, lists, alignment, case transforms, undo and redo — plus inline links, image embeds and resizable tables with row, column and header controls.",
  ),
  (
    "04",
    "Sync you control",
    "A background engine reconciles local changes against your own Rust API over REST and Server-Sent Events: debounced batching, exponential-backoff reconnects, last-write-wins conflicts. Flip 'go offline' in Settings and nothing breaks.",
  ),
];

#[component]
pub fn FeatureList() -> Element {
  rsx! {
      section {
          id: "features",
          class: "mx-auto max-w-6xl px-6 pt-24 pb-10 sm:pt-28",
          span { class: "block text-xs uppercase tracking-wider text-[#9a8fe0]", "What it does" }
          h2 {
              class: "mt-3 max-w-[24ch] text-[clamp(1.875rem,3.2vw,2.625rem)] font-medium leading-tight tracking-tight text-white",
              "Everything a notes app needs, nothing it doesn't."
          }
          div {
              class: "mt-12",
              for (number , title , description) in FEATURES {
                  div {
                      class: "grid grid-cols-1 gap-4 border-t border-white/10 py-10 first:border-t-0 sm:grid-cols-[minmax(56px,120px)_minmax(0,400px)_minmax(0,1fr)] sm:items-baseline sm:gap-x-10",
                      p { class: "font-medium text-[#9a8fe0] tabular-nums", "{number}" }
                      h3 { class: "text-2xl font-medium leading-tight tracking-tight text-white", "{title}" }
                      p { class: "max-w-[54ch] leading-relaxed text-white/80", "{description}" }
                  }
              }
          }
      }
  }
}
