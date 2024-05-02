#[derive(Debug)]
// enum ip
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // set enum value
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?} {:?}", four, six);
}
