//! A tiny crate for testing the iomarkets.dev Proof Arena end to end.

/// Counts up from 0 to `n` one step at a time and returns the count.
///
/// Always returns exactly `n`.
pub fn count_up(n: u32) -> u32 {
    let mut i = 0u32;
    while i < n {
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts() {
        assert_eq!(count_up(0), 0);
        assert_eq!(count_up(7), 7);
    }
}
