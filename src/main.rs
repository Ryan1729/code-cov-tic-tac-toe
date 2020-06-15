fn main() {
    fact(5);
}

fn fact(n: u8) -> u8 {
    match n.checked_sub(1) {
        None => 1,
        Some(n_minus_1) => {
            n.checked_mul(fact(n_minus_1))
                .unwrap_or_default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main_does_not_panic() {
        main();
    }

    #[test]
    fn fact_0_is_1() {
        assert_eq!(fact(0), 1);
    }
}
