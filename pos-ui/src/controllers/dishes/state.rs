//! Everything the dishes tab remembers between UI events.

use crate::domain::{Category, CommentTemplate, Dish, DishKind};

#[derive(Default)]
pub struct DishesState {
    pub search_text: String,
    /// Categories opened so far; the last one is the current category.
    pub category_stack: Vec<Category>,
    pub sub_categories: Vec<Category>,
    /// Dishes of every page loaded so far, in order.
    pub dishes: Vec<Dish>,
    pub loaded_pages: u32,
    pub has_next_page: bool,
    pub is_loading_first_page: bool,
    pub is_loading_next_page: bool,
    /// Bumped for every new list; responses for older lists are dropped.
    pub catalog_generation: u64,

    pub selected_dish: Option<Dish>,
    /// The open dish followed by its variants (merged with the dish), the
    /// way legacy lists them. Empty when the dish has no variants.
    pub variants: Vec<Dish>,
    /// `None` while the dish itself is chosen in the variants list.
    pub selected_variant: Option<Dish>,
    /// Bumped on every selection change; late variant / comment responses
    /// for a previous selection are dropped.
    pub selection_generation: u64,

    pub comments: Vec<CommentTemplate>,
    pub is_saving_comment: bool,
}

impl DishesState {
    pub fn current_category(&self) -> Option<&Category> {
        self.category_stack.last()
    }

    /// Back goes out of the variants list first, then up one category.
    pub fn can_go_back(&self) -> bool {
        let has_open_variants = self
            .selected_dish
            .as_ref()
            .is_some_and(|dish| dish.has_variants());
        !self.category_stack.is_empty() || has_open_variants
    }

    /// The dish or variant shown (and edited) in the details column.
    pub fn shown_eatable(&self) -> Option<&Dish> {
        self.selected_variant
            .as_ref()
            .or(self.selected_dish.as_ref())
    }

    pub fn clear_selection(&mut self) {
        self.selected_dish = None;
        self.selected_variant = None;
        self.variants.clear();
        self.comments.clear();
        self.selection_generation += 1;
    }

    /// Stores a fresh copy of a dish or variant wherever it is listed.
    pub fn replace_eatable(&mut self, updated: &Dish) {
        // Variants are kept merged with their parent dish (see `variants`).
        let fresh = match (updated.kind, &self.selected_dish) {
            (DishKind::Variant, Some(parent)) => updated.merged_with_parent(parent),
            _ => updated.clone(),
        };
        for listed in self.dishes.iter_mut().chain(self.variants.iter_mut()) {
            if listed.id == fresh.id {
                *listed = fresh.clone();
            }
        }
        let selections = [&mut self.selected_dish, &mut self.selected_variant];
        for selection in selections.into_iter().flatten() {
            if selection.id == fresh.id {
                *selection = fresh.clone();
            }
        }
    }
}
