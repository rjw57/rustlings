// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

#[derive(Debug)]
enum Message {
    Quit = 1,
    Echo = 2,
    Move = 3,
    ChangeColor = 4,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
