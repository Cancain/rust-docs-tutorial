//fn main() {
    //let mut string = String::from("Hello World");

    //let word = first_word(&string);

    //string.clear();

    //println!("{}", word);
//}

//fn first_word(s: &String) -> usize {
    //let bytes = s.as_bytes();

    //for (i, &item) in bytes.iter().enumerate() {
        //if item == b' ' {
            //return i;
        //}
    //}
    //s.len()
//}

//Intro to slices
//fn main() {
    //let s = String::from("Hello world");

    ////let hello = &s[0..5];
    //let hello = &s[..5];
    ////let world = &s[6..11];
    //let world = &s[6..];

    //println!("{} {}", hello, world);
//}

fn main() {
    let mut s = String::from("Hello World");
    println!("{}", first_word(&s));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s
}