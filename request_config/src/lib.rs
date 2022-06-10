extern crate core;

pub mod request_config;

#[cfg(test)]
mod request_config_tests
{
    use crate::request_config::{RequestConfig, Protocol};

    fn get_tested_strings(url: &str) -> (String, String, String)
    {
        let url = String::from(url);
        let request = RequestConfig::new(url);

        let (protocol, website, filename) =
            (request.get_protocol(), request.get_website(), request.get_filename());

        let protocol_name = match protocol
        {
            Some(Protocol::ftp) => "ftp",
            Some(Protocol::http) => "http",
            Some(Protocol::https) => "https",
            Some(Protocol::INVALID) => "INVALID",
            None => "",
        };

        (protocol_name.to_string(), website.to_string(), filename.to_string())
    }

    #[test]
    fn no_protocol_url()
    {
        let url = "www.90minut.pl/news/320/news3200020-Rafal-Wolsztynski-w-Chrobrym.html";
        let result = get_tested_strings(url);

        assert_eq!(result,
                   (
                       "".to_string(),
                       "www.90minut.pl".to_string(),
                    "/news/320/news3200020-Rafal-Wolsztynski-w-Chrobrym.html".to_string(),
                   ) );
    }

    #[test]
    fn https_url()
    {
        let url = "https://stackoverflow.com/questions/64362934/how-to-disable-camel-case-warnings";
        assert_eq!(get_tested_strings(url), (
                "https".to_string(),
                    "stackoverflow.com".to_string(),
                    "/questions/64362934/how-to-disable-camel-case-warnings".to_string(),

        ));
    }

    #[test]
    fn http_url()
    {
        let url = "http://www.forum-jagiellonii.pl/";
        assert_eq!(get_tested_strings(url), (
            "http".to_string(),
            "www.forum-jagiellonii.pl".to_string(),
            "/".to_string()
            ) );
    }

    #[test]
    fn ftp_url()
    {
        let url = "ftp://janpawel2.com";
        assert_eq!(get_tested_strings(url),
                   ("ftp".to_string(),
                   "janpawel2.com".to_string(),
                   "/".to_string())
        );
    }

    #[test]
    fn empty_url()
    {
        let url = "";
        assert_eq!(get_tested_strings(url), ("".to_string(), "".to_string(), "/".to_string()));
    }
}