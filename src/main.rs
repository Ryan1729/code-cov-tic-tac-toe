use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    if cfg!(test) {
        return Ok(());
    }

    let mut buffer = String::new();
    input.read_line(&mut buffer)?;

    print!("{}", buffer);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main_does_not_panic() {
        let _ = main();
    }
}
