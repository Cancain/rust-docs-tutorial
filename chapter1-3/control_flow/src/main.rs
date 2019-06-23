// fn main() {
//     let number: i32 = 7;

//     if number < 5 {
//         println!("Condition was true");
//     } else {
//         println!("Condition was false");
//     }
// }


// fn main() {
//     let number = 7;

//     if number != 0{
//         println!("There is a number");
//     }   
// }


// fn main() {
//     let number = 123;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3 or 2");
//     }
// }


// fn main() {
//     let condition = false;
//     let number = if condition {
//         5
//     } else {
//         6
//     };

//     println!("The value of number is: {}", number);
// }

// fn main() {
//     let mut counter = 0;
    
//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {}", result);    
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number -= 1;
//     }

//     println!("LIFTOFF!!")
// }


// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("The value is: {}", a[index] );

//         index += 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a.iter() {
//         println!("The value is: {}", element );
//     }
// }

fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}


