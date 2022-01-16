enum State {
    A,
    B,
}

struct Machine {
    state: State,
}

impl Machine {
    fn transit(&mut self, input: char) {
        self.state = match self.state {
            State::A => match input {
                'A'..='Z' | 'a'..='z' => State::B,
                _ => panic!("Invalid input!"),
            },
            State::B => match input {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => State::B,
                _ => panic!("Invalid input!"),
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
}
