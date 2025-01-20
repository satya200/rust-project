// This file explain about the enums
#[derive(Debug)]

enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

enum IpAddrEn {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
	 println!("Inside Message method");
	 //println!("val = {}", self.Write);
    }
}

fn main() {
    // Below home is called instance
    let home = IpAddr {
	kind: IpAddrKind::V4(String::from("IPV4")),
	addr: String::from("192.168.1.1"),
    };
    // Below loopback is called instance
    let loopback = IpAddr {
	kind: IpAddrKind::V6(String::from("IPV6")),
	addr: String::from("127.0.0.1"),
    };
    println!("home addr = {}\n ip = {:?}",home.addr, home.kind);
    println!("home addr = {}\n ip = {:?}",loopback.addr, loopback.kind);
    // TODO: How to print enum value. This is done usng match we can print below is the example
    let home1 = IpAddrEn::V4(127,0,0,1);
    let home2  = IpAddrEn::V6(String::from("127.0.0.1"));
    //println!("home2 = {:?}", home2);// We can not print enum. we can do only compare like below
    // match arms having 2 parts one is code which is mention :: and other is return value =>
    // Below syntax is same as if but if return type is boolean but match return type is anything
    match home1 {
        IpAddrEn::V4(ref ip, ip1, ip2, ip3) => println!("V4 ip address: {}.{}.{}.{}", ip, ip1, ip2, ip3),
        IpAddrEn::V6(ref str) => println!("V6 ip address = {}", str),
    }
    let m = Message::Write(String::from("Hello"));
    m.call();
    
}
