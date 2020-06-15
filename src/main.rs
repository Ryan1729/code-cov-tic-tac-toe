use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let input = stdin.lock();

    let output = io::stdout();

    if cfg!(test) {
        return Ok(());
    }

    run(input, output)    
}

fn run<R, W>(mut reader: R, mut writer: W) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    let mut s = String::new();
    reader.read_line(&mut s)?;
    write!(&mut writer, "{}", s)?;
    
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
