use std::cmp::min;

pub trait LevenshteinDistance {
    fn levenshtein_distance(&self, rhs: &Self) -> usize;
}

impl LevenshteinDistance for str {
    fn levenshtein_distance(&self, rhs: &Self) -> usize {
        let mut v0 = vec![0; rhs.len() + 1];
        let mut v1 = vec![0; rhs.len() + 1];

        for i in 0..=self.len() {
            v1[0] = i;
            for j in 0..=rhs.len() {
                if i == 0 {
                    v1[j] = j;
                }
                else if j > 0 {
                    let cost = if self.chars().nth(i - 1) == rhs.chars().nth(j - 1) { 0 } else { 1 };
                    v1[j] = min(v1[j - 1] + 1, min(v0[j] + 1, v0[j - 1] + cost));
                }
            }
            v0.clone_from_slice(&v1);
        }
        v1[rhs.len()]
    }
}

#[test]
fn test_levenshtein_distance() {
    let s1 = "kitten";
    let s2 = "sitting";
    assert_eq!(s1.levenshtein_distance(s2), 3);
}
