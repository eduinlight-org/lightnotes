//! This crate contains presentational components with no ties to a specific application.
//! App-specific compositions (things that reference this app's routes, brand or copy) live in
//! `packages/app/src/components` instead.

use dioxus::prelude::*;

pub mod components;

/// Theme variables consumed by every component in [`components`]. Link this once in each
/// platform's document head.
pub const THEME_CSS: Asset = asset!("/assets/dx-components-theme.css", AssetOptions::css().with_static_head(true));
