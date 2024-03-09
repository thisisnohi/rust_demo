use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(value: &str) -> Method {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String)
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version : Version,
    pub resource : Resource,
    pub headers : HashMap<String, String>,
    pub msg_body : String
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body: String = "".to_string();

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);

            } else if line.len() == 0 {
                println!("line is empty")
            } else {
                 parsed_msg_body = line.to_string();
            }
        }
        
        HttpRequest{
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body,
        }
    }
}

fn process_header_line(req: &str) -> (String, String) {
    let mut items = req.split(":");
    let mut key = String::from("");
    let mut value: String = String::from("");

    if let Some(s) = items.next() {
        key = s.to_string();
    }

    if let Some(s) = items.next() {
        value = s.to_string();
    }

    (key, value)

}

fn process_req_line(req: &str) -> (Method, Resource, Version) {
    let mut words = req.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use super::*;

    #[test]
    fn test_method_into () {
        let method = Method::from("Get");
        assert_eq!(method, Method::Get);
    }

    #[test]
    fn test_version_into () {
        let version : Version = "HTTP/1.1".into();
        assert_eq!(version, Version::V1_1);
    }

    #[test]
    fn test_read_into () {
       let s : String = String::from("Get /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent:curl 1111\r\nAccept: */*\r\nthis is msg");

       let mut headers_expected = HashMap::new();
       headers_expected.insert("Host".into(), " localhost".into());
       headers_expected.insert("Accept".into(), " */*".into());
       headers_expected.insert("User-Agent".into(), "curl 1111".into());

       let req : HttpRequest = s.into();
        
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_ascii_lowercase()), req.resource);
        assert_eq!(headers_expected, req.headers);
        
        

    }

}