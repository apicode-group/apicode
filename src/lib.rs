pub mod model;
pub use model::postman::*;
pub mod plugins;

#[cfg(test)]
mod tests {
    // use super::*;

    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_java_lombok_dto_source_gen_async(
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut file = File::create("tests/LombokDtoDemoAsync.java").await?;
        // package
        file.write_all(b"package ren.shuaipeng.demo.domain;\n")
            .await?;
        // line
        file.write_all(b"\n").await?;
        // import
        file.write_all(b"import lombok.Data;\n").await?;
        file.write_all(b"import lombok.experimental.Accessors;\n")
            .await?;
        // line
        file.write_all(b"\n").await?;
        // class
        file.write_all(b"@Data\n").await?;
        file.write_all(b"public class LombokDtoDemo {\n").await?;
        // field
        file.write_all(b"\tprivate String id;\n").await?;
        file.write_all(b"\tprivate Integer age;\n").await?;
        file.write_all(b"\tprivate Instant createTime;\n").await?;
        file.write_all(b"}\n").await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_java_controller_source_gen_async(
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut file = File::create("tests/WebFluxControllerDemoAsync.java").await?;
        // package
        file.write_all(b"package ren.shuaipeng.demo.controller;\n")
            .await?;
        // line
        file.write_all(b"\n").await?;
        // import
        file.write_all(b"import org.springframework.web.bind.annotation.*;\n")
            .await?;
        file.write_all(b"import reactor.core.publisher.Flux;\n")
            .await?;
        file.write_all(b"import reactor.core.publisher.Mono;\n")
            .await?;
        // line
        file.write_all(b"\n").await?;
        // class
        file.write_all(b"@RestController\n").await?;
        file.write_all(b"@RequestMapping(\"tasks\")\n").await?;
        file.write_all(b"public class WebFluxControllerDemo {\n")
            .await?;
        // line
        file.write_all(b"\n").await?;
        // field
        // method get
        file.write_all(b"\t@GetMapping\n").await?;
        file.write_all(b"\tpublic Flux page(Integer page, Integer size) {\n")
            .await?;
        file.write_all(b"\t    return Flux.error(new Exception(\"please write your code\"));\n")
            .await?;
        file.write_all(b"\t}\n").await?;
        // line
        file.write_all(b"\n").await?;
        file.write_all(b"}\n").await?;

        Ok(())
    }

    use hyper::Client;
    use hyper_tls::HttpsConnector;
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn text() -> Result<(), Box<dyn std::error::Error>> {
        let mut number = 1;
        while number != 2 {
            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, hyper::Body>(https);

            let res = client
                .get("https://sso.yunaq.com/captcha/".parse()?)
                .await?;
            assert_eq!(res.status(), 200);
            let buf = hyper::body::to_bytes(res).await?;
            let mut file = File::create(format!("tests/demo_{}.png", number)).await?;
            file.write_all(&buf).await?;
            thread::sleep(Duration::from_secs(1));
            number += 1;
        }
        Ok(())
    }

    #[tokio::test]
    async fn text1() -> Result<(), Box<dyn std::error::Error>> {
        let mut number = 1;
        let mut file = File::create("tests/demo.txt").await?;
        while number != 2 {
            file.write_all(format!("{}\n", number).as_bytes()).await?;
            number += 1;
        }
        Ok(())
    }
}
