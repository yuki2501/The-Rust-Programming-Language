#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match coin{
        Coin::Quarter(state) => println!("State quarter from {:?}",state),
        _ => count += 1
    }
    //if let Coin::Quarter(state) = coin {
    //    println!("State quarter from {:?}!", state);
    //} else {
    //   count += 1;
    //}

}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
          println!("Lucky penny!");
          1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("State quarter from {:?}",state);
            25
        },
    }
}
