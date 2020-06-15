fn main() {
    fact(5);
}

fn fact(x: u8) -> u8 {
    if x <= 1 {
        0
    } else {
        fact(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        main();
    }
}