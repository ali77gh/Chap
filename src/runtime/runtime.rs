
use crate::executable_line::ExecutableLine;
use super::state::State;

pub struct Runtime{
    state: State
}

impl Runtime {

    pub fn new() -> Self {
        Runtime { state: State::new() }
    }

    pub fn run_lines(&mut self, lines: Vec<ExecutableLine>){

        // loop{
            // TODO
        // }
    }

    fn run_line(line: ExecutableLine){

    }
}

