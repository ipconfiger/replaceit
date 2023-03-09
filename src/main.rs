use std::path::PathBuf;
use std::fs::read_to_string;
use clap::{App, Arg};


fn main() {
    let matches = App::new("Replace it")
    .version("0.1")
    .author("Alexander.Li")
    .about("Replace variable from template")
    .arg(Arg::with_name("tempfile")
            .help("The path to the template file")
            .required(true)
            .index(1))
    .arg(Arg::with_name("variables")
            .short('v')
            .long("vars")
            .value_name("VARS")
            .help("Sets variables to replace")
            .multiple(true)
            .takes_value(true))
        .get_matches();
    let file_path = matches.value_of("tempfile").unwrap();
    let path = PathBuf::from(file_path);
    let template_txt = read_to_string(path).expect("File not exist!");
    let mut rp_result = template_txt;
    if let Some(values) = matches.values_of("variables") {
        for value in values {
            let mut sp = value.splitn(2, "=");
            let key = sp.next().unwrap().to_uppercase();
            let val = sp.next().unwrap();
            let replace_key = format!("${{{}}}", key);
            rp_result = rp_result.replace(replace_key.as_str(), val);
        }
    }
    println!("{}", rp_result);
}
