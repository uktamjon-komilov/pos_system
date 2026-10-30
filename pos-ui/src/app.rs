//! Builds the window and connects it to its controllers and data source.
//! The one place that decides which `MealRepository` the app uses.

use std::sync::Arc;

use slint::{ComponentHandle, PlatformError};

use crate::controllers::MealsController;
use crate::repository::{MealRepository, MockMealRepository};
use crate::ui::{AppWindow, TextUtils};

pub fn run() -> Result<(), PlatformError> {
    let window = AppWindow::new()?;
    // Legacy runs full screen in production; maximized is the windowed equivalent.
    window.window().set_maximized(true);
    install_text_utils(&window);

    // Swap this line for the real API client once it exists.
    let repository: Arc<dyn MealRepository> = Arc::new(MockMealRepository::with_sample_data());
    MealsController::new(&window, repository);

    window.run()
}

fn install_text_utils(window: &AppWindow) {
    let text_utils = window.global::<TextUtils>();
    text_utils.on_drop_last_character(|text| {
        let mut shortened = text.to_string();
        shortened.pop();
        shortened.into()
    });
    text_utils.on_is_blank(|text| text.trim().is_empty());
}
