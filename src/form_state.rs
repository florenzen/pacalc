use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub struct FormState {
    pub pace: Duration,
    pub splits: usize,
    pub distance: usize,
    pub show_splits: bool,
}

impl Default for FormState {
    fn default() -> Self {
        Self {
            pace: Duration::ZERO,
            splits: 0,
            distance: 0,
            show_splits: true,
        }
    }
}
