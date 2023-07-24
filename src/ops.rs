use crate::MixVec;

impl std::ops::Add for MixVec {
    type Output = MixVec;

    fn add(mut self, mut other: MixVec) -> MixVec {
        let mut result = MixVec::new();
        // result.append(&mut self.clone());
        result.append(&mut self);
        // result.append(&mut other.clone());
        result.append(&mut other);
        result
    }
}

impl std::ops::Mul<usize> for MixVec {
    type Output = MixVec;

    fn mul(mut self, count: usize) -> MixVec {
        let mut result = MixVec::new();
        for _ in 0..count {
            // result.append(&mut self.clone());
            result.append(&mut self);
        }
        result
    }
}
