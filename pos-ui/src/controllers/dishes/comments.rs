//! Comment templates of the selected dish (variants share the dish's templates).

use std::rc::Rc;
use std::sync::Arc;

use super::DishesController;
use crate::controllers::background::run_in_background;
use crate::domain::CommentTemplate;
use crate::repository::RepositoryError;
use crate::ui::MealsError;

impl DishesController {
    pub(super) fn load_comments(self: &Rc<Self>, dish_id: &str) {
        let generation = self.state.borrow().selection_generation;
        let repository = Arc::clone(&self.repository);
        let controller = Rc::clone(self);
        let dish_id = dish_id.to_string();
        run_in_background(
            move || repository.comment_templates(&dish_id),
            move |result| controller.on_comments(generation, result),
        );
    }

    fn on_comments(&self, generation: u64, result: Result<Vec<CommentTemplate>, RepositoryError>) {
        {
            let mut state = self.state.borrow_mut();
            if generation != state.selection_generation {
                return;
            }
            match result {
                Ok(comments) => state.comments = comments,
                Err(error) => {
                    eprintln!("could not load comment templates: {error}");
                    self.view.show_error(MealsError::LoadFailed);
                }
            }
        }
        self.view.show_comments(&self.state.borrow());
    }

    pub fn add_comment(self: &Rc<Self>, text: &str) {
        self.view.show_error(MealsError::None);
        if !CommentTemplate::is_valid_text(text) {
            return;
        }
        let dish_id = {
            let mut state = self.state.borrow_mut();
            let Some(dish) = state.selected_dish.as_ref() else {
                return;
            };
            let dish_id = dish.id.clone();
            state.is_saving_comment = true;
            dish_id
        };
        self.view.show_comments(&self.state.borrow());

        let repository = Arc::clone(&self.repository);
        let controller = Rc::clone(self);
        let text = text.to_string();
        let saved_for = dish_id.clone();
        run_in_background(
            move || repository.create_comment_template(&saved_for, &text),
            move |result| {
                controller.state.borrow_mut().is_saving_comment = false;
                // Legacy clears the field whether or not saving worked.
                controller.view.clear_comment_draft();
                if let Err(error) = result {
                    eprintln!("could not add comment template: {error}");
                    controller.view.show_error(MealsError::SaveFailed);
                }
                controller.view.show_comments(&controller.state.borrow());
                controller.reload_comments_if_still_selected(&dish_id);
            },
        );
    }

    pub fn delete_comment(self: &Rc<Self>, comment_id: &str) {
        self.view.show_error(MealsError::None);
        let Some(dish_id) = self
            .state
            .borrow()
            .selected_dish
            .as_ref()
            .map(|dish| dish.id.clone())
        else {
            return;
        };
        let repository = Arc::clone(&self.repository);
        let controller = Rc::clone(self);
        let comment_id = comment_id.to_string();
        run_in_background(
            move || repository.delete_comment_template(&comment_id),
            move |result| {
                if let Err(error) = result {
                    eprintln!("could not delete comment template: {error}");
                    controller.view.show_error(MealsError::SaveFailed);
                }
                controller.reload_comments_if_still_selected(&dish_id);
            },
        );
    }

    /// Re-reads the list from the repository, like legacy's query invalidation.
    fn reload_comments_if_still_selected(self: &Rc<Self>, dish_id: &str) {
        let still_selected = self
            .state
            .borrow()
            .selected_dish
            .as_ref()
            .is_some_and(|dish| dish.id == dish_id);
        if still_selected {
            self.load_comments(dish_id);
        }
    }
}
