use crate::domain::common::percentage::Percentage;

#[derive(Debug, Clone)]
pub struct GroupConfig {
    pub quorum: Percentage,
    pub is_private: bool,
}

impl GroupConfig {
    pub fn new(quorum: Percentage, is_private: bool) -> Self {
        Self { quorum, is_private }
    }
}

impl Default for GroupConfig {
    fn default() -> Self {
        Self {
            quorum: Percentage::new(50.0).expect("50.0 is a valid percentage"),
            is_private: false,
        }
    }
}
