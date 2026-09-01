enum UsState {
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1819,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin{
        if state.existed_in(1900) {
            Some(format!("{State:?}"))
        } else {
            Some(format!("{state:?}"))
        }
    } else {
        None
    }
}

fn main() {
    println!("Hello, world!");
}
