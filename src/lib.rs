#![cfg_attr(kani, feature(stmt_expr_attributes, proc_macro_hygiene))]
//! A tiny crate for testing the iomarkets.dev Proof Arena end to end.

/// Counts up from 0 to `n` one step at a time and returns the count.
///
/// Always returns exactly `n`.
pub fn count_up(n: u32) -> u32 {
    let mut i = 0u32;
    #[cfg_attr(kani, kani::loop_invariant(i <= n))]
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

/// Proved in the Proof Arena (iomarkets.dev/proof): Kani checks this loop for every input with the invariant above.
#[cfg(kani)]
mod iomarkets_proof {
    use super::*;

    #[kani::proof]
    fn count_up_returns_n() {
        let n: u32 = kani::any();
        assert_eq!(count_up(n), n);
    }
}
