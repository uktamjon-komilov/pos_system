//! Moving through the menu: search, categories, pages, dish and variant selection.

use std::rc::Rc;
use std::sync::Arc;

use super::state::DishesState;
use super::DishesController;
use crate::controllers::background::run_in_background;
use crate::domain::{CatalogPage, CatalogQuery, Dish};
use crate::repository::RepositoryError;
use crate::ui::MealsError;

impl DishesController {
    /// Back to the menu root with nothing selected (legacy `setDefaults`).
    pub fn reset(self: &Rc<Self>) {
        *self.state.borrow_mut() = DishesState::default();
        self.view.show_error(MealsError::None);
        self.view.close_limit_editor();
        self.view.clear_comment_draft();
        self.show_everything();
        self.reload_list();
    }

    /// A new search starts from the menu root, as in legacy.
    pub fn set_search(self: &Rc<Self>, search_text: &str) {
        self.view.show_error(MealsError::None);
        {
            let mut state = self.state.borrow_mut();
            state.search_text = search_text.to_string();
            state.category_stack.clear();
            state.clear_selection();
        }
        self.show_everything();
        self.reload_list();
    }

    pub fn open_category(self: &Rc<Self>, category_id: &str) {
        self.view.show_error(MealsError::None);
        {
            let mut state = self.state.borrow_mut();
            let Some(category) = state
                .sub_categories
                .iter()
                .find(|category| category.id == category_id)
                .cloned()
            else {
                return;
            };
            state.category_stack.push(category);
            state.clear_selection();
        }
        self.show_everything();
        self.reload_list();
    }

    /// Leaves the variants list if one is open, otherwise goes up one category.
    pub fn navigate_back(self: &Rc<Self>) {
        self.view.show_error(MealsError::None);
        let leaves_variants = {
            let mut state = self.state.borrow_mut();
            let leaves_variants = state
                .selected_dish
                .as_ref()
                .is_some_and(|dish| dish.has_variants());
            if !leaves_variants {
                state.category_stack.pop();
            }
            state.clear_selection();
            leaves_variants
        };
        self.show_everything();
        if !leaves_variants {
            self.reload_list();
        }
    }

    pub fn select_dish(self: &Rc<Self>, dish_id: &str) {
        self.view.show_error(MealsError::None);
        let selected = {
            let mut state = self.state.borrow_mut();
            let Some(dish) = state.dishes.iter().find(|dish| dish.id == dish_id).cloned() else {
                return;
            };
            state.clear_selection();
            state.selected_dish = Some(dish.clone());
            dish
        };
        self.view.close_limit_editor();
        self.show_everything();
        if selected.has_variants() {
            self.load_variants(&selected);
        }
        self.load_comments(&selected.id);
    }

    pub fn select_variant(self: &Rc<Self>, variant_id: &str) {
        self.view.show_error(MealsError::None);
        {
            let mut state = self.state.borrow_mut();
            let is_parent_dish = state
                .selected_dish
                .as_ref()
                .is_some_and(|dish| dish.id == variant_id);
            let variant = state
                .variants
                .iter()
                .find(|variant| variant.id == variant_id)
                .cloned();
            state.selected_variant = if is_parent_dish { None } else { variant };
        }
        self.view.close_limit_editor();
        self.view.show_selection(&self.state.borrow());
    }

    pub fn load_next_page(self: &Rc<Self>) {
        let next_page = {
            let mut state = self.state.borrow_mut();
            if !state.has_next_page || state.is_loading_next_page || state.is_loading_first_page {
                return;
            }
            state.is_loading_next_page = true;
            state.loaded_pages + 1
        };
        self.view.show_list(&self.state.borrow());
        self.request_catalog_page(next_page);
    }

    /// Drops the loaded pages and fetches page 1 of the current list.
    fn reload_list(self: &Rc<Self>) {
        {
            let mut state = self.state.borrow_mut();
            state.catalog_generation += 1;
            state.sub_categories.clear();
            state.dishes.clear();
            state.loaded_pages = 0;
            state.has_next_page = false;
            state.is_loading_next_page = false;
            state.is_loading_first_page = true;
        }
        self.view.show_list(&self.state.borrow());
        self.view.scroll_list_to_top();
        self.request_catalog_page(1);
    }

    fn request_catalog_page(self: &Rc<Self>, page: u32) {
        let (query, generation) = {
            let state = self.state.borrow();
            let query = CatalogQuery {
                category_id: state.current_category().map(|category| category.id.clone()),
                search_text: state.search_text.clone(),
                page,
            };
            (query, state.catalog_generation)
        };
        let repository = Arc::clone(&self.repository);
        let controller = Rc::clone(self);
        run_in_background(
            move || repository.catalog(&query),
            move |result| controller.on_catalog_page(generation, page, result),
        );
    }

    fn on_catalog_page(
        &self,
        generation: u64,
        page: u32,
        result: Result<CatalogPage, RepositoryError>,
    ) {
        {
            let mut state = self.state.borrow_mut();
            if generation != state.catalog_generation {
                return;
            }
            state.is_loading_first_page = false;
            state.is_loading_next_page = false;
            match result {
                Ok(catalog_page) => {
                    // Sub-categories come from page 1 only, like legacy.
                    if page == 1 {
                        state.sub_categories = catalog_page.sub_categories;
                    }
                    state.dishes.extend(catalog_page.dishes);
                    state.loaded_pages = page;
                    state.has_next_page = catalog_page.has_next_page;
                }
                // The list stays as it was and an alert explains why
                // (legacy shows nothing here).
                Err(error) => {
                    eprintln!("could not load catalog page {page}: {error}");
                    self.view.show_error(MealsError::LoadFailed);
                }
            }
        }
        self.view.show_list(&self.state.borrow());
    }

    fn load_variants(self: &Rc<Self>, dish: &Dish) {
        let generation = self.state.borrow().selection_generation;
        let repository = Arc::clone(&self.repository);
        let controller = Rc::clone(self);
        let dish_id = dish.id.clone();
        run_in_background(
            move || repository.dish_variants(&dish_id),
            move |result| controller.on_variants(generation, result),
        );
    }

    fn on_variants(&self, generation: u64, result: Result<Vec<Dish>, RepositoryError>) {
        {
            let mut state = self.state.borrow_mut();
            if generation != state.selection_generation {
                return;
            }
            let Some(parent) = state.selected_dish.clone() else {
                return;
            };
            match result {
                Ok(variants) => {
                    let merged = variants
                        .iter()
                        .map(|variant| variant.merged_with_parent(&parent));
                    state.variants = std::iter::once(parent.clone()).chain(merged).collect();
                }
                Err(error) => {
                    eprintln!("could not load variants of {}: {error}", parent.id);
                    self.view.show_error(MealsError::LoadFailed);
                }
            }
        }
        self.show_everything();
    }

    pub(super) fn show_everything(&self) {
        let state = self.state.borrow();
        self.view.show_list(&state);
        self.view.show_selection(&state);
        self.view.show_comments(&state);
    }
}
