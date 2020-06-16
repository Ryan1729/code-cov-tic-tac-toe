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

#[derive(Clone, Copy)]
enum Space {
    Blank,
    X,
    O,
}
use Space::*;

const LENGTH: usize = 9;
type Board = [Space; LENGTH];

enum ProgramMove {
    GameIsOver,
    Move(usize, bool),
}
use ProgramMove::*;

fn program_move(board: Board) -> ProgramMove {
    enum Player {
        X,
        O,
    }

    fn spaces_are_line(s1: Space, s2: Space, s3: Space) -> bool {
        match (s1, s2, s3) {
            (X, X, X) | (O, O, O) => true,
            _ => false,
        }
    }

    fn helper(board: Board, player: Player) -> ProgramMove {
        if {
            spaces_are_line(board[0], board[1], board[2]) 
            || spaces_are_line(board[3], board[4], board[5])
            || spaces_are_line(board[6], board[7], board[8])
            || spaces_are_line(board[0], board[3], board[6])
            || spaces_are_line(board[1], board[4], board[7])
            || spaces_are_line(board[2], board[5], board[8])
            || spaces_are_line(board[0], board[4], board[8])
            || spaces_are_line(board[2], board[4], board[6])
        } {
            return GameIsOver;
        }

        let blank_indicies: Vec<_> = board
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                if let Blank = s { Some(i) } else { None }
            })
            .collect();

        if blank_indicies.len() == 0 {
            GameIsOver
        } else {
            let mut potential_moves = std::collections::VecDeque::with_capacity(
                LENGTH
            );

            for i in blank_indicies {
                let mut board_after_1_turn = board.clone();

                board_after_1_turn[i] = match player {
                    Player::X => X,
                    Player::O => O,
                };

                let other_player = match player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                };

                match helper(board_after_1_turn, other_player) {
                    Move(_, true) => {
                        // probably a bad move
                        potential_moves.push_back(i);
                    }
                    GameIsOver => {
                        // always take a win.
                        return Move(i, true);
                    }
                    Move(_, false) => {
                        // possibly a good move
                        potential_moves.push_front(i);
                    }
                }
            }

            assert!(potential_moves.len() > 0);

            Move(potential_moves.pop_front().unwrap(), false)
        }
    }

    helper(board, Player::O)
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

    let mut board: Board;
    let mut state: State;

    macro_rules! clear_board {
        () => {
            board = [Blank; LENGTH];
            state = Game;
        }
    }
    clear_board!();

    let mut buffer = String::new();

    macro_rules! space_to_char {
        ($index: literal) => {{
            let index = $index;
            let space = board[$index];
            match space {
                Blank => match index {
                    0 => '1',
                    1 => '2',
                    2 => '3',
                    3 => '4',
                    4 => '5',
                    5 => '6',
                    6 => '7',
                    7 => '8',
                    8 => '9',
                    _ => '?'
                },
                X => 'X',
                O => 'O'
            }
        }}
    }

    macro_rules! write_board {
        () => {
            write!(&mut writer, 
r#"
+-----+
|{6}|{7}|{8}|
+-----+
|{3}|{4}|{5}|
+-----+
|{0}|{1}|{2}|
+-----+
"#, 
                space_to_char!(0),
                space_to_char!(1),
                space_to_char!(2),
                space_to_char!(3),
                space_to_char!(4),
                space_to_char!(5),
                space_to_char!(6),
                space_to_char!(7),
                space_to_char!(8),
            )?;
        }
    }

    write!(&mut writer, "Enter 0 at any time to quit.\n\nEnter the number corresponding to the space to place an X there.")?;

    loop {
        match state {
            Game => {
                write_board!();
            }
            _ => { // ConfirmQuit or invalid enum
                write!(
                    &mut writer, 
                    "Are you sure you want to quit? (Enter 1 for yes).\n"
                )?;
            }
        }
        writer.flush()?;

        reader.read_line(&mut buffer)?;

        if let Some(c) = buffer.chars().next() {
            match state {
                Game => {
                    macro_rules! handle_selection {
                        ($index: literal) => {{
                            let index = $index;
                            match board[index] {
                                Blank => {
                                    board[index] = X;

                                    match program_move(board) {
                                        GameIsOver => {
                                            write_board!();
                                            write!(
                                                &mut writer,
                                                "You win! Congrats!\n"
                                            )?;
                                            clear_board!();
                                        }
                                        Move(i, did_win) => {
                                            board[i] = O;

                                            if did_win {
                                                write_board!();
                                                write!(
                                                    &mut writer,
                                                    "I win!\n"
                                                )?;
                                                clear_board!();
                                            }
                                        }
                                    }
                                }
                                X => {
                                    write!(
                                        &mut writer,
                                        "You already played there!\n"
                                    )?;
                                }
                                O => {
                                    write!(
                                        &mut writer,
                                        "I already played there!\n"
                                    )?;
                                }
                            }
                        }}
                    }

                    match c {
                        '0' => {
                            state = ConfirmQuit;
                        }
                        '1' => handle_selection!(0),
                        '2' => handle_selection!(1),
                        '3' => handle_selection!(2),
                        '4' => handle_selection!(3),
                        '5' => handle_selection!(4),
                        '6' => handle_selection!(5),
                        '7' => handle_selection!(6),
                        '8' => handle_selection!(7),
                        '9' => handle_selection!(8),
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
    fn run_allows_playing_each_space_then_quitting() {
        macro_rules! play {
            ($input: expr) => {
                let input = $input;
                let mut output = Vec::new();
            
                run(&input[..], &mut output).unwrap();
        
                quit_assert!(output);
            }
        }
        play!(b"1\n0\n1\n");
        play!(b"2\n0\n1\n");
        play!(b"3\n0\n1\n");
        play!(b"4\n0\n1\n");
        play!(b"5\n0\n1\n");
        play!(b"6\n0\n1\n");
        play!(b"7\n0\n1\n");
        play!(b"8\n0\n1\n");
        play!(b"9\n0\n1\n");
    }

    #[test]
    fn run_terminates_on_an_initial_read_error() {
        let mut output = Vec::new();
    
        let _ = run(FailingReader, &mut output);
    }

    #[test]
    fn space_is_cloneable() {
        let _ = Space::Blank.clone();
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
