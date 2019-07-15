fn main() {
    let some_u8_value = Some(0u8);

    ////With match
    //match some_u8_value {
        //Some(3) => println!("three"),
        //_ => (),
    //}

    //With if let
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
