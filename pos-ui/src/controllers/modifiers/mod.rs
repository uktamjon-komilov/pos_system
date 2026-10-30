//! Controller of the modifiers tab: list, selection, stop-list and limit.

mod state;
mod view;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use slint::ComponentHandle;

use super::background::run_in_background;
use crate::domain::{Modifier, ModifierUpdate, PortionLimit};
use crate::repository::{MealRepository, RepositoryError};
use crate::ui::{AppWindow, MealsError, ModifiersStore};
use state::ModifiersState;
use view::ModifiersView;

pub struct ModifiersController {
    repository: Arc<dyn MealRepository>,
    state: RefCell<ModifiersState>,
    view: ModifiersView,
}

impl ModifiersController {
    pub fn new(window: &AppWindow, repository: Arc<dyn MealRepository>) -> Rc<ModifiersController> {
        let controller = Rc::new(ModifiersController {
            repository,
            state: RefCell::new(ModifiersState::default()),
            view: ModifiersView::new(window),
        });
        let store = window.global::<ModifiersStore>();

        let handler = Rc::clone(&controller);
        store.on_modifier_clicked(move |modifier_id| handler.select(&modifier_id));

        let handler = Rc::clone(&controller);
        store.on_stop_list_toggled(move |enabled| handler.toggle_stop_list(enabled));

        let handler = Rc::clone(&controller);
        store.on_limit_saved(move |unlimited, count_text| {
            handler.save_portion_limit(unlimited, &count_text)
        });

        controller
    }

    pub fn reset(self: &Rc<Self>) {
        *self.state.borrow_mut() = ModifiersState::default();
        self.view.close_limit_editor();
        self.load("");
    }

    /// Legacy keeps the selection while searching if the modifier is still listed.
    pub fn load(self: &Rc<Self>, search_text: &str) {
        self.view.show_error(MealsError::None);
        let generation = {
            let mut state = self.state.borrow_mut();
            state.request_generation += 1;
            state.is_loading = true;
            state.request_generation
        };
        self.view.show(&self.state.borrow());

        let repository = Arc::clone(&self.repository);
        let controller = Rc::clone(self);
        let search_text = search_text.to_string();
        run_in_background(
            move || repository.modifiers(&search_text),
            move |result| controller.on_loaded(generation, result),
        );
    }

    fn on_loaded(&self, generation: u64, result: Result<Vec<Modifier>, RepositoryError>) {
        {
            let mut state = self.state.borrow_mut();
            if generation != state.request_generation {
                return;
            }
            state.is_loading = false;
            match result {
                Ok(modifiers) => {
                    state.modifiers = modifiers;
                    if state.selected().is_none() {
                        state.selected_id = None;
                    }
                }
                Err(error) => {
                    eprintln!("could not load modifiers: {error}");
                    self.view.show_error(MealsError::LoadFailed);
                }
            }
        }
        self.view.show(&self.state.borrow());
    }

    fn select(&self, modifier_id: &str) {
        self.view.show_error(MealsError::None);
        self.state.borrow_mut().selected_id = Some(modifier_id.to_string());
        self.view.close_limit_editor();
        self.view.show(&self.state.borrow());
    }

    /// Flips at once like legacy; rolled back if the backend refuses. The
    /// switch stays disabled until the reply, so replies cannot overtake
    /// each other.
    fn toggle_stop_list(self: &Rc<Self>, enabled: bool) {
        let Some(original) = self.state.borrow().selected().cloned() else {
            return;
        };
        let update = ModifierUpdate::stop_list(enabled);
        let mut optimistic = original.clone();
        optimistic.apply(&update);
        self.store(&optimistic);
        self.view.show_error(MealsError::None);
        self.view.show_saving_stock(true);

        let repository = Arc::clone(&self.repository);
        let controller = Rc::clone(self);
        let target_id = original.id.clone();
        run_in_background(
            move || repository.update_modifier(&target_id, &update),
            move |result| {
                controller.view.show_saving_stock(false);
                match result {
                    Ok(saved) => controller.store(&saved),
                    Err(error) => {
                        eprintln!("could not update modifier: {error}");
                        controller.view.show_error(MealsError::SaveFailed);
                        controller.store(&original);
                    }
                }
            },
        );
    }

    /// The form closes only once the backend accepted the new limit.
    fn save_portion_limit(self: &Rc<Self>, unlimited: bool, count_text: &str) {
        let Some(selected_id) = self.state.borrow().selected_id.clone() else {
            return;
        };
        self.view.show_error(MealsError::None);
        let update = match PortionLimit::from_form(unlimited, count_text) {
            Some(limit) => ModifierUpdate::portion_limit(limit),
            None => {
                self.view.show_error(MealsError::InvalidLimit);
                return;
            }
        };
        self.view.show_saving_stock(true);
        let repository = Arc::clone(&self.repository);
        let controller = Rc::clone(self);
        run_in_background(
            move || repository.update_modifier(&selected_id, &update),
            move |result| {
                controller.view.show_saving_stock(false);
                match result {
                    Ok(saved) => {
                        controller.store(&saved);
                        // Another modifier may have been selected while saving.
                        let still_selected = controller.state.borrow().selected_id.as_deref()
                            == Some(saved.id.as_str());
                        if still_selected {
                            controller.view.close_limit_editor();
                        }
                    }
                    Err(error) => {
                        eprintln!("could not update modifier limit: {error}");
                        controller.view.show_error(MealsError::SaveFailed);
                    }
                }
            },
        );
    }

    fn store(&self, modifier: &Modifier) {
        self.state.borrow_mut().replace(modifier);
        self.view.show(&self.state.borrow());
    }
}
