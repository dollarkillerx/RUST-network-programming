use std::net::{IpAddr,Ipv4Addr,Ipv6Addr,SocketAddr};
use std::str::FromStr;

fn main() {
    ip_addr_test();
    println!();
    socket_addr_test();
}

fn ip_addr_test() {
    let v4 = IpAddr::V4(Ipv4Addr::new(127,0,0,1));
    let v6 = IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,1));
    let _a = IpAddr::from_str("8.8.8.8");

    assert_eq!("127.0.0.1".parse(),Ok(v4));
    assert_eq!("::1".parse(),Ok(v6));

    assert_eq!(v4.is_loopback(),true); // 判断是不是环回地址
    assert_eq!(v4.is_multicast(),false); // 判断是不是广播地址
    assert_eq!(v4.is_ipv4(),true); // 判断是不是广播地址
    assert_eq!(v6.is_ipv6(),true); // 判断是不是广播地址
}

fn socket_addr_test() {
    // ip + port
    let v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 8080);
    let v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 8080);

    let vv = SocketAddr::from_str("0.0.0.0:8081").unwrap();

    println!("v4: {}", v4);
    println!("v6: {}", v6);
    println!("vv: {}",vv);
}