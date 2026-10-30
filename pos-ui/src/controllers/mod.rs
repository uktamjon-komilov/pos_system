//! Glue between Slint callbacks and the repository.
//!
//! A controller receives a UI event, updates its small state struct, asks the
//! repository for data on a worker thread, and pushes display structs back
//! to the Slint globals. Business rules stay in `domain`, wording in Slint.

mod background;
mod dishes;
mod meals_controller;
mod modifiers;

pub use meals_controller::MealsController;
