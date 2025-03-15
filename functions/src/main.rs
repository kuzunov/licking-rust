fn main() {
    println!("Hello, world!");

    another_fn(5, 's');
    print!(" and ");
    another_fn(6, 'h');
    println!("");
    // this is an expression that evaluates at 4
    let y = {
        let x = 3;
        x + 1 //if there's a ; here, it will become a statement and will not return a value
    };

    println!("The value of y is: {y}");

    let x = return_type();
    let z = plus_one(x);

    println!("The value of typed fn is: {x}, z is {z}");
}

fn another_fn(x: i32, label: char) {
    print!("{x}{label}")
}

fn return_type() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
