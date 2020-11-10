/// Returns the square of the sum of the first n natural numbers.
pub fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1) / 2).pow(2)
}

/// Returns the sum of the squares of the first n natural numbers.
pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

/// Returns the difference between the square of sum and the sum of squares of the first n natural
/// numbers.
pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
