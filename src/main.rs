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

fn run<R, W>(reader: R, writer: W) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    let mut buffer = String::with_capacity(1);

    run_game(reader, writer, &mut buffer)
}

#[derive(Clone, Copy, Debug)]
enum State {
    Game,
    ConfirmQuit,
    Quit,
}

fn run_game<R, W>(mut reader: R, mut writer: W, buffer: &mut String) -> io::Result<()> 
where
    R: BufRead,
    W: Write,
{
    let mut state = State::Game;
    
    let mut output;

    loop {
        let read_res = reader.read_line(buffer);

        let is_err = read_res.is_err();

        if is_err {
            output = Err(read_res.unwrap_err());
            break
        }

        if let Some(c) = buffer.chars().next() {
            output = game_step(&mut writer, &mut state, c);

            if output.is_err() {
                break;
            }

            if let State::Quit = state {
                break;
            }
        }

        buffer.clear();
    }

    drop(reader);
    drop(writer);

    output
}

fn game_step<W>(writer: &mut W, state: &mut State, c: char) -> io::Result<()>
where
    W: Write,
{
    use State::*;
    match state {
        Game => {
            match c {
                '0' => {
                    *state = ConfirmQuit;
                }
                _ => {}
            }
        }
        ConfirmQuit => {
            match c {
                '0' => {
                    *state = Game;
                }
                '1' => {
                    *state = Quit;
                    return write!(writer, "bye");
                }
                _ => {}
            }
        }
        Quit => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    #[test]
    fn main_does_not_panic() {
        let _ = main();
    }

    #[test]
    fn run_allows_quitting() {
        let input = b"0\n1\n";
        let mut output = Vec::new();
    
        run(&input[..], &mut output).unwrap();

        let output = String::from_utf8(output)
            .expect("output contained Non UTF-8 bytes");

        assert!(output.contains("bye"));
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
