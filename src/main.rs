fn main() {
    let src = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("pow(3 * 93 * 10000 *z 749, 2)".to_string());

    match expr::eval(&src) {
        Ok(v) => println!("{v}"),
        Err(err) => {
            println!("{}", err.to_pretty_string(&src).unwrap());
        }
    };
}
