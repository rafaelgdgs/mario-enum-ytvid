use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum State {
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
    Dead,
}

#[derive(Debug, PartialEq, Eq, Hash)]
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
    fn property(&self) -> HashSet<TransitionProperty> {
        let mut transitions: HashSet<TransitionProperty> = HashSet::new();

        match self {
            Transition::Mushroom => transitions.insert(TransitionProperty::None),
            Transition::Flower => transitions.insert(TransitionProperty::None),
            Transition::Damage => transitions.insert(TransitionProperty::None),
            Transition::Feather => transitions.insert(TransitionProperty::Revive),
        };
        transitions
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
        #[cfg(debug_assertions)]
        print!("Consuming {:?}, ", &power);

        match (&self.state, power) {
            (State::Dead, power) if power.property().contains(&TransitionProperty::Revive) => {
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

        #[cfg(debug_assertions)]
        println!("Current state: {:?}", self.state);
    }
}

fn main() {
    let mut dplayer = Player::new();
    dplayer.collect(Transition::Mushroom);
    dplayer.collect(Transition::Flower);
    dplayer.collect(Transition::Feather);
    dplayer.collect(Transition::Mushroom);
    dplayer.collect(Transition::Mushroom);
    dplayer.collect(Transition::Mushroom);
    dplayer.collect(Transition::Damage);
    dplayer.collect(Transition::Damage);
    dplayer.collect(Transition::Flower);
    dplayer.collect(Transition::Damage);
    dplayer.collect(Transition::Damage);
    dplayer.collect(Transition::Damage);
    dplayer.collect(Transition::Damage);
    dplayer.collect(Transition::Feather);
    dplayer.collect(Transition::Damage);
    // assert!(dplayer.state == State::CapeMario);
    println!("{:?}", dplayer.state);
}
