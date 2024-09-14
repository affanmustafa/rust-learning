#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width: i32 = 30;
    let height: i32 = 50;
    let rectangle1: (i32, i32) = (10, 20);
    let rectangle_struct: Rectangle = Rectangle {
        width: 20,
        height: 10,
    };
    let rectangle_struct_2: Rectangle = Rectangle {
        width: 3,
        height: 5,
    };
    println!(
        "rectangle1 can hold rectangle_struct {}",
        rectangle_struct.can_hold(&rectangle_struct_2)
    );
    println!(
        "The area of the rectangle via method is {}",
        rectangle_struct.area()
    );
    println!(
        "The area of the rectangle is {}",
        calculate_area(width, height)
    );
    println!(
        "The area of the rectanlge with tuple is {}",
        calculate_area_tuple(rectangle1)
    );
    println!(
        "The area of struct rectangle is {}",
        calculate_are_struct(&rectangle_struct)
    );
    println!("Rectangle Struct is {:#?}", rectangle_struct);
}

fn calculate_area(width: i32, height: i32) -> i32 {
    width * height
}

fn calculate_area_tuple(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

// We use & here to reference and borrow the value of the struct. That way, we don't have to copy
// the value of the struct and main can still use it after the function call.
fn calculate_are_struct(rectangle: &Rectangle) -> i32 {
    rectangle.width * rectangle.height
}
