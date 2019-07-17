mod lib;
use::rand::Rng;

//Multiple imports from same lib
use std::{cmp::Ordering, io};
use std::io::{self, Write};

//Global operator, imports all
use std::collections::*;

fn main() {
   lib::eat_at_resturant();
   let number = rand::thread_rng().gen_range(1,101);
   println!("number is: {}", number);
}