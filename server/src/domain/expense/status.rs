#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpenseStatus {
    Created,
    Verified,
    Updated,
    Deleted,
}

impl ExpenseStatus {
    pub fn is_deleted(self) -> bool {
        matches!(self, ExpenseStatus::Deleted)
    }

    pub fn is_active(self) -> bool {
        !self.is_deleted()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deleted_is_not_active() {
        assert!(ExpenseStatus::Deleted.is_deleted());
        assert!(!ExpenseStatus::Deleted.is_active());
    }

    #[test]
    fn created_updated_verified_are_active() {
        for status in [
            ExpenseStatus::Created,
            ExpenseStatus::Updated,
            ExpenseStatus::Verified,
        ] {
            assert!(status.is_active());
            assert!(!status.is_deleted());
        }
    }
}
