// structures

#[derive(Debug)]
struct FriendOfMine {
    name: String,
    age: u8,
    is_alive: bool,
}

struct RGBColor(u8, u8, u8);

fn new_friend(name: String, age: u8) -> FriendOfMine {
    FriendOfMine {
        name,
        age,
        is_alive: true,
    }
}

impl FriendOfMine {
    fn new(name: String, age: u8) -> Self {
        new_friend(name, age)
    }

    fn describe(&self) -> String {
        let description = format!("{} is {} years old.", self.name, self.age);
        println!("{}", description);
        description
    }
    fn dies(&mut self) {
        self.is_alive = false;
    }
}

fn main() {
    let new_friend = FriendOfMine::new(String::from("John"), 22);
    let second_friend = FriendOfMine::new(new_friend.name.clone(), new_friend.age);

    println!("{:?}", second_friend);
    new_friend.describe();

    let mut another_friend = FriendOfMine {
        name: String::from("Jane"),
        ..new_friend
    };
    println!(
        "{} is {} years old",
        another_friend.name, another_friend.age
    );

    another_friend.dies();

    let red = RGBColor(255, 0, 0);
    let green = RGBColor(0, 255, 0);
    let blue = RGBColor(0, 0, 255);

    for color in [red, green, blue].iter() {
        println!("R: {}, G: {}, B: {}", color.0, color.1, color.2);
    }
}
