use core::fmt;

#[derive(Debug)]
pub struct MergeRule(pub(crate) (String, String));

impl fmt::Display for MergeRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (left, right) = &self.0;
        write!(f, "{left}{right}")
    }
}
