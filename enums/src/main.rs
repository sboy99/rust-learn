mod enums;

use enums::direction::Direction;

fn main() {
    let direction = Direction::Up;
    match direction {
        Direction::Up => {
            println!("Up");
            println!("{:?}",direction)
        },
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right"),
    }
}
