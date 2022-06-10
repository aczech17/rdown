use request_config::request_config::*;
use net_utils::get_request_address;


#[allow(unused_variables)]
fn main()
{
    let url = "http://www.forum-jagiellonii.pl/\
    viewtopic.php?t=2373&sid=a0bff4cfedc7a059ab4424cb54ab5315&start=780".to_string();

    let rq = RequestConfig::new(url);

    let ra = get_request_address(&rq);
    println!("{}", ra);
}
