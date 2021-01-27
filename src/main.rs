#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let width = 30;
    let height = 50;

    let rectangle_tuple = (width, height);
    let rectangle_struct = Rectangle { width, height};

    println!("The area of the rectangle is {} square pixels.", area(width, height));
    println!("The area of the rectangle is {} square pixels.", tuple_area(rectangle_tuple));
    println!("The area of the rectangle is {} square pixels.", struct_area(&rectangle_struct));

    println!("The rectangle is {:?}", rectangle_struct);
    println!("The rectangle is {:#?}", rectangle_struct);
}
