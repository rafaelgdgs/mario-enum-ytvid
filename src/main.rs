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

fn game_menu() -> std::result::Result<u8, String> {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    match user_input.as_str() {
        "0\n" => {
            println!("Options: 1-Mushroom 2-Flower 3-Feather 4-Damage q-Leave");
            Ok(0)
        }
        "1\n" => Ok(1),
        "2\n" => Ok(2),
        "3\n" => Ok(3),
        "4\n" => Ok(4),
        "q\n" | "Q\n" => Ok(255),
        _ => Err("Valor Invalido!".to_string()),
    }
}

fn run_game(player: &mut Player) {
    println!("Press '0' to show menu");
    loop {
        match game_menu() {
            Ok(value) => match value {
                255 => {
                    println!("Leaving ...");
                    break;
                }
                1 => player.collect(Transition::Mushroom),
                2 => player.collect(Transition::Flower),
                3 => player.collect(Transition::Feather),
                4 => player.collect(Transition::Damage),
                0 => {}
                _ => {}
            },
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}

fn main() {
    let mut dplayer = Player::new();
    run_game(&mut dplayer);
    // dplayer.collect(Transition::Mushroom);
    // dplayer.collect(Transition::Flower);
    // dplayer.collect(Transition::Feather);
    // dplayer.collect(Transition::Mushroom);
    // dplayer.collect(Transition::Mushroom);
    // dplayer.collect(Transition::Mushroom);
    // dplayer.collect(Transition::Damage);
    // dplayer.collect(Transition::Damage);
    // dplayer.collect(Transition::Flower);
    // dplayer.collect(Transition::Damage);
    // dplayer.collect(Transition::Damage);
    // dplayer.collect(Transition::Damage);
    // dplayer.collect(Transition::Damage);
    // dplayer.collect(Transition::Feather);
    // dplayer.collect(Transition::Damage);
    // assert!(dplayer.state == State::CapeMario);
    println!("Last state: {:?}", dplayer.state);
}
