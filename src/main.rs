use std::path::Path;

fn main() {
    let src = std::env::args()
        .skip(1)
        .fold(String::new(), |src, arg| format!("{src} {arg}"));

    if src.is_empty() {
        let bin_name = std::env::args().next().unwrap();
        let bin_name = Path::new(&bin_name).file_name().unwrap().to_str().unwrap();
        println!("Usage: {bin_name} <expression>");
        return;
    }

    match expr::eval(&src) {
        Ok(v) => println!("{v}"),
        Err(err) => {
            println!("{}", err.to_pretty_string(&src).unwrap());
        }
    };
}
