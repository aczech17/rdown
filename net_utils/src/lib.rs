pub mod tcp_port;

extern crate dns_lookup;
use request_config::request_config::RequestConfig;
use tcp_port::TcpPort;
use dns_lookup::lookup_host;

pub fn get_request_address(rc: &RequestConfig) -> String
{
    let port = TcpPort::from_protocol(&rc.get_protocol().unwrap());
    let website = rc.get_website();

    let ip_vec = lookup_host(website).unwrap();
    let mut addr = String::new();
    for i in ip_vec
    {
        addr = addr + &i.to_string();
    }

    addr = addr + ":" + &port.as_string();
    return addr;
}