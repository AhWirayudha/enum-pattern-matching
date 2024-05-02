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
}

fn route(ip_kind: IpAddrKind) {}
