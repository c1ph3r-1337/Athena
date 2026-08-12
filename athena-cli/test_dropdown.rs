use rustyline::completion::{Completer, Pair};
use rustyline::hint::Hinter;
use rustyline::highlight::Highlighter;
use rustyline::validate::Validator;
use rustyline::Helper;
use std::io::Write;

struct MyHelper;
impl Completer for MyHelper {
    type Candidate = Pair;
    fn complete(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        Ok((0, vec![]))
    }
}
impl Hinter for MyHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> {
        let mut stdout = std::io::stdout();
        write!(stdout, "\x1b[s\x1b[1B\x1b[J"); // Save cursor, move down 1, clear below
        if line.starts_with('/') {
            write!(stdout, "Dropdown option 1\nDropdown option 2");
        }
        write!(stdout, "\x1b[u"); // Restore cursor
        stdout.flush().unwrap();
        None
    }
}
impl Highlighter for MyHelper {}
impl Validator for MyHelper {
    fn validate(&self, ctx: &mut rustyline::validate::ValidationContext) -> rustyline::Result<rustyline::validate::ValidationResult> {
        Ok(rustyline::validate::ValidationResult::Valid(None))
    }
    fn validate_while_typing(&self) -> bool { false }
}
impl Helper for MyHelper {}

fn main() {
    let mut rl = rustyline::Editor::<MyHelper, rustyline::history::DefaultHistory>::new().unwrap();
    rl.set_helper(Some(MyHelper));
    rl.readline("> ");
}
