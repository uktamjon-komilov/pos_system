//! A dish or a dish variant (size), and the partial update staff can apply to it.

use super::PortionLimit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DishKind {
    Dish,
    /// A size of a dish ("30 см"); it has its own price, stock flags and limit.
    Variant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductionType {
    Piece,
    /// Sold by weight: portion limits make no sense, so the UI hides them.
    WeightBased,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dish {
    pub id: String,
    pub kind: DishKind,
    /// Set for variants only.
    pub parent_dish_id: Option<String>,
    /// Set for dishes only.
    pub category_id: Option<String>,
    pub name: String,
    /// Set for variants only, e.g. "30 см".
    pub size_name: Option<String>,
    /// Whole sums; the currency has no minor unit in practice.
    pub price: u64,
    pub variant_count: u32,
    /// 0 means "not specified".
    pub cooking_time_minutes: u32,
    pub description: String,
    pub production_type: ProductionType,
    pub is_in_go_list: bool,
    pub is_in_stop_list: bool,
    pub portion_limit: PortionLimit,
}

impl Dish {
    pub fn has_variants(&self) -> bool {
        self.kind == DishKind::Dish && self.variant_count > 0
    }

    /// A variant inherits what it does not define itself from its parent
    /// dish (cooking time, description, ...), the way legacy merged the two.
    pub fn merged_with_parent(&self, parent: &Dish) -> Dish {
        Dish {
            category_id: parent.category_id.clone(),
            variant_count: parent.variant_count,
            cooking_time_minutes: parent.cooking_time_minutes,
            description: parent.description.clone(),
            production_type: parent.production_type,
            ..self.clone()
        }
    }

    pub fn apply(&mut self, update: &DishUpdate) {
        if let Some(is_in_go_list) = update.is_in_go_list {
            self.is_in_go_list = is_in_go_list;
        }
        if let Some(is_in_stop_list) = update.is_in_stop_list {
            self.is_in_stop_list = is_in_stop_list;
        }
        if let Some(portion_limit) = update.portion_limit {
            self.portion_limit = portion_limit;
        }
    }
}

/// Fields that the POS may change on a dish; `None` leaves a field untouched.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DishUpdate {
    pub is_in_go_list: Option<bool>,
    pub is_in_stop_list: Option<bool>,
    pub portion_limit: Option<PortionLimit>,
}

impl DishUpdate {
    /// A dish cannot be pushed (go-list) and blocked (stop-list) at the same
    /// time, so switching one on switches the other off.
    pub fn go_list(dish: &Dish, enabled: bool) -> DishUpdate {
        DishUpdate {
            is_in_go_list: Some(enabled),
            is_in_stop_list: Some(if enabled { false } else { dish.is_in_stop_list }),
            portion_limit: None,
        }
    }

    pub fn stop_list(dish: &Dish, enabled: bool) -> DishUpdate {
        DishUpdate {
            is_in_go_list: Some(if enabled { false } else { dish.is_in_go_list }),
            is_in_stop_list: Some(enabled),
            portion_limit: None,
        }
    }

    pub fn portion_limit(limit: PortionLimit) -> DishUpdate {
        DishUpdate {
            portion_limit: Some(limit),
            ..DishUpdate::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_dish() -> Dish {
        Dish {
            id: "d".into(),
            kind: DishKind::Dish,
            parent_dish_id: None,
            category_id: None,
            name: "Плов".into(),
            size_name: None,
            price: 1000,
            variant_count: 0,
            cooking_time_minutes: 0,
            description: String::new(),
            production_type: ProductionType::Piece,
            is_in_go_list: false,
            is_in_stop_list: true,
            portion_limit: PortionLimit::Unlimited,
        }
    }

    #[test]
    fn go_list_turns_stop_list_off() {
        let mut dish = sample_dish();
        dish.apply(&DishUpdate::go_list(&dish.clone(), true));
        assert!(dish.is_in_go_list);
        assert!(!dish.is_in_stop_list);
    }

    #[test]
    fn turning_go_list_off_keeps_stop_list() {
        let dish = sample_dish();
        let update = DishUpdate::go_list(&dish, false);
        assert_eq!(update.is_in_stop_list, Some(true));
    }
}
