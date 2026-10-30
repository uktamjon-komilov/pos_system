//! Plain data types of the menu ("eatables") and the few rules that belong to
//! them. Nothing here knows about Slint or about where the data comes from.

mod catalog;
mod category;
mod comment;
mod dish;
mod modifier;
mod portion_limit;

pub use catalog::{CatalogPage, CatalogQuery, CATALOG_PAGE_SIZE};
pub use category::Category;
pub use comment::CommentTemplate;
pub use dish::{Dish, DishKind, DishUpdate, ProductionType};
pub use modifier::{Modifier, ModifierUpdate};
pub use portion_limit::PortionLimit;
