fn main() {
    let s1 = String::from("hello asd");
    let mut mut_s = String::from("mutate me");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
    // let mutable_ref_to_s2 = &mut mut_s; we can only have one mutable ref to a val; unless we create a new scope with {}
    {
        let mutable_ref_to_s_in_new_scope = &mut mut_s;
    }
    let mutable_ref_to_s = &mut mut_s; //if this is on top of the scope, it's still in scope and not dropped, so we can't create another ref in the inner scope;

    // let ref1 = &mut_s;
    // let ref2 = &mut_s;
    // let mut_ref_1 = &mut mut_s;
    //we cannot have both mutable and immutable refs to the same val

    change_ref(mutable_ref_to_s);
}
fn calculate_length(s: &String) -> usize {
    // s is a ref to a string;
    s.len()
    //when fn is complete, it's not dropped, because refs are not 'owned'
    //s is not mutable, unless explicitly stated
}
fn change_ref(s: &mut String) {
    s.push_str(", world");
}
// fn dangle() -> &String {
//     let s = String::from("hell0");
//     &s s goes out of scope, val is dropped and we cant return a ref to a dropped val
// }
fn no_dangle() -> String {
    let s = String::from("hell0");
    s //returning the val moves the ownership out of the scope, nothing is dropped
}
