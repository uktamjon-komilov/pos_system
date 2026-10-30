//! Everything the modifiers tab remembers between UI events.

use crate::domain::Modifier;

#[derive(Default)]
pub struct ModifiersState {
    pub modifiers: Vec<Modifier>,
    pub selected_id: Option<String>,
    pub is_loading: bool,
    /// Bumped per request; answers to older searches are dropped.
    pub request_generation: u64,
}

impl ModifiersState {
    pub fn selected(&self) -> Option<&Modifier> {
        let selected_id = self.selected_id.as_deref()?;
        self.modifiers
            .iter()
            .find(|modifier| modifier.id == selected_id)
    }

    pub fn replace(&mut self, updated: &Modifier) {
        for listed in self
            .modifiers
            .iter_mut()
            .filter(|listed| listed.id == updated.id)
        {
            *listed = updated.clone();
        }
    }
}
