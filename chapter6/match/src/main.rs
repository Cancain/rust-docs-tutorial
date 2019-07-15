//#[derive(Debug)]
//enum UsState {
    //Alabama,
    //Alaska,
//}

//enum Coin {
    //Penny, 
    //Nickel,
    //Dime,
    //Quarter(UsState),
//}

//fn value_in_cents(coin: Coin) -> u8 {
    //match coin {
        //Coin::Penny => {
            //println!("Lucky penny!");
            //1
        //},
        //Coin::Nickel => 5,
        //Coin::Dime => 10,
        //Coin::Quarter(state) => {println!("State quarter from {:?}!", state );
            //25        
        //}
    //}
//}

//fn main() {
    //let coin = Coin::Quarter(UsState::Alabama);
    //println!("{} worth", value_in_cents(coin))
//}

//fn plus_one(x: Option<i32>) -> Option<i32> {
    //match x {
        //None => {
            //println!("Nothing");
            //None
        //},
        //Some(i) => {
            //println!("{}", i);
            //Some(i + 1)
        //},
    //}
//}
//fn main() {
    //let five = Some(5);
    //let six = plus_one(five);
    //let none = plus_one(None);
//}

fn main() {
    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("One"),
        3 => println!("Three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("Something else..."),

    }
}