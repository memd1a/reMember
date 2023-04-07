/// Helper type to calculate size hint
pub struct SizeHint(pub Option<usize>);

impl SizeHint {
    pub const fn zero() -> Self {
        Self(Some(0))
    }

    /// Sum two Option<usize>
    /// When const traits become stable Add can be implemented
    pub const fn add(self, rhs: Self) -> Self {
        Self(match (self.0, rhs.0) {
            (Some(a), Some(b)) => Some(a + b),
            _ => None,
        })
    }
}