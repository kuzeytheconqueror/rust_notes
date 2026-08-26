enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind)
{}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

fn main() {

    let adres1:IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let adress2:IpAddr = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("::1"),
    };

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    println!("Hello, world!");
}
