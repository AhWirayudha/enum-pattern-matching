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
}

fn route(ip_kind: IpAddrKind) {}
