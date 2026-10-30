//! Controller of the dishes tab. Split by responsibility:
//! `browsing` (lists, categories, selection), `stock` (go/stop-list, limits),
//! `comments` (comment templates). `state` holds the data, `view` shows it.

mod browsing;
mod comments;
mod state;
mod stock;
mod view;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use slint::ComponentHandle;

use crate::repository::MealRepository;
use crate::ui::{AppWindow, DishesStore};
use state::DishesState;
use view::DishesView;

pub struct DishesController {
    repository: Arc<dyn MealRepository>,
    state: RefCell<DishesState>,
    view: DishesView,
}

impl DishesController {
    pub fn new(window: &AppWindow, repository: Arc<dyn MealRepository>) -> Rc<DishesController> {
        let controller = Rc::new(DishesController {
            repository,
            state: RefCell::new(DishesState::default()),
            view: DishesView::new(window),
        });
        controller.connect_callbacks(window);
        controller
    }

    fn connect_callbacks(self: &Rc<Self>, window: &AppWindow) {
        let store = window.global::<DishesStore>();

        let controller = Rc::clone(self);
        store.on_category_clicked(move |category_id| controller.open_category(&category_id));

        let controller = Rc::clone(self);
        store.on_dish_clicked(move |dish_id| controller.select_dish(&dish_id));

        let controller = Rc::clone(self);
        store.on_variant_clicked(move |variant_id| controller.select_variant(&variant_id));

        let controller = Rc::clone(self);
        store.on_next_page_requested(move || controller.load_next_page());

        let controller = Rc::clone(self);
        store.on_go_list_toggled(move |enabled| controller.toggle_go_list(enabled));

        let controller = Rc::clone(self);
        store.on_stop_list_toggled(move |enabled| controller.toggle_stop_list(enabled));

        let controller = Rc::clone(self);
        store.on_limit_saved(move |unlimited, count_text| {
            controller.save_portion_limit(unlimited, &count_text)
        });

        let controller = Rc::clone(self);
        store.on_comment_added(move |text| controller.add_comment(&text));

        let controller = Rc::clone(self);
        store.on_comment_deleted(move |comment_id| controller.delete_comment(&comment_id));
    }
}
