fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );


    let rect1 = (30,50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tup(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );

    println!("single line: The rectangle was {:?}", rect2); // single line
    println!("mutli line: The rectangle was {:#?}", rect2); // multi line

    // finally `dbg!` macro
    dbg!(&rect1);
    dbg!(&rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)] // this allows for Rectangles to be printed to stdout using `:?` or `:#?`
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}