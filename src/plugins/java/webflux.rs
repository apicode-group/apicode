use std::fs::File;
use std::io::prelude::*;

pub fn create_code() {
    let mut file = File::create("tests/WebFluxControllerDemo.java").unwrap();
    // package
    file.write(b"package ren.shuaipeng.demo.controller;\n")
        .unwrap();
    // line
    file.write(b"\n").unwrap();
    // import
    file.write(b"import org.springframework.web.bind.annotation.*;\n")
        .unwrap();
    file.write(b"import reactor.core.publisher.Flux;\n")
        .unwrap();
    file.write(b"import reactor.core.publisher.Mono;\n")
        .unwrap();
    // line
    file.write(b"\n").unwrap();
    // class
    file.write(b"@RestController\n").unwrap();
    file.write(b"@RequestMapping(\"tasks\")\n").unwrap();
    file.write(b"public class WebFluxControllerDemo {\n")
        .unwrap();
    // line
    file.write(b"\n").unwrap();
    // field
    // method get
    file.write(b"\t@GetMapping\n").unwrap();
    file.write(b"\tpublic Flux page(Integer page, Integer size) {\n")
        .unwrap();
    file.write(b"\t    return Flux.error(new Exception(\"please write your code\"));\n")
        .unwrap();
    file.write(b"\t}\n").unwrap();
    // line
    file.write(b"\n").unwrap();
    file.write(b"}\n").unwrap();
}
