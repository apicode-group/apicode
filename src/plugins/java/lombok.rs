use std::fs::File;
use std::io::prelude::*;

pub fn create_code() {
    let mut file = File::create("tests/LombokDtoDemo.java").unwrap();
    // package
    file.write(b"package ren.shuaipeng.demo.domain;\n").unwrap();
    // line
    file.write(b"\n").unwrap();
    // import
    file.write(b"import lombok.Data;\n").unwrap();
    file.write(b"import lombok.experimental.Accessors;\n")
        .unwrap();
    // line
    file.write(b"\n").unwrap();
    // class
    file.write(b"@Data\n").unwrap();
    file.write(b"public class LombokDtoDemo {\n").unwrap();
    // field
    file.write(b"\tprivate String id;\n").unwrap();
    file.write(b"\tprivate Integer age;\n").unwrap();
    file.write(b"\tprivate Instant createTime;\n").unwrap();
    file.write(b"}\n").unwrap();
}
