// struct User{
//     name: String,
//     age: u32,
//     active: bool,
// }
use std::fs;

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

enum Direction{
    North,
    South,
    East,
    West,
}

struct Rect{
    width: u32,
    height: u32,
}

impl Rect{
    fn area(&self) -> u32 {
        return self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height)
    }
    fn debug() -> &'static str {
        "Debugging Rect struct"
    }
}

enum Shape{
    Circle(f64),
    Rectangle(f64,f64),
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
    println!("debug is {}", Rect::debug());

    let polygon_area = NoShape::area();
    println!("Area of no shape: {}", polygon_area);

    let mydirection = Direction::North;
    let new_direction = mydirection; //No error because Direction is Copy type
    move_around(new_direction);


    let rect = Shape::Rectangle(5.0,2.0);
    calculate_area(rect);
    let circle = Shape::Circle(3.0);
    calculate_area(circle);


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


fn calculate_area(shape:Shape)->f64{
    let area = match shape{
        Shape::Rectangle(a,b)=>a*b,
        Shape::Circle(r)=>3.14*r*r,
    };
    return area;
}