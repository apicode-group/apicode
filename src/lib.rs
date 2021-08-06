use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Postman {
    info: Info,
    item: Vec<Item>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Info {
    _postman_id: String,
    name: String,
    schema: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Item {
    name: String,
    item: Option<Vec<Item>>,
    request: Option<Request>,
    response: Option<Vec<Response>>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Request {
    method: String,
    header: Vec<Header>,
    url: Url,
    query: Option<Vec<Query>>,
    body: Option<Body>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Body {
    mode: String,
    raw: String,
    options: BodyRaw,
}
#[derive(Serialize, Deserialize, Debug)]
struct BodyRaw {
    raw: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Header {
    key: String,
    name: Option<String>,
    value: String,
    #[serde(alias = "type")]
    htype: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Url {
    raw: String,
    host: Vec<String>,
    path: Vec<String>,
    query: Option<Vec<Query>>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Query {
    key: String,
    value: String,
    disabled: Option<bool>,
    description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
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
    // use std::fs::*;
    // use std::io::Write;

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

    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_java_source_gen() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut file = File::create("tests/demo.java").await?;
        // package
        // import
        // class
        file.write_all(b"@Data\n").await?;
        file.write_all(b"public class Demo {\n").await?;
        // field
        file.write_all(b"\tprivate String id;\n").await?;
        file.write_all(b"\tprivate Integer age;\n").await?;
        file.write_all(b"\tprivate Instant createTime;\n").await?;
        file.write_all(b"}").await?;

        Ok(())
    }

    use hyper::Client;
    use hyper_tls::HttpsConnector;

    #[tokio::test]
    async fn text() -> Result<(), Box<dyn std::error::Error>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let res = client.get("https://sso.yunaq.com/captcha/".parse()?).await?;
        assert_eq!(res.status(), 200);
        let buf = hyper::body::to_bytes(res).await?;
        let mut file = File::create("tests/demo.png").await?;
        file.write_all(&buf).await?;
        Ok(())
    }
}
