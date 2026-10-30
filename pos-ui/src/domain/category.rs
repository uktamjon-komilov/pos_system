//! A menu category. Categories nest: a category can hold sub-categories and dishes.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Category {
    pub id: String,
    pub name: String,
    /// `None` for top-level categories.
    pub parent_id: Option<String>,
    /// Number of dishes in this category, including its sub-categories.
    pub dishes_count: u32,
}
