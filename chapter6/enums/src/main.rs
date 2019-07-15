//enum IpAddrKind {
    //v4,
    //v6,
//}

//fn main() {
    //let four = IpAddrKind::v4;
    //let six = IpAddrKind::v6;
    //route(four);
    //route(six);
//}

//fn route(ip_kind: IpAddrKind){
    
//}

//combining structs and enums
//enum IpAddrKind {
    //v4,
    //v6,
//}

//struct IpAddr {
    //kind: IpAddrKind,
    //adress: String
//}

//let home = IpAddr {
    //kind: IpAddrKind::v4,
    //adress: String::from("127.0.0.1")
//}

//let loopback = IpAddr {
    //kind: IpAddrKind::v6,
    //adress: String::from("::1")
//}

//Combining Strings in enums
//enum IpAddr {
    //v4(String),
    //v6(String),
//}

//let home = IpAddr::v4(String::from("127.0.0.1"));
//let loopback = IpAddr::v6(String::from("::1"));

//enum IpAddr {
    //v4(u8, u8, u8, u8),
    //v6(String)
//}

//let home = IpAddr::v4(127, 0, 0, 1);

//let loopback = IpAddr::v6(String::from("::1"));

//enum Message {
    //Quit,
    //Move {x: i32, y: i32},
    //Write(String),
    //ChangeColor(i32, i32, i32),
//}

//struct QuitMessage;
//struct MoveMessage {
    //x: i32,
    //y: i32,
//}
//struct WriteMessage(String);
//struct ChangeColorMessage(i32, i32, i32);

//impl Message {
    //fn Call(&self) {
        ////Method body here
    //}
//}

//let m Message::Write(String::from("Hello"));
//m.Call()

enum Option<T> {
    Some(T),
    None,
};

let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;

fn Main () {
    let x: i8 = 5;
    let y: Option<i8> = Some(5)
}