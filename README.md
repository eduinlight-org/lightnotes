# Development

Your new workspace contains a member crate for each of the web, desktop and mobile platforms, an `app` crate for the shared routes/pages, a `ui` crate for shared presentational components, and an `api` crate for shared backend logic:

```
your_project/
├─ README.md
├─ Cargo.toml
├─ apps/
│  ├─ web/
│  │  └─ ... # Web specific launch config/assets
│  ├─ desktop/
│  │  └─ ... # Desktop specific launch config/assets
│  ├─ mobile/
│  │  └─ ... # Mobile specific launch config/assets
│  └─ api/
│     └─ ... # All shared server logic
└─ packages/
   ├─ app/
   │  └─ ... # Shared routes, pages, and app-specific components used by every platform
   └─ ui/
      └─ ... # Generic, app-agnostic presentational components shared between multiple platforms
```

## Platform crates

Each platform crate is a thin binary: it wires up the platform-specific renderer feature and assets (favicon, stylesheets) and hands off routing to the shared `app` crate. For example, the desktop crate in the workspace looks something like this:

```
desktop/ # The desktop crate contains platform specific launch config, assets and dependencies for the desktop app
├─ assets/ # Assets used by the desktop app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the desktop app. Links platform assets and mounts app::Route
├─ Cargo.toml # The desktop crate's Cargo.toml - This should include all desktop specific dependencies
```

Since the platform crates start out almost identical, the actual routes, pages and layout live once in `app` rather than being duplicated per platform. As the app grows, a platform can still diverge by defining its own view instead of using the shared one from `app`.

## Shared app crate

The workspace contains an `app` crate with the `Route` enum, the page components (`Home`, `Blog`) and the shared navbar layout used by every platform's router. This is also where **app-specific components** live: anything that is tied to this particular app (references `Route`, hardcodes brand/copy, composes other components into a page section) belongs in `app`, not `ui`, even if it's presentational. The `app` crate starts out something like this:

```
app/
├─ src/
│  ├─ lib.rs # Defines the Route enum and the shared layout, re-exports the views
│  ├─ components/
│  │  ├─ mod.rs # Defines the module for app-specific components and re-exports them
│  │  ├─ app_bar.rs # The sticky header used by the shared layout
│  │  ├─ hero.rs # The Home page hero section
│  │  ├─ sections.rs # The Home page feature sections
│  │  ├─ footer.rs # The shared footer used by the layout
│  │  ├─ login_button.rs # The login button used in the app bar
│  ├─ views/
│  │  ├─ mod.rs # Defines the module for the views and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route
```

## Shared UI crate

The workspace contains a `ui` crate with presentational components that have **no link to a specific application**: no references to `app::Route`, no hardcoded brand/copy, nothing that only makes sense for this one app. Only put a component here if it would make sense to copy-paste into a completely different Dioxus app unchanged. The `ui` crate starts out something like this:

```
ui/
├─ src/
│  ├─ lib.rs # The entrypoint for the ui crate
│  ├─ components/ # A generic, app-agnostic component primitives library (buttons, cards, navbar, dialogs, ...)
│  ├─ echo.rs # The shared echo component that communicates with the server
```

If a component under `ui/src/components` needs app-specific content or behavior (copy, routes, branding), don't edit it in place — compose it from `app/src/components` instead, the same way `app_bar.rs` wraps `ui::components::navbar`.

## Shared backend logic

The workspace contains a `api` crate with shared backend logic. This crate defines all of the shared server functions for all platforms. Server functions are async functions that expose a public API on the server. They can be called like a normal async function from the client. When you run `dx serve`, all of the server functions will be collected in the server build and hosted on a public API for the client to call. The `api` crate starts out something like this:

```
api/
├─ src/
│  ├─ lib.rs # Exports a server function that echos the input string
```

## Styling with Tailwind

Dioxus has built-in support for TailwindCSS: `dx` automatically runs the TailwindCSS CLI whenever it detects a `tailwind.css` file at the root of a platform app. No Node/npm install or separate watch process is needed.

Each platform app (`web`, `desktop`, `mobile`) has its own `tailwind.css` at its crate root, scanning its own `src/` as well as the shared `packages/app/src/` and `packages/ui/src/` for class names:

```css
@import "tailwindcss";
@source "./src/**/*.{rs,html,css}";
@source "../../packages/app/src/**/*.{rs,html,css}";
@source "../../packages/ui/src/**/*.{rs,html,css}";
```

`dx build`/`dx serve` compile it into `assets/tailwind.css`, which is linked in `main.rs` like any other asset. The generated `assets/tailwind.css` is a build artifact and is not committed.

### Serving Your App

Navigate to the platform crate of your choice:
```bash
cd apps/web
```

and serve:

```bash
dx serve
```

