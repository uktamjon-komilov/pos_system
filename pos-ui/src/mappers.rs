//! Converts domain structs into the display structs the Slint screens use.
//! Only number formatting happens here; all wording lives in `ui/strings.slint`.

use crate::domain::{Category, CommentTemplate, Dish, DishKind, Modifier, ProductionType};
use crate::ui::{
    CategoryCardData, CommentData, DishCardData, DishDetailsData, ModifierCardData,
    ModifierDetailsData,
};

/// Groups thousands with spaces, as legacy `formatMoney` did: 1250000 → "1 250 000".
pub fn format_money(amount: u64) -> String {
    let digits = amount.to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            grouped.push(' ');
        }
        grouped.push(digit);
    }
    grouped
}

fn to_slint_int(value: u32) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}

pub fn category_card(category: &Category) -> CategoryCardData {
    CategoryCardData {
        id: category.id.as_str().into(),
        name: category.name.as_str().into(),
        dishes_count: to_slint_int(category.dishes_count),
    }
}

pub fn dish_card(dish: &Dish) -> DishCardData {
    let is_variant = dish.kind == DishKind::Variant;
    let title = match (&dish.size_name, is_variant) {
        (Some(size_name), true) => size_name.as_str(),
        _ => dish.name.as_str(),
    };
    DishCardData {
        id: dish.id.as_str().into(),
        title: title.into(),
        price_text: format_money(dish.price).into(),
        is_variant,
        variant_count: to_slint_int(dish.variant_count),
        is_in_go_list: dish.is_in_go_list,
        is_in_stop_list: dish.is_in_stop_list,
        // Unlimited maps to 0, which hides the card badge like legacy's null.
        limit_count: to_slint_int(dish.portion_limit.display_count()),
        cooking_minutes: to_slint_int(dish.cooking_time_minutes),
    }
}

pub fn dish_details(dish: &Dish) -> DishDetailsData {
    DishDetailsData {
        id: dish.id.as_str().into(),
        name: dish.name.as_str().into(),
        description: dish.description.as_str().into(),
        price_text: format_money(dish.price).into(),
        variant_count: to_slint_int(dish.variant_count),
        is_in_go_list: dish.is_in_go_list,
        is_in_stop_list: dish.is_in_stop_list,
        is_unlimited: dish.portion_limit.is_unlimited(),
        limit_count: to_slint_int(dish.portion_limit.display_count()),
        has_portion_limit: dish.production_type != ProductionType::WeightBased,
    }
}

pub fn modifier_card(modifier: &Modifier) -> ModifierCardData {
    ModifierCardData {
        id: modifier.id.as_str().into(),
        name: modifier.name.as_str().into(),
        price_text: format_money(modifier.price).into(),
        limit_count: to_slint_int(modifier.portion_limit.display_count()),
    }
}

pub fn modifier_details(modifier: &Modifier) -> ModifierDetailsData {
    ModifierDetailsData {
        id: modifier.id.as_str().into(),
        name: modifier.name.as_str().into(),
        is_in_stop_list: modifier.is_in_stop_list,
        is_unlimited: modifier.portion_limit.is_unlimited(),
        limit_count: to_slint_int(modifier.portion_limit.display_count()),
    }
}

pub fn comment(comment: &CommentTemplate) -> CommentData {
    CommentData {
        id: comment.id.as_str().into(),
        text: comment.text.as_str().into(),
    }
}

#[cfg(test)]
mod tests {
    use super::format_money;

    #[test]
    fn groups_thousands_with_spaces() {
        assert_eq!(format_money(0), "0");
        assert_eq!(format_money(999), "999");
        assert_eq!(format_money(45_000), "45 000");
        assert_eq!(format_money(1_250_000), "1 250 000");
    }
}
