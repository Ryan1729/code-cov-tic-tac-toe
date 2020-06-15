
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
    fn fact_returns_expected_values_for_all_inputs() {
        assert_eq!(fact(0), 1);
        assert_eq!(fact(1), 1);
        assert_eq!(fact(2), 2);
        assert_eq!(fact(3), 6);
        assert_eq!(fact(4), 24);
        assert_eq!(fact(5), 120);

        let max_value = u8::max_value();
        let mut i = 6;
        while i < max_value {
            assert_eq!(fact(i), 0);
            i = i.wrapping_add(1);
        }

        assert_eq!(fact(max_value), 0);
    }
}
