#[derive(Debug, PartialEq)]
enum State {
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OLA")
    }
}

#[derive(Debug)]
enum Transition {
    Mushroom,
    Flower,
    Feather,
}

#[derive(Debug)]
struct Player {
    state: State,
}

impl Player {
    fn new() -> Self {
        Self {
            state: State::Mario,
        }
    }
    fn collect(&mut self, power: Transition) {
        match (&self.state, power) {
            (State::Mario, Transition::Mushroom) => self.state = State::SuperMario,
            (_, Transition::Flower) => self.state = State::FireMario,
            (_, Transition::Feather) => self.state = State::CapeMario,
            (_, Transition::Mushroom) => {}
        }
    }
}

fn main() {
    let mut dplayer = Player::new();
    dplayer.collect(Transition::Mushroom);
    dplayer.collect(Transition::Flower);
    dplayer.collect(Transition::Feather);
    dplayer.collect(Transition::Flower);
    assert!(dplayer.state == State::CapeMario);
    println!("{}", dplayer.state);
}
