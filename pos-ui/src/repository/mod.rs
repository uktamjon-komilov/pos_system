//! Data access boundary for the meals section.
//!
//! The UI talks only to `MealRepository`. Today the implementation is
//! `MockMealRepository` (in memory); a real HTTP client implements the same
//! trait and is swapped in `app.rs` without touching controllers or Slint.
//!
//! Methods are blocking on purpose: controllers always call them from a
//! worker thread (see `controllers::background`), so an HTTP client can use a
//! plain blocking request and the UI thread never waits.

mod error;
mod mock;

pub use error::RepositoryError;
pub use mock::MockMealRepository;

use crate::domain::{
    CatalogPage, CatalogQuery, CommentTemplate, Dish, DishUpdate, Modifier, ModifierUpdate,
};

pub trait MealRepository: Send + Sync {
    fn catalog(&self, query: &CatalogQuery) -> Result<CatalogPage, RepositoryError>;

    /// Variants (sizes) of a dish, without the dish itself.
    fn dish_variants(&self, dish_id: &str) -> Result<Vec<Dish>, RepositoryError>;

    /// Dishes and variants live behind different backend endpoints, hence two methods.
    fn update_dish(&self, dish_id: &str, update: &DishUpdate) -> Result<Dish, RepositoryError>;
    fn update_variant(
        &self,
        variant_id: &str,
        update: &DishUpdate,
    ) -> Result<Dish, RepositoryError>;

    fn comment_templates(&self, dish_id: &str) -> Result<Vec<CommentTemplate>, RepositoryError>;
    fn create_comment_template(
        &self,
        dish_id: &str,
        text: &str,
    ) -> Result<CommentTemplate, RepositoryError>;
    fn delete_comment_template(&self, comment_id: &str) -> Result<(), RepositoryError>;

    fn modifiers(&self, search_text: &str) -> Result<Vec<Modifier>, RepositoryError>;
    fn update_modifier(
        &self,
        modifier_id: &str,
        update: &ModifierUpdate,
    ) -> Result<Modifier, RepositoryError>;
}
