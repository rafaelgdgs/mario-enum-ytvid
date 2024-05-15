#[derive(Debug, PartialEq)]
enum State {
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
    Dead,
}

#[derive(Debug)]
enum Transition {
    Mushroom,
    Flower,
    Feather,
    Damage,
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
            (State::Dead, _) => {}
            (State::Mario, Transition::Damage) => self.state = State::Dead,
            (State::SuperMario, Transition::Damage) => self.state = State::Mario,
            (State::Mario, Transition::Mushroom) => self.state = State::SuperMario,
            (_, Transition::Flower) => self.state = State::FireMario,
            (_, Transition::Feather) => self.state = State::CapeMario,
            (_, Transition::Mushroom) => {}
            (_, Transition::Damage) => self.state = State::SuperMario,
        }
    }
}

fn main() {
    let mut dplayer = Player::new();
    dplayer.collect(Transition::Mushroom);
    dplayer.collect(Transition::Flower);
    dplayer.collect(Transition::Feather);
    dplayer.collect(Transition::Flower);
    dplayer.collect(Transition::Damage);
    // assert!(dplayer.state == State::CapeMario);
    println!("{:?}", dplayer.state);
}
