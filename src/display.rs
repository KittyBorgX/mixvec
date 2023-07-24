use crate::MixVec;
use crate::MixVecElement;

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
                MixVecElement::Custom(c) => write!(f, "{:?}", c)?,
            }
        }
        write!(f, "]")
    }
}
