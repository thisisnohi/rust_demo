use std::io::Write;

use http::{
    httprequest::{self, HttpRequest},
    httpresponse::HttpResponse,
};
use super::handler::{Handler, PageNotFoundHandler,StaticPageHandler,WebServiceHandler};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream:&mut impl Write) -> () {
        match req.method {
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();

                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handler(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handler(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                println!("method is undefind");
                let resp: HttpResponse = PageNotFoundHandler::handler(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
