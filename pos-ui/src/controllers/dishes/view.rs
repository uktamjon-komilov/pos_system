//! Pushes `DishesState` into the `DishesStore` Slint global.
//!
//! The list models are created once and handed to Slint; later updates only
//! replace their contents, so Slint keeps observing the same models.

use std::rc::Rc;

use slint::{ComponentHandle, Model, ModelRc, VecModel, Weak};

use super::state::DishesState;
use crate::domain::Dish;
use crate::mappers;
use crate::ui::{
    AppWindow, CategoryCardData, CommentData, DishCardData, DishesStore, MealsError, MealsStore,
};

pub struct DishesView {
    window: Weak<AppWindow>,
    categories: Rc<VecModel<CategoryCardData>>,
    dishes: Rc<VecModel<DishCardData>>,
    variants: Rc<VecModel<DishCardData>>,
    comments: Rc<VecModel<CommentData>>,
}

impl DishesView {
    pub fn new(window: &AppWindow) -> DishesView {
        let view = DishesView {
            window: window.as_weak(),
            categories: Rc::new(VecModel::default()),
            dishes: Rc::new(VecModel::default()),
            variants: Rc::new(VecModel::default()),
            comments: Rc::new(VecModel::default()),
        };
        let store = window.global::<DishesStore>();
        store.set_categories(ModelRc::from(view.categories.clone()));
        store.set_dishes(ModelRc::from(view.dishes.clone()));
        store.set_variants(ModelRc::from(view.variants.clone()));
        store.set_comments(ModelRc::from(view.comments.clone()));
        view
    }

    /// Runs `update` with the store, unless the window is already gone.
    fn with_store(&self, update: impl FnOnce(DishesStore<'_>)) {
        if let Some(window) = self.window.upgrade() {
            update(window.global::<DishesStore>());
        }
    }

    pub fn show_list(&self, state: &DishesState) {
        self.categories.set_vec(
            state
                .sub_categories
                .iter()
                .map(mappers::category_card)
                .collect::<Vec<_>>(),
        );
        self.dishes.set_vec(
            state
                .dishes
                .iter()
                .map(mappers::dish_card)
                .collect::<Vec<_>>(),
        );
        self.variants.set_vec(
            state
                .variants
                .iter()
                .map(mappers::dish_card)
                .collect::<Vec<_>>(),
        );
        let category_name = state
            .current_category()
            .map(|category| category.name.as_str())
            .unwrap_or_default();
        self.with_store(|store| {
            store.set_category_name(category_name.into());
            store.set_is_loading_first_page(state.is_loading_first_page);
            store.set_has_next_page(state.has_next_page);
            store.set_is_loading_next_page(state.is_loading_next_page);
        });
        if let Some(window) = self.window.upgrade() {
            window
                .global::<MealsStore>()
                .set_can_go_back(state.can_go_back());
        }
    }

    /// Asks the list to scroll back to the top (a different list is shown).
    pub fn scroll_list_to_top(&self) {
        self.with_store(|store| store.set_list_revision(store.get_list_revision() + 1));
    }

    pub fn show_selection(&self, state: &DishesState) {
        let selected_dish_id = state
            .selected_dish
            .as_ref()
            .map(|dish| dish.id.as_str())
            .unwrap_or_default();
        let selected_variant_id = state
            .selected_variant
            .as_ref()
            .map(|variant| variant.id.as_str())
            .unwrap_or(selected_dish_id);
        let details = state
            .shown_eatable()
            .map(mappers::dish_details)
            .unwrap_or_default();
        self.with_store(|store| {
            store.set_has_selection(state.selected_dish.is_some());
            store.set_selected_dish_id(selected_dish_id.into());
            store.set_selected_variant_id(selected_variant_id.into());
            store.set_details(details);
        });
    }

    /// Refreshes the card of one dish or variant without rebuilding the lists.
    pub fn show_eatable_card(&self, dish: &Dish) {
        let card = mappers::dish_card(dish);
        for model in [&self.dishes, &self.variants] {
            for row in 0..model.row_count() {
                if model
                    .row_data(row)
                    .is_some_and(|listed| listed.id == card.id)
                {
                    model.set_row_data(row, card.clone());
                }
            }
        }
    }

    pub fn show_comments(&self, state: &DishesState) {
        self.comments.set_vec(
            state
                .comments
                .iter()
                .map(mappers::comment)
                .collect::<Vec<_>>(),
        );
        self.with_store(|store| store.set_is_saving_comment(state.is_saving_comment));
    }

    pub fn clear_comment_draft(&self) {
        self.with_store(|store| store.set_comment_draft("".into()));
    }

    pub fn close_limit_editor(&self) {
        self.with_store(|store| store.set_is_editing_limit(false));
    }

    pub fn show_saving_stock(&self, is_saving: bool) {
        self.with_store(|store| store.set_is_saving_stock(is_saving));
    }

    /// `MealsError::None` hides the alert.
    pub fn show_error(&self, error: MealsError) {
        self.with_store(|store| store.set_error(error));
    }
}
