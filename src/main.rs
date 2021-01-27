#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (other.height <= self.height) && (other.width <= self.width)
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
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
    let rectangle_struct = Rectangle { width, height };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        tuple_area(rectangle_tuple)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        struct_area(&rectangle_struct)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle_struct.area()
    );

    println!("The contents of rectangle are: {:?}", rectangle_struct);
    println!("The contents of rectangle are: {:#?}", rectangle_struct);

    let rectangle_a = Rectangle {
        width: 30,
        height: 50,
    };
    let rectangle_b = Rectangle {
        width: 10,
        height: 40,
    };
    let rectangle_c = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "It is {} that rectangle A can hold rectangle B.",
        rectangle_a.can_hold(&rectangle_b)
    );
    println!(
        "It is {} that rectangle A can hold rectangle C.",
        rectangle_a.can_hold(&rectangle_c)
    );

    let square_a = Rectangle::square(30);
    let square_b = Rectangle::square(40);
    println!(
        "it is {} that square A can hold square B, and {} that square B can hold square A.",
        square_a.can_hold(&square_b),
        square_b.can_hold(&square_a)
    );
}
