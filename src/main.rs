fn main() {
    fact(5);
}

fn fact(n: u8) -> u8 {
    let difference = n.checked_sub(1);

    if let Some(n_minus_1) = difference {
        n.checked_mul(fact(n_minus_1))
            .unwrap_or_default()
    } else {
        1
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
