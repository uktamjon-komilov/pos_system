//! Pushes the modifiers tab state into the `ModifiersStore` Slint global.

use std::rc::Rc;

use slint::{ComponentHandle, ModelRc, VecModel, Weak};

use super::state::ModifiersState;
use crate::mappers;
use crate::ui::{AppWindow, MealsError, ModifierCardData, ModifiersStore};

pub struct ModifiersView {
    window: Weak<AppWindow>,
    modifiers: Rc<VecModel<ModifierCardData>>,
}

impl ModifiersView {
    pub fn new(window: &AppWindow) -> ModifiersView {
        let view = ModifiersView {
            window: window.as_weak(),
            modifiers: Rc::new(VecModel::default()),
        };
        window
            .global::<ModifiersStore>()
            .set_modifiers(ModelRc::from(view.modifiers.clone()));
        view
    }

    fn with_store(&self, update: impl FnOnce(ModifiersStore<'_>)) {
        if let Some(window) = self.window.upgrade() {
            update(window.global::<ModifiersStore>());
        }
    }

    pub fn show(&self, state: &ModifiersState) {
        self.modifiers.set_vec(
            state
                .modifiers
                .iter()
                .map(mappers::modifier_card)
                .collect::<Vec<_>>(),
        );
        let selected = state.selected();
        let details = selected.map(mappers::modifier_details).unwrap_or_default();
        self.with_store(|store| {
            store.set_is_loading(state.is_loading);
            store.set_has_selection(selected.is_some());
            store.set_selected_id(state.selected_id.clone().unwrap_or_default().into());
            store.set_details(details);
        });
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
