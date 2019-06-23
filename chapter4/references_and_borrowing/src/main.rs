//fn main() {
    //let s1 = String::from("Hello");

    //let len = calculate_length(&s1);

    //println!("The length of {} is {}", s1, len);
//}

//fn calculate_length(s: &String) -> usize {
    //s.len()
//}

//Cant cange unmut references
//fn main() {
    //let s = String::from("Hello");

    //change(&s);
//}

//fn change(some_string: &String){
    //some_string.push_str(", World");
//}

//fn main() {
    //let mut s = String::from("Hello");

    //change(&mut s);
    //println!("{}", s)
//}

//fn change(some_string: &mut String){
    //some_string.push_str(", World");
//}

//Can only change one mutable refecence in scope
//fn main() {
    //let mut s = String::from("hello");

    //let r1 = &mut s;
    //let r2 = &mut s;

    //println!("{}, {}", r1, r2);
//}

//Create a block-scope to allow for more mutable references
//fn main() {
    //let mut s = String::from("hello");

    //{
        //let r1 = &mut s;
    //}

    //let r2 = &mut s;

    //println!("{}", r2)
//}

//Cant reference same variable as unmutable and mutable insame scope
//fn main() {
    //let mut s = String::from("hello");

    //let r1 = &s;
    //let r2 = &s;
    //let r3 = &mut s;

    //println!("{}, {} and {}", r1, r2, r3)
//}

//can reference Variable if it's not under use
//fn main() {
    //let mut s = String::from("hello");

    //let r1 = &s;
    //let r2 = &s;

    //println!("{} and {}", r1, r2);

    //let r3 = &mut s;
    //println!("{}", r3);
//}

//Example of a dangling reference, dangle() returns a reference to variable that goes out of scope
//fn main() {
    //let reference_to_nothing = dangle();
//}

//fn dangle() ->&String {
    //let s = String::from("Hello");
    //&s
//}

fn main() {
    let string = no_dangle();

    println!("{}", string);
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}