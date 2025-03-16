fn main() {
    let s = String::from("hello"); //s comes into scope
    takes_ownership(s); // s's value moves into the function and is no longer valid

    let x = 5; //x comes into scope;

    makes_copy(x); //i32 implements the Copy trait and does not move into function, it's copied, and can be used
    println!("{}", x)
} // x is out of scope, then s, but s is moved to the fn

fn takes_ownership(str: String) {
    // str comes into scope
    println!("{str}")
} // str goes out of scope here and drop is called, memory is freed

fn makes_copy(num: i32) {
    // num comes into scope
    println!("{num}");
} //int goes out of scope, but its copied, so nothing special happens
