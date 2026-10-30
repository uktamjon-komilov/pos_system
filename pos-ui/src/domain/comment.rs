//! Comment templates: ready-made order comments ("без лука", ...) attached to a dish.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommentTemplate {
    pub id: String,
    pub text: String,
}

impl CommentTemplate {
    /// Legacy refused to add a comment that is empty after trimming.
    pub fn is_valid_text(text: &str) -> bool {
        !text.trim().is_empty()
    }
}
