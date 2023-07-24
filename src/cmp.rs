use std::cmp::Ordering;

use crate::MixVecElement;
impl PartialEq for MixVecElement {
    fn eq(&self, other: &Self) -> bool {
        use MixVecElement::*;
        match (self, other) {
            (Integer(a), Integer(b)) => a == b,
            (Float(a), Float(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Boolean(a), Boolean(b)) => a == b,
            (Character(a), Character(b)) => a == b,
            (_, _) => false,
        }
    }
}
impl PartialEq<i32> for MixVecElement {
    fn eq(&self, other: &i32) -> bool {
        if let MixVecElement::Integer(val) = self {
            val == other
        } else {
            false
        }
    }
}

impl PartialEq<f64> for MixVecElement {
    fn eq(&self, other: &f64) -> bool {
        if let MixVecElement::Float(val) = self {
            val == other
        } else {
            false
        }
    }
}

impl PartialEq<String> for MixVecElement {
    fn eq(&self, other: &String) -> bool {
        if let MixVecElement::String(val) = self {
            val == other.as_str()
        } else {
            false
        }
    }
}

impl PartialEq<bool> for MixVecElement {
    fn eq(&self, other: &bool) -> bool {
        if let MixVecElement::Boolean(val) = self {
            val == other
        } else {
            false
        }
    }
}

impl PartialEq<char> for MixVecElement {
    fn eq(&self, other: &char) -> bool {
        if let MixVecElement::Character(val) = self {
            val == other
        } else {
            false
        }
    }
}

impl Ord for MixVecElement {
    fn cmp(&self, other: &Self) -> Ordering {
        use MixVecElement::*;
        match (self, other) {
            (Integer(a), Integer(b)) => a.cmp(b),
            (Float(a), Float(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
            (String(a), String(b)) => a.cmp(b),
            (Boolean(a), Boolean(b)) => a.cmp(b),
            (Character(a), Character(b)) => a.cmp(b),
            // Different variants are not comparable, so we'll compare their discriminants.
            _ => format!("{:?}", self).cmp(&format!("{:?}", other)),
        }
    }
}

impl Eq for MixVecElement {}

impl PartialOrd for MixVecElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
