/// Returns the sum of all the natural numbers up to (but not including) `limit` that are multiples
/// of any of the `factors`.
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| factors.iter().any(|y| *y > 0 && x % y == 0))
        .sum()
}
