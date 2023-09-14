

pub struct State {
    current_line: usize,
    no_error_mod: bool
}

impl State{
    
    pub fn new() -> Self{
        State { current_line: 0, no_error_mod: false }
    }
}