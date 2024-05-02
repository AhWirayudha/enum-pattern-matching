// create enum type IpAddrKind
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
// enum with String value
#[derive(Debug)]
enum IpAddr_String {
    V4(String),
    V6(String),
}
// enum with different value
#[derive(Debug)]
enum IpAddr_different {
    V4(u8, u8, u8, u8),
    V6(String),
}

// create struct
#[derive(Debug)] // annotation of struct
struct IpAddr {
    kind: IpAddrKind, // field with enum type
    address: String,
}

struct IpAddrv4 {
    address: String,
}

struct IpAddrv6 {
    address: String,
}

// you can put struct in enum
enum IpAddrEnum {
    V4(IpAddrv4),
    V6(IpAddrv6),
}

// enum with variety of type
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum Message similar to this struct
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// method in enum
impl Message {
    fn call(&self) {}
}

// enum option, already defined by standard library you can use None or Some with Option::
// enum Option<T> {
//     None,
//     Some(T),
// }

// match in enum
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    // set enum value
    let four = IpAddrKind::V4; // value V4 with type enum IpAddrKind, separate by double colon
    let six = IpAddrKind::V6;

    println!("{:?} {:?}", four, six);

    // call function
    route(IpAddrKind::V4); // only accept value enum with type IpAddrKind

    // struct
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:#?} {:#?}", home, loopback);

    // enum with string value
    let home = IpAddr_String::V4(String::from("127.0.0.1"));
    let loopback = IpAddr_String::V6(String::from("::1"));

    println!("{:#?} {:#?}", home, loopback);

    // enum with different value
    let home = IpAddr_different::V4(127, 0, 0, 1);
    let loopback = IpAddr_different::V6(String::from("::1"));

    println!("{:#?} {:#?}", home, loopback);

    // enum impl, can call method
    let m = Message::Write(String::from("hello"));
    m.call();

    // enum option, Option<T> similar to generic? *standart library
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // non valid value sample, error
    // let x: i8 = 5; // different type enum cause error
    let y: Option<i8> = Some(5);

    //let sum = x + y;
    dbg!(y);
    dbg!(y.unwrap());

    // test coin
    let coin = Coin::Penny;
    let value = value_in_cents(coin);

    println!("{}", value);
}

fn route(ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
