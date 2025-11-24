use crate::core::strumming_pattern::StrummingPattern;

pub struct AppState {
    pub strumming_pattern: StrummingPattern,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            strumming_pattern: StrummingPattern::new_random(8),
        }
    }
}

