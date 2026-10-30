//! Stock actions on the shown dish or variant: go-list, stop-list, portion limit.

use std::rc::Rc;
use std::sync::Arc;

use super::DishesController;
use crate::controllers::background::run_in_background;
use crate::domain::{Dish, DishKind, DishUpdate, PortionLimit};
use crate::repository::{MealRepository, RepositoryError};
use crate::ui::MealsError;

impl DishesController {
    /// Switches flip at once (legacy did the same) and are rolled back if
    /// the backend refuses.
    pub fn toggle_go_list(self: &Rc<Self>, enabled: bool) {
        let Some(shown) = self.state.borrow().shown_eatable().cloned() else {
            return;
        };
        let update = DishUpdate::go_list(&shown, enabled);
        self.send_optimistic_update(shown, update);
    }

    pub fn toggle_stop_list(self: &Rc<Self>, enabled: bool) {
        let Some(shown) = self.state.borrow().shown_eatable().cloned() else {
            return;
        };
        let update = DishUpdate::stop_list(&shown, enabled);
        self.send_optimistic_update(shown, update);
    }

    /// Unlike the switches, the limit form waits for the backend and only
    /// closes on success; on failure it stays open with what was typed.
    pub fn save_portion_limit(self: &Rc<Self>, unlimited: bool, count_text: &str) {
        let Some(shown) = self.state.borrow().shown_eatable().cloned() else {
            return;
        };
        self.view.show_error(MealsError::None);
        match PortionLimit::from_form(unlimited, count_text) {
            Some(limit) => self.send_limit_update(shown, DishUpdate::portion_limit(limit)),
            None => self.view.show_error(MealsError::InvalidLimit),
        }
    }

    /// The switches stay disabled until the reply, so replies cannot overtake
    /// each other and roll back to an outdated state.
    fn send_optimistic_update(self: &Rc<Self>, original: Dish, update: DishUpdate) {
        let mut optimistic = original.clone();
        optimistic.apply(&update);
        self.store_eatable(&optimistic);
        self.view.show_error(MealsError::None);
        self.view.show_saving_stock(true);

        let repository = Arc::clone(&self.repository);
        let controller = Rc::clone(self);
        let target = original.clone();
        run_in_background(
            move || save_eatable(repository, &target, &update),
            move |result| {
                controller.view.show_saving_stock(false);
                match result {
                    Ok(saved) => controller.store_eatable(&saved),
                    Err(error) => {
                        eprintln!("could not update {}: {error}", original.id);
                        controller.view.show_error(MealsError::SaveFailed);
                        controller.store_eatable(&original);
                    }
                }
            },
        );
    }

    fn send_limit_update(self: &Rc<Self>, target: Dish, update: DishUpdate) {
        self.view.show_saving_stock(true);
        let repository = Arc::clone(&self.repository);
        let controller = Rc::clone(self);
        run_in_background(
            move || save_eatable(repository, &target, &update),
            move |result| {
                controller.view.show_saving_stock(false);
                match result {
                    Ok(saved) => {
                        controller.store_eatable(&saved);
                        // Another dish may have been opened while saving.
                        let still_shown = controller
                            .state
                            .borrow()
                            .shown_eatable()
                            .is_some_and(|shown| shown.id == saved.id);
                        if still_shown {
                            controller.view.close_limit_editor();
                        }
                    }
                    Err(error) => {
                        eprintln!("could not update dish limit: {error}");
                        controller.view.show_error(MealsError::SaveFailed);
                    }
                }
            },
        );
    }

    fn store_eatable(&self, dish: &Dish) {
        self.state.borrow_mut().replace_eatable(dish);
        let state = self.state.borrow();
        for listed in state.dishes.iter().chain(state.variants.iter()) {
            if listed.id == dish.id {
                self.view.show_eatable_card(listed);
            }
        }
        self.view.show_selection(&state);
    }
}

/// Dishes and variants are saved through different repository calls.
fn save_eatable(
    repository: Arc<dyn MealRepository>,
    target: &Dish,
    update: &DishUpdate,
) -> Result<Dish, RepositoryError> {
    match target.kind {
        DishKind::Dish => repository.update_dish(&target.id, update),
        DishKind::Variant => repository.update_variant(&target.id, update),
    }
}
