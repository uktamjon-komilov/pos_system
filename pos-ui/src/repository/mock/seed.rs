//! Sample menu for the mock: categories, modifiers and comment templates.
//! Chosen to hit every UI state: nesting, empty and long-named categories,
//! stop-listed and limited items, many entries for scrolling.

use super::data::{MockData, StoredComment};
use super::seed_dishes::sample_dishes;
use crate::domain::{Category, CommentTemplate, Modifier, PortionLimit};

pub fn sample_data() -> MockData {
    let comments = sample_comments();
    MockData {
        categories: sample_categories(),
        dishes: sample_dishes(),
        next_comment_number: comments.len() as u32,
        comments,
        modifiers: sample_modifiers(),
    }
}

fn category(id: &str, name: &str, parent_id: Option<&str>) -> Category {
    Category {
        id: id.to_string(),
        name: name.to_string(),
        parent_id: parent_id.map(str::to_string),
        // Recomputed from the dish table on every query.
        dishes_count: 0,
    }
}

fn sample_categories() -> Vec<Category> {
    vec![
        category("cat-hot", "Горячие блюда", None),
        category("cat-soups", "Супы", Some("cat-hot")),
        category("cat-grill", "Шашлыки", Some("cat-hot")),
        category("cat-national", "Плов и национальные блюда", Some("cat-hot")),
        category("cat-salads", "Салаты", None),
        category("cat-pizza", "Пицца", None),
        category("cat-burgers", "Бургеры и сэндвичи", None),
        category("cat-breakfast", "Завтраки", None),
        category("cat-desserts", "Десерты", None),
        category("cat-drinks", "Напитки", None),
        category("cat-hot-drinks", "Горячие напитки", Some("cat-drinks")),
        category("cat-tea", "Чай", Some("cat-hot-drinks")),
        category("cat-cold-drinks", "Холодные напитки", Some("cat-drinks")),
        category("cat-kids", "Детское меню", None),
        category("cat-seasonal", "Сезонное меню", None),
        category(
            "cat-chef",
            "Блюда от шеф-повара по специальным праздничным рецептам нашего ресторана",
            None,
        ),
    ]
}

fn modifier(id: &str, name: &str, price: u64) -> Modifier {
    Modifier {
        id: id.to_string(),
        name: name.to_string(),
        price,
        is_in_stop_list: false,
        is_visible: true,
        portion_limit: PortionLimit::Unlimited,
    }
}

fn sample_modifiers() -> Vec<Modifier> {
    vec![
        modifier("mod-cheese", "Сыр моцарелла", 6_000),
        modifier("mod-bacon", "Бекон", 9_000),
        Modifier {
            portion_limit: PortionLimit::Count(12),
            ..modifier("mod-mushrooms", "Шампиньоны", 5_000)
        },
        modifier("mod-jalapeno", "Халапеньо", 4_000),
        Modifier {
            is_in_stop_list: true,
            is_visible: false,
            ..modifier("mod-truffle", "Трюфельное масло", 18_000)
        },
        modifier("mod-garlic-sauce", "Чесночный соус", 3_000),
        modifier("mod-cheese-sauce", "Сырный соус", 3_000),
        Modifier {
            portion_limit: PortionLimit::Count(4),
            ..modifier("mod-avocado", "Авокадо", 12_000)
        },
        modifier("mod-egg", "Яйцо пашот", 5_000),
        modifier("mod-onion", "Карамелизированный лук", 3_500),
        modifier(
            "mod-double-patty",
            "Двойная котлета из мраморной говядины",
            25_000,
        ),
        modifier("mod-lemon", "Лимон", 2_000),
        modifier("mod-honey", "Мёд", 4_000),
        Modifier {
            is_in_stop_list: true,
            is_visible: false,
            ..modifier("mod-syrup", "Сироп карамельный", 5_000)
        },
        modifier("mod-milk", "Растительное молоко", 7_000),
        modifier("mod-ice", "Лёд", 0),
        modifier("mod-bread", "Хлеб тандырный", 4_000),
        Modifier {
            portion_limit: PortionLimit::Count(30),
            ..modifier("mod-greens", "Зелень", 2_500)
        },
    ]
}

fn comment(number: u32, dish_id: &str, text: &str) -> StoredComment {
    StoredComment {
        dish_id: dish_id.to_string(),
        comment: CommentTemplate {
            id: format!("comment-{number}"),
            text: text.to_string(),
        },
    }
}

fn sample_comments() -> Vec<StoredComment> {
    vec![
        comment(1, "dish-plov", "Без лука"),
        comment(2, "dish-plov", "Поменьше масла"),
        comment(3, "dish-plov", "Салат отдельно"),
        comment(4, "dish-plov", "Мясо порезать"),
        comment(5, "dish-margherita", "Хорошо пропечь"),
        comment(6, "dish-margherita", "Без базилика"),
        comment(7, "dish-lagman", "Острый"),
        comment(8, "dish-burger", "Прожарка medium"),
        comment(
            9,
            "dish-burger",
            "Без соуса, соус подать отдельно в маленькой соуснице",
        ),
    ]
}
