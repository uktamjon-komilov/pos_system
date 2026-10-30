//! Query and result types for browsing the dish catalog page by page.

use super::{Category, Dish};

/// The legacy client always asked the backend for pages of 12 dishes.
pub const CATALOG_PAGE_SIZE: usize = 12;

/// What the dish list currently asks for.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CatalogQuery {
    /// `None` means the root of the menu ("Все блюда").
    pub category_id: Option<String>,
    /// Empty string means "no search".
    pub search_text: String,
    /// 1-based, like the backend pagination.
    pub page: u32,
}

/// One page of the catalog.
#[derive(Clone, Debug, Default)]
pub struct CatalogPage {
    /// Sub-categories of the requested category. Only meaningful on page 1;
    /// later pages repeat them and the client ignores them, as legacy did.
    pub sub_categories: Vec<Category>,
    pub dishes: Vec<Dish>,
    pub has_next_page: bool,
}
