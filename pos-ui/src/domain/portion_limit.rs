//! How many portions of a dish or modifier may still be sold.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortionLimit {
    /// The backend calls this `is_limited = true` (sic): the flag is inverted
    /// there, and the legacy UI labels it "Установлено бесконечное количество порций".
    Unlimited,
    Count(u32),
}

impl PortionLimit {
    /// Count shown in "Установлено" / "Осталось". Unlimited shows 0, as in legacy.
    pub fn display_count(self) -> u32 {
        match self {
            PortionLimit::Unlimited => 0,
            PortionLimit::Count(count) => count,
        }
    }

    pub fn is_unlimited(self) -> bool {
        self == PortionLimit::Unlimited
    }

    /// Builds a limit from the limit form. The keypad lets the user type any
    /// mix of digits and dots, so `None` means the count is not a whole number.
    pub fn from_form(unlimited: bool, count_text: &str) -> Option<PortionLimit> {
        if unlimited {
            return Some(PortionLimit::Unlimited);
        }
        let trimmed = count_text.trim();
        if trimmed.is_empty() {
            return Some(PortionLimit::Count(0));
        }
        trimmed.parse::<u32>().ok().map(PortionLimit::Count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unlimited_ignores_the_count_text() {
        assert_eq!(
            PortionLimit::from_form(true, "abc"),
            Some(PortionLimit::Unlimited)
        );
    }

    #[test]
    fn empty_count_means_zero() {
        assert_eq!(
            PortionLimit::from_form(false, " "),
            Some(PortionLimit::Count(0))
        );
    }

    #[test]
    fn fractional_count_is_rejected() {
        assert!(PortionLimit::from_form(false, "1.5").is_none());
    }
}
