fn main() {
    // let data = "initial contents";
    // let s = data.to_string();

    // let s = "initial contents".to_string();
    // let data = "stuff";

    // let s = String::from(data);
    
    // let mut s = String::from("foo");
    // let s2 = "bar";
    // s.push_str(s2);

    // // println!("{}", s);
    // println!("{}", s2);
    
    // let mut s  = String::from("lo");
    // s.push('l');

    // println!("{}", s);

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("World!");
    // let s3 = s1 + &s2;
    // println!("{}", s3);

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // // let s = s1 + "-" + &s2 + "-" + &s3;
    // let s = format!("{}-{}-{}", s1,s2,s3);

    // println!("{}", s);

    // let len = String::from("Hello").len();

    // println!("{}", len);

    // let hello = "Здравствуйте";

    // let s = &hello[0..4];

    // println!("{}", s);

    // for c in "नमस्ते".chars() {
    //     println!("{}", c);
    // }

    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }

}
