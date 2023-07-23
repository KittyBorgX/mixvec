pub mod cmp;
pub mod display;
pub mod from;
pub mod ops;

#[derive(Debug, Clone)]
pub struct MixVec {
    data: Vec<MixVecElement>,
}
#[derive(Debug, Clone)]
pub enum MixVecElement {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Character(char),
}

impl MixVec {
    pub fn new() -> Self {
        MixVec { data: Vec::new() }
    }

    pub fn push<T>(&mut self, element: T)
    where
        T: Into<MixVecElement>,
    {
        self.data.push(element.into());
    }

    pub fn pop(&mut self) -> Option<MixVecElement> {
        self.data.pop()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn append(&mut self, other: &mut MixVec) {
        self.data.append(&mut other.data);
    }

    pub fn insert<T: Into<MixVecElement>>(&mut self, index: usize, element: T) {
        if index <= self.data.len() {
            self.data.insert(index, element.into());
        }
    }

    pub fn remove(&mut self, element: &MixVecElement) -> Option<MixVecElement> {
        if let Some(index) = self.data.iter().position(|x| x == element) {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    pub fn sort(&mut self) {
        self.data.sort();
    }
}

#[macro_export]
macro_rules! mixvec {
    ($($item:expr),*) => {{
        let mut temp_vec = MixVec::new();
        $(temp_vec.push($item);)*
        temp_vec
    }};
}
