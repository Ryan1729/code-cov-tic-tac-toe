fn main() {
    fact(5);
}

fn fact(x: u8) -> u8 {
    if x <= 1 {
        1
    } else {
        x * fact(x - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main_does_not_panic() {
        main();
    }
}
