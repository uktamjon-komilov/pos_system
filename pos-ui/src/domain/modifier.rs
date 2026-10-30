//! A modifier (add-on such as "Сыр" or "Соус") and its partial update.

use super::PortionLimit;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Modifier {
    pub id: String,
    pub name: String,
    pub price: u64,
    pub is_in_stop_list: bool,
    pub is_visible: bool,
    pub portion_limit: PortionLimit,
}

impl Modifier {
    pub fn apply(&mut self, update: &ModifierUpdate) {
        if let Some(is_in_stop_list) = update.is_in_stop_list {
            self.is_in_stop_list = is_in_stop_list;
        }
        if let Some(is_visible) = update.is_visible {
            self.is_visible = is_visible;
        }
        if let Some(portion_limit) = update.portion_limit {
            self.portion_limit = portion_limit;
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ModifierUpdate {
    pub is_in_stop_list: Option<bool>,
    pub is_visible: Option<bool>,
    pub portion_limit: Option<PortionLimit>,
}

impl ModifierUpdate {
    /// A modifier on the stop-list is also hidden from the order screen.
    pub fn stop_list(enabled: bool) -> ModifierUpdate {
        ModifierUpdate {
            is_in_stop_list: Some(enabled),
            is_visible: Some(!enabled),
            portion_limit: None,
        }
    }

    pub fn portion_limit(limit: PortionLimit) -> ModifierUpdate {
        ModifierUpdate {
            portion_limit: Some(limit),
            ..ModifierUpdate::default()
        }
    }
}
