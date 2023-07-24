use std::ops::Index;

use crate::MixVec;
use crate::MixVecElement;

impl From<i32> for MixVecElement {
    fn from(value: i32) -> Self {
        MixVecElement::Integer(value)
    }
}

impl From<f64> for MixVecElement {
    fn from(value: f64) -> Self {
        MixVecElement::Float(value)
    }
}

impl From<String> for MixVecElement {
    fn from(value: String) -> Self {
        MixVecElement::String(value)
    }
}

impl From<bool> for MixVecElement {
    fn from(value: bool) -> Self {
        MixVecElement::Boolean(value)
    }
}

impl From<char> for MixVecElement {
    fn from(value: char) -> Self {
        MixVecElement::Character(value)
    }
}

impl From<&str> for MixVecElement {
    fn from(item: &str) -> Self {
        MixVecElement::String(item.to_string())
    }
}

impl Index<usize> for MixVec {
    type Output = MixVecElement;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
