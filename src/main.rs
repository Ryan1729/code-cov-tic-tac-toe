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
    enum State {
        Game,
        ConfirmQuit,
    }
    use State::*;

    let mut state = Game;

    let mut buffer = String::new();
    loop {
        reader.read_line(&mut buffer)?;

        if let Some(c) = buffer.chars().next() {
            match state {
                Game => {
                    match c {
                        '0' => {
                            state = ConfirmQuit;
                        }
                        _ => {}
                    }
                }
                _ => { // ConfirmQuit or invalid enum
                    match c {
                        '0' => {
                            state = Game;
                        }
                        '1' => {
                            write!(&mut writer, "bye\n")?;
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }

        buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    #[test]
    fn main_does_not_panic() {
        let _ = main();
    }

    macro_rules! output_to_string {
        ($output_vec: ident) => {{
            let output = String::from_utf8($output_vec)
                .expect("output contained Non UTF-8 bytes");

            assert!(output.ends_with("\n"));

            output
        }}
    }

    macro_rules! quit_assert {
        ($output_vec: ident) => {
            let output = output_to_string!($output_vec);

            assert!(output.contains("bye"));
        }
    }

    #[test]
    fn run_allows_quitting() {
        let input = b"0\n1\n";
        let mut output = Vec::new();
    
        run(&input[..], &mut output).unwrap();

        quit_assert!(output);
    }

    #[test]
    fn run_allows_deciding_not_to_quit_then_really_quitting() {
        let input = b"0\n0\n0\n1\n";
        let mut output = Vec::new();
    
        run(&input[..], &mut output).unwrap();

        quit_assert!(output);
    }

    #[test]
    fn run_allows_typing_a_non_number_then_quitting() {
        let input = b"hi\n0\n1\n";
        let mut output = Vec::new();
    
        run(&input[..], &mut output).unwrap();

        quit_assert!(output);
    }

    #[test]
    fn run_terminates_on_an_initial_read_error() {
        let mut output = Vec::new();
    
        let _ = run(FailingReader, &mut output);
    }

    struct FailingReader;

    impl BufRead for FailingReader {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Err(io::Error::new(io::ErrorKind::Other, "FailingReader fill_buf error"))
        }

        fn consume(&mut self, _: usize) {}
    }

    impl Read for FailingReader {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> { 
            Err(io::Error::new(io::ErrorKind::Other, "FailingReader read error"))
        }
    }

    mod meta {
        use super::*;
        #[test]
        fn failing_reader_terminates() {
            let _ = FailingReader.read(&mut []);
            let _ = FailingReader.fill_buf();
            let _ = FailingReader.consume(0);
        }
    }
}
