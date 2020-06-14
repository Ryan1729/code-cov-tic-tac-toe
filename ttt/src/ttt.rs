pub fn main() -> i32 {
    4
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(main(), 4);
    }
}
