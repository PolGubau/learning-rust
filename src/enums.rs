enum Vehicle {
    Electric(Car),
    Gas(Car),
    Hybrid(Car),
}

struct Car {
    hp: i32,
    brand: String,
    model: String,
    cost: i32,
}

fn main() {
    let fav_num: Option<i8> = None;
    // let fav_num: Option<i8> = Some(7);

    match fav_num {
        Some(7) => println!("Lucky number 7!"),
        None => println!("You don't have a favorite number."),
        _ => println!("Your favorite number is not 7."),
        // Some(n) => println!("Your favorite number is {}", n),
    }
}
