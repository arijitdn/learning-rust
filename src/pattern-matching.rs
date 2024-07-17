enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let value = value_in_cents(Coin::Dime);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // OR
    if let Some(3) = some_value {
        println!("three");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None => None,
        Some(i) => Some(i+1),
        _ => None, // Default
    }
}