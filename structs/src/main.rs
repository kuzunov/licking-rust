#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // let rect1 = (30, 50);
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    // println!("The area is {} square px", area(&rect1));
    println!("rect1 is {rect1:#?}");
    dbg!(&rect1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
