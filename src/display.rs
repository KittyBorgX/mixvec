use crate::{MixVec, MixVecElement};

impl std::fmt::Display for MixVec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, elem) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            match elem {
                MixVecElement::Integer(i) => write!(f, "{}", i)?,
                MixVecElement::Float(fl) => write!(f, "{}", fl)?,
                MixVecElement::String(s) => write!(f, "\"{}\"", s)?,
                MixVecElement::Boolean(b) => write!(f, "{}", b)?,
                MixVecElement::Character(c) => write!(f, "{}", c)?,
            }
        }
        write!(f, "]")
    }
}

impl std::fmt::Display for MixVecElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MixVecElement::Integer(val) => write!(f, "{}", val),
            MixVecElement::Float(val) => write!(f, "{}", val),
            MixVecElement::String(val) => write!(f, "{}", val),
            MixVecElement::Boolean(val) => write!(f, "{}", val),
            MixVecElement::Character(val) => write!(f, "{}", val),
        }
    }
}
