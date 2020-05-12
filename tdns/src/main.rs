use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::udp::UdpClientConnection;
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass,Name,RData,Record,RecordType};
use std::env;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("please input ./xxx 0.0.0.0:8081");
        std::process::exit(1);
    }

    let address = "8.8.8.8:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();

    let client = SyncClient::new(conn);

    let domain = Name::from_str(args[1].clone().as_str()).unwrap();

    let response: DnsResponse = client.query(&domain,DNSClass::IN,RecordType::A).unwrap();
    let answers: &[Record] = response.answers();

    if let &RData::A(ref ip) = answers[0].rdata() {
        println!("ip: {}",*ip);
    }else {
        println!("Not Data");
    }

}
