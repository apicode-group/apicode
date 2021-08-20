#[macro_use]
extern crate clap;

use std::fs;

pub mod model;
pub use model::postman::*;

fn main() {
    // parse
    let matches = clap_app!(myapp =>
        (version: "0.0.1")
        (author: "niaoshuai <niao.shuai123@163.com>")
        (about: "api code generate")
        (@arg CONFIG: -c --config +takes_value "postman json")
        (@arg gen: +required +empty_values "generate code")
        (@subcommand java =>
            (about: "gennerate java code")
            (version: "0.0.1")
            (author: "niaoshuai <niao.shuai123@163.com>")
            (@arg framework: -f --framework +required +takes_value "choose framework（webflux，webmvc）")
            (@arg package: -p --package +takes_value "set package (e.g. com.demo)")
        )
        (@subcommand typescript =>
            (about: "gennerate typescript code")
            (version: "0.0.1")
            (author: "niaoshuai <niao.shuai123@163.com>")
            (@arg framework: -f --framework +required +takes_value "choose framework（ice）")
            (@arg dir: -d --dir +required +takes_value "choose dir (e.g. page)")
        )
    ).get_matches();

    let config = matches.value_of("CONFIG");
    println!("The config passed is: {:?}", config);

    // parse config
    let contents =
        fs::read_to_string(config.unwrap()).expect("Something went wrong reading the file");
    let _postman: Postman = serde_json::from_str(&contents).unwrap();

    let gen = matches.value_of("gen");
    println!("The gen passed is: {:?}", gen);

    if let Some(sub_java) = matches.subcommand_matches("java") {
        let _package_str = sub_java.value_of("package");
        let _framework_str = sub_java.value_of("dir");
    }

    if let Some(sub_java) = matches.subcommand_matches("typescript") {
        let package_str = sub_java.value_of("package");
        let framework_str = sub_java.value_of("dir");
        println!(
            "The subcommands passed is: {:?} {:?}",
            package_str, framework_str
        );
    }
}
