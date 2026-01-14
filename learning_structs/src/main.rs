// struct User{
//     name: String,
//     age: u32,
//     active: bool,
// }
use std::fs;
struct Rect{
    width: u32,
    height: u32,
}

struct Point<T>{
    x: T,
    y: T, //can U be different type
}
enum Result<T,E>{
    Ok(T),
    Err(E),
}


struct NoShape;

impl NoShape{
    fn area() -> u32 {
        return 0
    }
}

impl Rect{
    fn area(&self) -> u32 {
        return self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height)
    }
}

enum Direction{
    North,
    South,
    East,
    West,
}

fn main() {
    println!("Hello, world!");
    // let user1 = User {
    //     name: String::from("Alice"),
    //     age: 30,
    //     active: true,
    // };

    let rect = Rect {
        width: 10,
        height: 20,
    };
    println!("Area of the rectangle: {}", rect.area());
    println!("Perimeter of the rectangle: {}", rect.perimeter());

    let polygon_area = NoShape::area();
    println!("Area of no shape: {}", polygon_area);

    let mydirection = Direction::North;
    let new_direction = mydirection;
    move_around(new_direction);


    //Error Handling with Structs
    let res = fs::read_to_string("example.txt");
    match res{
        Ok(content)=>println!("File content: {}", content),
        Err(e)=>println!("Error reading file: {}", e),
    }
}

fn move_around(_dir: Direction){
    //
}
