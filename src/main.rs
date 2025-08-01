// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6,
//     }
//
//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }
//
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//
//     match home.kind {
//         IpAddrKind::V4 => {
//             println!("It's V4.");
//         }
//         IpAddrKind::V6 => {
//             println!("It's V6.");
//         }
//     }
// }

// fn main() {
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }
//
//     let home1 = IpAddr::V4(127, 0, 0, 1);
//
//     let home2 = IpAddr::V6(String::from("::1"));
//
//     match home2 {
//         IpAddr::V4(a,b,c,d) => {
//             println!("It's V4.");
//         }
//         IpAddr::V6(str) => {
//             println!("It's V6.");
//         }
//     }
// }

// fn main() {
//     enum Option<T> { // !!! it is defined by the standard library !!!
//         None,
//         Some(T),
//     }
//
//     let some_number = Option::Some(5);
//     let some_char = Option::Some('e');
//
//     let absent_number: Option<i32> = Option::None; // This is how Rust represents NULL
// }

// fn main() {
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);
//     // let sum = x + y;
//     let sum = x + y.unwrap();
//     println!("{}", sum);
// }

// fn main() {
//     fn plus_one(x: &Option<i32>) -> Option<i32> {
//         return match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         };
//     }
//
//     let five = Some(5);
//     println!("{:?}", five);
//     let six = plus_one(&five);
//     println!("{:?}", six);
//     let none = plus_one(&None);
//     println!("{:?}", none);
// }

// fn main() {
//     let a = String::from("123");
//     let k = match a.as_str() {
//         "Apple" => {
//             println!("It's Apple.");
//         }
//         _ => {
//             println!("It's something else, non-Apple.");
//         }
//     };
// }

// fn main() {
//     use std::fmt::Display;
//     fn printVal<T: Display>(opt: &Option<T>) {
//         let Some(key) = opt else {
//             println!("opt=None");
//             return;
//         };
//         println!("key={}", key);
//     }
//
//     let val = Some(123);
//     printVal(&val);
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = String::from("world");
//     let s3 = format!("{s1}, {s2}"); // only borrows
//     println!("{}", s3);
//     println!("{}", s2);
//     println!("{}", s1);
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let mut largest = &number_list[0];
//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//
//     println!("The first number is {}, and the largest is {}.", number_list[0], largest);
// }

// fn main() {
//     #[derive(Debug)]
//     struct Rectangle {
//         width: u32,
//         height: u32,
//     }
//
//     let mut list = [
//         Rectangle {
//             width: 10,
//             height: 1,
//         },
//         Rectangle {
//             width: 3,
//             height: 5,
//         },
//         Rectangle {
//             width: 7,
//             height: 12,
//         },
//     ];
//
//     list.sort_by_key(|r: &Rectangle| r.width);
//     println!("{:#?}", list);
// }

fn main() {
    println!("123");
}