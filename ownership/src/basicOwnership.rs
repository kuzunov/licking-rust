use String;

fn main() {
    // let mut s = String::from("hello"); // uses heap
    // s.push_str(", world!");
    // println!("{s}");
    // double_memory_drop();
} // when out of scope, drop() is called automatically to free allocated memory; RAII

// fn double_memory_drop() {
//     // let s1 = String::from("hello");
//     // // let s2 = s1;

//     // // println!("{s1}, world!"); // after reassign, s1 is no longer valid, i.e. only s2 calls drop() to free the heap value; s1 and s2 both point to the same heap location; string is pointer + len + capacity; pointer to heap; not a shallow copy (only pointer data copied; it's a move because s1 is invalidated;)

//     // let mut s = String::from("hello");
//     // s = String::from("ahoy"); // memory allocated to s that contains "hello" is dropped here, because nothing points to it; the string is out of scope and dropped

//     // println!("{s}, world!");

//     let x = 5;
//     let y = x;

//     println!("x = {x}, y = {y}"); //ints and similar types that have known memory footprint are always 'deep copied'; always cloned on stack; no diff there between this and .copy()
// }
