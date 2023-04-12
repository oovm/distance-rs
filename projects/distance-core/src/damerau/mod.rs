pub trait DamerauDistance {
    fn damerau_distance(&self, rhs: &Self) -> usize;
}

impl DamerauDistance for str {
    fn damerau_distance(&self, rhs: &Self) -> usize {
        todo!()
    }
}
