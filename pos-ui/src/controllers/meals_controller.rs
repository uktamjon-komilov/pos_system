//! Owns the two tab controllers and handles what both tabs share: opening
//! the section, switching tabs, search, back and home.

use std::rc::Rc;
use std::sync::Arc;

use slint::{ComponentHandle, Weak};

use super::dishes::DishesController;
use super::modifiers::ModifiersController;
use crate::repository::MealRepository;
use crate::ui::{AppWindow, MealsStore, MealsTab};

pub struct MealsController {
    window: Weak<AppWindow>,
    dishes: Rc<DishesController>,
    modifiers: Rc<ModifiersController>,
}

impl MealsController {
    pub fn new(window: &AppWindow, repository: Arc<dyn MealRepository>) -> Rc<MealsController> {
        let controller = Rc::new(MealsController {
            window: window.as_weak(),
            dishes: DishesController::new(window, Arc::clone(&repository)),
            modifiers: ModifiersController::new(window, repository),
        });
        let store = window.global::<MealsStore>();

        let handler = Rc::clone(&controller);
        store.on_opened(move || handler.open_tab(MealsTab::Dishes));

        let handler = Rc::clone(&controller);
        store.on_tab_selected(move |tab| handler.open_tab(tab));

        let handler = Rc::clone(&controller);
        store.on_search_edited(move |text| handler.search(&text));

        let handler = Rc::clone(&controller);
        store.on_back_clicked(move || handler.dishes.navigate_back());

        let handler = Rc::clone(&controller);
        store.on_home_clicked(move || handler.go_home());

        controller
    }

    /// Legacy resets search, category path and selection whenever a tab is
    /// (re)entered, then loads only that tab's data.
    pub fn open_tab(&self, tab: MealsTab) {
        self.with_store(|store| {
            store.set_active_tab(tab);
            store.set_search_text("".into());
        });
        match tab {
            MealsTab::Dishes => self.dishes.reset(),
            MealsTab::Modifiers => self.modifiers.reset(),
        }
    }

    fn search(&self, text: &str) {
        match self.active_tab() {
            MealsTab::Dishes => self.dishes.set_search(text),
            MealsTab::Modifiers => self.modifiers.load(text),
        }
    }

    /// On the modifiers tab legacy only clears the search; the selected
    /// modifier is local to that screen and survives.
    fn go_home(&self) {
        self.with_store(|store| store.set_search_text("".into()));
        match self.active_tab() {
            MealsTab::Dishes => self.dishes.reset(),
            MealsTab::Modifiers => self.modifiers.load(""),
        }
    }

    fn active_tab(&self) -> MealsTab {
        self.window
            .upgrade()
            .map(|window| window.global::<MealsStore>().get_active_tab())
            .unwrap_or(MealsTab::Dishes)
    }

    fn with_store(&self, update: impl FnOnce(MealsStore<'_>)) {
        if let Some(window) = self.window.upgrade() {
            update(window.global::<MealsStore>());
        }
    }
}
