use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    fn laod_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);
        let content = fs::read_to_string(full_path);
        content.ok()
    }
}

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

pub struct StaticPageHandler;
impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        match &req.resource {
            http::httprequest::Resource::Path(s) => {
                let route: Vec<&str> = s.split("/").collect();
                match route[1] {
                    "" => HttpResponse::new("200", None, Self::laod_file("index.html")),
                    "health" => HttpResponse::new("200", None, Self::laod_file("health.html")),
                    path => match Self::laod_file(path) {
                        Some(content) => {
                            let mut map: HashMap<&str, &str> = HashMap::new();
                            if path.contains(".css") {
                                map.insert("Content-type", "text/css");
                            } else if path.contains(".js") {
                                map.insert("Content-type", "text/javascript");
                            } else {
                                map.insert("Content-type", "text/html");
                            }
                            HttpResponse::new("200", Some(map), Some(content))
                        }
                        None => HttpResponse::new("404", None, Self::laod_file("404.html")),
                    },
                }
            }
        }
    }
}
pub struct PageNotFoundHandler;

impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::laod_file("404.html"))
    }
}
pub struct WebServiceHandler;
impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_content = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> =
            serde_json::from_str(json_content.unwrap().as_str()).unwrap();
        orders
    }
}
impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        match &req.resource {
            http::httprequest::Resource::Path(s) => {
                let route: Vec<&str> = s.split("/").collect();
                match route[2] {
                    "shipping" if route.len() > 2 && route[3] == "orders" => {
                        let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                        let mut header: HashMap<&str, &str> = HashMap::new();
                        header.insert("Content-type", "application/json");
                        HttpResponse::new("200", Some(header), body)
                    }
                    _ => HttpResponse::new("404", None, Self::laod_file("404.html")),
                }
            }
        }
    }
}
