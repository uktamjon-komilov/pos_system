//! The mock's tables and the queries the backend would run on them.

use crate::domain::{
    CatalogPage, CatalogQuery, Category, CommentTemplate, Dish, DishKind, DishUpdate, Modifier,
    ModifierUpdate, CATALOG_PAGE_SIZE,
};
use crate::repository::RepositoryError;

pub struct StoredComment {
    pub dish_id: String,
    pub comment: CommentTemplate,
}

pub struct MockData {
    pub categories: Vec<Category>,
    /// Dishes and variants together, like the backend's single dish table.
    pub dishes: Vec<Dish>,
    pub comments: Vec<StoredComment>,
    pub modifiers: Vec<Modifier>,
    pub next_comment_number: u32,
}

fn matches_search(name: &str, search_lowercase: &str) -> bool {
    search_lowercase.is_empty() || name.to_lowercase().contains(search_lowercase)
}

impl MockData {
    /// Assumed backend semantics (not visible in legacy code): a search looks
    /// through the whole menu; without a search the root shows top-level
    /// categories plus every dish, and a category shows its direct content.
    pub fn catalog_page(&self, query: &CatalogQuery) -> CatalogPage {
        let search = query.search_text.trim().to_lowercase();
        let category_id = query.category_id.as_deref();

        let sub_categories: Vec<Category> = self
            .categories
            .iter()
            .filter(|category| {
                if !search.is_empty() {
                    matches_search(&category.name, &search)
                } else {
                    category.parent_id.as_deref() == category_id
                }
            })
            .map(|category| self.with_dish_count(category))
            .collect();

        let matching_dishes: Vec<&Dish> = self
            .dishes
            .iter()
            .filter(|dish| dish.kind == DishKind::Dish)
            .filter(|dish| {
                if !search.is_empty() {
                    matches_search(&dish.name, &search)
                } else {
                    category_id.is_none() || dish.category_id.as_deref() == category_id
                }
            })
            .collect();

        let page_index = query.page.max(1) as usize - 1;
        let start = page_index * CATALOG_PAGE_SIZE;
        let end = (start + CATALOG_PAGE_SIZE).min(matching_dishes.len());
        let dishes = matching_dishes
            .get(start..end)
            .unwrap_or_default()
            .iter()
            .map(|dish| (*dish).clone())
            .collect();

        CatalogPage {
            sub_categories,
            dishes,
            has_next_page: end < matching_dishes.len(),
        }
    }

    /// Assumed backend semantics: the count includes dishes of sub-categories,
    /// otherwise every parent category would read "Нет блюда".
    fn with_dish_count(&self, category: &Category) -> Category {
        Category {
            dishes_count: self.count_dishes_under(&category.id),
            ..category.clone()
        }
    }

    fn count_dishes_under(&self, category_id: &str) -> u32 {
        let direct = self
            .dishes
            .iter()
            .filter(|dish| dish.category_id.as_deref() == Some(category_id))
            .count() as u32;
        let nested: u32 = self
            .categories
            .iter()
            .filter(|child| child.parent_id.as_deref() == Some(category_id))
            .map(|child| self.count_dishes_under(&child.id))
            .sum();
        direct + nested
    }

    pub fn variants_of(&self, dish_id: &str) -> Vec<Dish> {
        self.dishes
            .iter()
            .filter(|dish| dish.parent_dish_id.as_deref() == Some(dish_id))
            .cloned()
            .collect()
    }

    pub fn update_dish(
        &mut self,
        id: &str,
        kind: DishKind,
        update: &DishUpdate,
    ) -> Result<Dish, RepositoryError> {
        let dish = self
            .dishes
            .iter_mut()
            .find(|dish| dish.id == id && dish.kind == kind)
            .ok_or_else(|| RepositoryError::not_found("dish", id))?;
        dish.apply(update);
        Ok(dish.clone())
    }

    pub fn comments_of(&self, dish_id: &str) -> Vec<CommentTemplate> {
        self.comments
            .iter()
            .filter(|stored| stored.dish_id == dish_id)
            .map(|stored| stored.comment.clone())
            .collect()
    }

    pub fn add_comment(
        &mut self,
        dish_id: &str,
        text: &str,
    ) -> Result<CommentTemplate, RepositoryError> {
        if !self.dishes.iter().any(|dish| dish.id == dish_id) {
            return Err(RepositoryError::not_found("dish", dish_id));
        }
        self.next_comment_number += 1;
        let comment = CommentTemplate {
            id: format!("comment-{}", self.next_comment_number),
            text: text.trim().to_string(),
        };
        self.comments.push(StoredComment {
            dish_id: dish_id.to_string(),
            comment: comment.clone(),
        });
        Ok(comment)
    }

    pub fn delete_comment(&mut self, comment_id: &str) -> Result<(), RepositoryError> {
        let count_before = self.comments.len();
        self.comments
            .retain(|stored| stored.comment.id != comment_id);
        if self.comments.len() == count_before {
            return Err(RepositoryError::not_found("comment", comment_id));
        }
        Ok(())
    }

    pub fn modifiers_matching(&self, search_text: &str) -> Vec<Modifier> {
        let search = search_text.trim().to_lowercase();
        self.modifiers
            .iter()
            .filter(|modifier| matches_search(&modifier.name, &search))
            .cloned()
            .collect()
    }

    pub fn update_modifier(
        &mut self,
        id: &str,
        update: &ModifierUpdate,
    ) -> Result<Modifier, RepositoryError> {
        let modifier = self
            .modifiers
            .iter_mut()
            .find(|modifier| modifier.id == id)
            .ok_or_else(|| RepositoryError::not_found("modifier", id))?;
        modifier.apply(update);
        Ok(modifier.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::super::seed::sample_data;
    use crate::domain::{CatalogQuery, CATALOG_PAGE_SIZE};

    fn query(category_id: Option<&str>, search_text: &str, page: u32) -> CatalogQuery {
        CatalogQuery {
            category_id: category_id.map(str::to_string),
            search_text: search_text.to_string(),
            page,
        }
    }

    #[test]
    fn root_pages_through_every_dish() {
        let data = sample_data();
        let first = data.catalog_page(&query(None, "", 1));
        assert_eq!(first.dishes.len(), CATALOG_PAGE_SIZE);
        assert!(first.has_next_page);
        assert!(first
            .sub_categories
            .iter()
            .all(|category| category.parent_id.is_none()));
    }

    #[test]
    fn search_ignores_case_and_category() {
        let data = sample_data();
        let page = data.catalog_page(&query(Some("cat-pizza"), "ЧАЙ", 1));
        assert!(page.dishes.iter().any(|dish| dish.id == "dish-green-tea"));
        assert!(page
            .dishes
            .iter()
            .all(|dish| dish.name.to_lowercase().contains("чай")));
    }

    #[test]
    fn parent_category_counts_nested_dishes() {
        let data = sample_data();
        let root = data.catalog_page(&query(None, "", 1));
        let drinks = root
            .sub_categories
            .iter()
            .find(|category| category.id == "cat-drinks");
        assert!(drinks.is_some_and(|category| category.dishes_count > 0));
    }

    #[test]
    fn comments_can_be_added_and_deleted() {
        let mut data = sample_data();
        let added = data
            .add_comment("dish-lagman", "  Без зелени ")
            .expect("dish exists");
        assert_eq!(added.text, "Без зелени");
        assert!(data.comments_of("dish-lagman").contains(&added));
        data.delete_comment(&added.id).expect("comment exists");
        assert!(!data.comments_of("dish-lagman").contains(&added));
    }
}
