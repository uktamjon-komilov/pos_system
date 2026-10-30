# pos-ui architecture

## Folder map

```
Cargo.toml                 workspace (one member today: pos-ui)
pos-ui/
  build.rs                 compiles ui/app-window.slint
  assets/fonts, icons/     Inter TTF, SVG icons (Gravity + legacy in-house)
  src/
    main.rs                startup only
    app.rs                 builds window, picks the repository, wires controllers
    ui.rs                  Rust types generated from Slint
    domain/                plain structs + rules (Dish, Category, Modifier, PortionLimit, ...)
    repository/            trait MealRepository; mock/ = in-memory implementation + seed data
    controllers/           meals_controller (tabs, search, back/home)
                           dishes/{browsing,stock,comments,state,view}.rs
                           modifiers/{mod,state,view}.rs, background.rs (worker thread → UI thread)
    mappers.rs             domain → Slint display structs, number formatting
  ui/
    app-window.slint       window, fonts, sidebar + section switch, exports for Rust
    theme.slint            ALL colors, sizes, radii, fonts (logical px; Slint applies OS display scale)
    strings.slint          ALL user-facing text, wrapped in @tr()
    components/            generic widgets: Button, Card, TextField, Toggle, ...
    shell/                 Sidebar, SidebarItem, SectionFrame
    sections/              section-placeholder + meals/ (stores, pages, cards, forms)
```

## Data flow: click → repository → screen

1. A card calls `DishesStore.dish-clicked(id)` (a Slint global callback).
2. `DishesController::select_dish` (Rust, UI thread) updates `DishesState`.
3. `run_in_background` calls the repository on a worker thread, so the UI
   never blocks.
4. The result returns to the UI thread (`slint::spawn_local` + oneshot).
   The controller stores it, drops it if a newer request has superseded it
   (`*_generation` counters), and calls `DishesView`.
5. `DishesView` maps domain → Slint structs (`mappers.rs`) and updates the
   `VecModel`s / store properties. Slint re-renders through its bindings.

Slint never holds business logic: it shows store properties and forwards
events. Rules such as "go-list and stop-list exclude each other" live in
`domain` (`DishUpdate::go_list`).

## Adding a reusable component

Create `ui/components/<name>.slint` with one exported component. It takes
`in` properties and callbacks only, and takes every value from `Theme`. If it
must fill a free-form parent (not a layout), inherit `Rectangle`. Plain
components shrink to their preferred size there.

## Adding a section

1. Write `ui/sections/<name>.slint` (or a folder for a large section).
2. Add the value to `Section` (`shell/sidebar-item.slint`). In
   `app-window.slint`, add a `nav-items` entry with that `section` and an
   `if root.active-section == Section.<name>: ...` line. Put the label in
   `strings.slint`.
3. If it needs data: add a `<name>-store.slint` global, export it from
   `app-window.slint`, and add a controller under `src/controllers/` that is
   wired in `app.rs`.

## Swapping the mock for a real API client

Implement `repository::MealRepository` in a new module
`src/repository/api/` inside pos-ui (for example `ApiMealRepository`). The
trait and the domain types live in the pos-ui binary crate, so the client
cannot be a separate crate unless they first move into a library crate.
Methods may block, because they always run on worker threads. Then change
the one line in `app.rs` that constructs `MockMealRepository`. The client
maps backend JSON into the domain types and must invert the backend's
`is_limited` flag into `PortionLimit::Unlimited`. Nothing in controllers or
Slint changes.
