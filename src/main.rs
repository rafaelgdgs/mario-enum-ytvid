#[derive(Debug, PartialEq)]
enum State {
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
    Dead,
}

#[derive(Debug, PartialEq)]
enum TransitionProperty {
    Revive,
    None,
}

#[derive(Debug)]
enum Transition {
    Mushroom,
    Flower,
    Feather,
    Damage,
}

impl Transition {
    fn property(&self) -> TransitionProperty {
        match self {
            Transition::Mushroom => TransitionProperty::None,
            Transition::Flower => TransitionProperty::None,
            Transition::Damage => TransitionProperty::None,
            Transition::Feather => TransitionProperty::Revive,
        }
    }
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
            (State::Dead, power) if power.property() == TransitionProperty::Revive => {
                self.state = State::Mario
            }
            (State::Dead, _) => {}
            (State::Mario, Transition::Damage) => self.state = State::Dead,
            (State::SuperMario, Transition::Damage) => self.state = State::Mario,
            (_, Transition::Damage) => self.state = State::SuperMario,
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
    dplayer.collect(Transition::Damage);
    // assert!(dplayer.state == State::CapeMario);
    println!("{:?}", dplayer.state);
}
