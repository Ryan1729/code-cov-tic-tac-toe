pub fn main() -> i32 {
    2 + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(main(), 4);
    }

    #[test]
    fn it_works2() {
        it_works();
        assert_eq!(main(), 4);
    }
}
