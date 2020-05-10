use ipnet::{IpNet,Ipv4Net,Ipv6Net};
use std::net::{Ipv4Addr,Ipv6Addr};
use std::str::FromStr;

fn main() {
    // ipv4Addr + 前缀长度
    let v4 = Ipv4Net::new(Ipv4Addr::new(10,1,1,0),24).unwrap();

    // let ss4 = Ipv4Addr::from_str("10.1.1.0/24").unwrap();
    let ss6 = Ipv6Addr::from_str("fd00::/24").unwrap();

    let ss4: Ipv4Addr = "10.0.0.1".parse().unwrap();

    println!("v4: {}",v4);
    println!("ss4: {}",ss4);
    println!("ss6: {}",ss6);

    let net = IpNet::V4(Ipv4Net::from(ss4));

    println!("net {}, hostmask = {}",net,net.hostmask());
}
