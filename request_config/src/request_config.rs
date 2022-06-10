#[allow(non_camel_case_types)]
pub enum Protocol
{
    ftp,
    http,
    https,

    INVALID,
}

impl Protocol
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            Protocol::ftp => "ftp".to_string(),
            Protocol::http => "http".to_string(),
            Protocol::https => "https".to_string(),
            Protocol::INVALID => "INVALID".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Option<Protocol>
    {
        match s
        {
            "ftp" => Some(Protocol::ftp),
            "http" => Some(Protocol::http),
            "https" => Some(Protocol::https),
            "" => None,
            _ => Some(Protocol::INVALID),
        }
    }
}

pub struct RequestConfig
{
    protocol: Option<Protocol>,
    website: String,
    filename: String,
}

impl RequestConfig
{

    fn parse_protocol(url: &String) -> Option<Protocol>
    {
        let parts: Vec<&str> = url.split("://").collect();
        if parts.len() <= 1 { // no protocol or no url at all
            return None;
        }
        let protocol_name: &str = *parts.get(0).unwrap();

        Protocol::from_str(protocol_name)
    }

    fn cut_protocol(url: &String) -> String
    {
        match url.find("://")
        {
            None => url.to_string(), // url unchanged
            Some(pos) => String::from(&url[pos + 3..]),
        }
    }

    fn parse_website_and_filename(url: &String) -> (String, String)
    {
        let slash_pos = match url.find('/')
        {
            None => return (url.to_string(), "/".to_string()),
            Some(pos) => pos
        };

        let website = (&url[..slash_pos]).to_string();
        let filename = (&url[slash_pos..]).to_string();

        (website, filename)
    }

    pub fn new(url: String) -> RequestConfig
    {
        let protocol = Self::parse_protocol(&url);
        let url = Self::cut_protocol(&url);
        let (website, filename) = Self::parse_website_and_filename(&url);
        RequestConfig { protocol, website, filename }
    }

    pub fn get_website(&self) -> &String
    {
        &self.website
    }

    pub fn get_filename(&self) -> &String
    {
        &self.filename
    }

    pub fn get_protocol(&self) -> Option<&Protocol>
    {
        match &self.protocol
        {
            Some(prot_ref) => Some(prot_ref),
            None => None,
        }
    }
}