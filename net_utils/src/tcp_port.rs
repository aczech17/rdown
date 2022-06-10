extern crate request_config;
use request_config::request_config::Protocol;

pub struct TcpPort
{
    number: u16
}

impl TcpPort
{
    pub fn from_protocol(protocol: &Protocol) -> TcpPort
    {
        let number: u16 = match protocol
        {
            Protocol::ftp => 20, // 21?
            Protocol::http => 80,
            Protocol::https => 443,
            Protocol::INVALID => 0,
        };
        TcpPort{number}
    }

    pub fn as_string(&self) -> String
    {
        self.number.to_string()
    }

    pub fn as_u16(&self) -> u16
    {
        self.number
    }
}