use request_config::request_config::*;
use net_utils::tcp_port;
use tcp_port::TcpPort;
use dns_lookup::lookup_host;

fn get_request_address(rc: &RequestConfig) -> String
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

#[allow(unused_variables)]
fn main()
{
    let url = "http://www.forum-jagiellonii.pl/viewtopic.php?t=2373&sid=a0bff4cfedc7a059ab4424cb54ab5315&start=780".to_string();
    let rq = RequestConfig::new(url);

    let ra = get_request_address(&rq);
    println!("{}", ra);
}
