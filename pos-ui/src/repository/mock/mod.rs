//! In-memory `MealRepository` used until the real API client exists.
//!
//! It sleeps a little on every call so that loading states are visible and
//! the UI is exercised the same way a network would exercise it.

mod data;
mod seed;
mod seed_dishes;

use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

use super::{MealRepository, RepositoryError};
use crate::domain::{
    CatalogPage, CatalogQuery, CommentTemplate, Dish, DishKind, DishUpdate, Modifier,
    ModifierUpdate,
};
use data::MockData;

/// Long enough to see spinners, short enough not to annoy while clicking around.
const SIMULATED_LATENCY: Duration = Duration::from_millis(150);

pub struct MockMealRepository {
    data: Mutex<MockData>,
}

impl MockMealRepository {
    pub fn with_sample_data() -> MockMealRepository {
        MockMealRepository {
            data: Mutex::new(seed::sample_data()),
        }
    }

    /// Simulates the round trip, then locks the data. A poisoned lock only
    /// means another worker panicked mid-update; the data is still usable.
    fn data_after_latency(&self) -> MutexGuard<'_, MockData> {
        thread::sleep(SIMULATED_LATENCY);
        self.data
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl MealRepository for MockMealRepository {
    fn catalog(&self, query: &CatalogQuery) -> Result<CatalogPage, RepositoryError> {
        Ok(self.data_after_latency().catalog_page(query))
    }

    fn dish_variants(&self, dish_id: &str) -> Result<Vec<Dish>, RepositoryError> {
        Ok(self.data_after_latency().variants_of(dish_id))
    }

    fn update_dish(&self, dish_id: &str, update: &DishUpdate) -> Result<Dish, RepositoryError> {
        self.data_after_latency()
            .update_dish(dish_id, DishKind::Dish, update)
    }

    fn update_variant(
        &self,
        variant_id: &str,
        update: &DishUpdate,
    ) -> Result<Dish, RepositoryError> {
        self.data_after_latency()
            .update_dish(variant_id, DishKind::Variant, update)
    }

    fn comment_templates(&self, dish_id: &str) -> Result<Vec<CommentTemplate>, RepositoryError> {
        Ok(self.data_after_latency().comments_of(dish_id))
    }

    fn create_comment_template(
        &self,
        dish_id: &str,
        text: &str,
    ) -> Result<CommentTemplate, RepositoryError> {
        self.data_after_latency().add_comment(dish_id, text)
    }

    fn delete_comment_template(&self, comment_id: &str) -> Result<(), RepositoryError> {
        self.data_after_latency().delete_comment(comment_id)
    }

    fn modifiers(&self, search_text: &str) -> Result<Vec<Modifier>, RepositoryError> {
        Ok(self.data_after_latency().modifiers_matching(search_text))
    }

    fn update_modifier(
        &self,
        modifier_id: &str,
        update: &ModifierUpdate,
    ) -> Result<Modifier, RepositoryError> {
        self.data_after_latency()
            .update_modifier(modifier_id, update)
    }
}
