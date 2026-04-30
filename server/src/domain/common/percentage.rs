#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage(pub f64);

impl Percentage {
    pub fn new(value: f64) -> Result<Self, String> {
        if value < 0.0 || value > 100.0 {
            return Err("Percentage must be between 0 and 100".to_string());
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
