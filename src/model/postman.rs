use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Postman {
    info: Info,
    pub item: Vec<Item>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    _postman_id: String,
    name: String,
    schema: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub item: Option<Vec<Item>>,
    pub request: Option<Request>,
    pub response: Option<Vec<Response>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub method: String,
    pub header: Vec<Header>,
    pub url: Url,
    pub query: Option<Vec<Query>>,
    pub body: Option<Body>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    mode: String,
    raw: String,
    options: BodyRaw,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BodyRaw {
    raw: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    key: String,
    name: Option<String>,
    value: String,
    #[serde(alias = "type")]
    htype: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Url {
    raw: String,
    host: Vec<String>,
    path: Vec<String>,
    query: Option<Vec<Query>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    key: String,
    value: String,
    disabled: Option<bool>,
    description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub name: String,
    #[serde(alias = "originalRequest")]
    original_request: Request,
    #[serde(alias = "_postman_previewlanguage")]
    language: Option<String>,
    cookie: Vec<String>,
    header: Option<Header>,
    body: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_postman_json() {
        // 解析JSON
        let contents = fs::read_to_string("tests/data/demo.postman_collection.json")
            .expect("Something went wrong reading the file");
        let postman: Postman = serde_json::from_str(&contents).unwrap();
        let collection: Option<&Vec<Item>> = postman.item[0].item.as_ref();
        for folder in collection.iter() {
            // 迭代文件夹
            for folder in folder.iter() {
                // 请求
                for req in folder.item.as_ref().iter() {
                    let tmp_req = &req[0];
                    assert_eq!(tmp_req.name, "学员列表");
                    let request: Option<&Request> = tmp_req.request.as_ref();
                    let response: Option<&Vec<Response>> = tmp_req.response.as_ref();
                    assert_eq!(response.unwrap()[0].name, "OK");
                    assert_eq!(request.unwrap().method, "GET");
                }
            }
        }
    }
}
