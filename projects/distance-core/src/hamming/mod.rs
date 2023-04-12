pub trait HammingDistance {
    // Must same length, or it will panic
    fn hamming_distance(&self, rhs: &Self) -> usize;
    fn hamming_weight(&self) -> usize;
}

impl HammingDistance for bool {
    fn hamming_distance(&self, rhs: &Self) -> usize {
        match self.eq(rhs) {
            true => 0,
            false => 1,
        }
    }

    fn hamming_weight(&self) -> usize {
        match self {
            true => 1,
            false => 0,
        }
    }
}

macro_rules! impl_primitive_type {
    ($($t:ty)*) => ($(
        impl HammingDistance for $t {
            fn hamming_distance(&self, rhs: &Self) -> usize {
                (self ^ rhs).count_ones() as usize
            }

            fn hamming_weight(&self) -> usize {
                self.count_ones() as usize
            }
        }
    )*)
}

impl_primitive_type! { u8 u16 u32 u64 u128 usize }
impl_primitive_type! { i8 i16 i32 i64 i128 isize }

impl HammingDistance for str {
    fn hamming_distance(&self, rhs: &Self) -> usize {
        debug_assert_eq!(self.len(), rhs.len(), "The length of two strings must be same");
        self.chars().zip(rhs.chars()).map(|(a, b)| a != b).count()
    }

    fn hamming_weight(&self) -> usize {
        panic!("Hamming weight is not defined for string")
    }
}

impl HammingDistance for String {
    fn hamming_distance(&self, rhs: &Self) -> usize {
        self.as_str().hamming_distance(rhs.as_str())
    }

    fn hamming_weight(&self) -> usize {
        self.as_str().hamming_weight()
    }
}

impl<T> HammingDistance for [T]
where
    T: PartialEq + HammingDistance,
{
    fn hamming_distance(&self, rhs: &Self) -> usize {
        debug_assert_eq!(self.len(), rhs.len(), "The length of two byte arrays must be same");
        self.iter().zip(rhs.iter()).map(|(a, b)| a != b).count()
    }

    fn hamming_weight(&self) -> usize {
        self.iter().map(|x| x.hamming_weight()).sum()
    }
}
