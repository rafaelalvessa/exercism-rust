/// Returns the nth prime number, using a 0-based index.
///
/// # Examples
///
/// ```
/// use nth_prime::nth;
/// assert_eq!(nth(0), 2);
/// assert_eq!(nth(1), 3);
/// ```
pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        n => {
            let mut primes = vec![];

            (3..)
                .step_by(2)
                .filter(|x| {
                    if primes.iter().all(|y| x % y != 0) {
                        primes.push(*x);
                        return true;
                    }

                    false
                })
                .nth(n as usize - 1)
                .unwrap()
        }
    }
}
