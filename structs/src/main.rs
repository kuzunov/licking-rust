#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, another_rect: &Rectangle) -> bool {
        self.width > another_rect.width && self.height > another_rect.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    } //assosiated fn, methods that don't have self; often used for constructors, i.e. rectangle with the same w and h is a square; fn is namespaced and called with ::
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // let rect1 = (30, 50);
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(15 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let sq = Rectangle::square(40);
    // println!("The area is {} square px", area(&rect1));
    // println!("rect1 is {rect1:#?}");
    // dbg!(&rect1);
    println!("Area of rect is {} sq px", rect1.area());
    if rect1.width() {
        println!("Rect non-zero width: {}", rect1.width)
    }
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
