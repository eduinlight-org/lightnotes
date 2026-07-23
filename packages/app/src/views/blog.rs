use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Blog(id: i32) -> Element {
  rsx! {
      div {
          id: "blog",
          class: "mt-[50px]",

          // Content

          h1 {
            class: "bg-red-200",
            "This is blog #{id}!"
          }

          p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

          // Navigation links
          Link {
              to: Route::Blog { id: id - 1 },
              class: "text-white mt-[50px]",
              "Previous"
          }
          span { " <---> " }
          Link {
              to: Route::Blog { id: id + 1 },
              class: "text-white mt-[50px]",
              "Next"
          }
      }
  }
}
