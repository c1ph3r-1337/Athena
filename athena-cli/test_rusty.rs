use rustyline::completion::{Completer, Pair};
use rustyline::hint::Hinter;
use rustyline::highlight::Highlighter;
use rustyline::validate::Validator;
use rustyline::Helper;

struct MyHelper;
impl Completer for MyHelper {
    type Candidate = Pair;
    fn complete(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        Ok((0, vec![]))
    }
}
impl Hinter for MyHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> { None }
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
}
