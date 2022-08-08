use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .arg(Arg::new("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .min_values(1)
        )
        .arg(Arg::new("omit_newline")
            .short('n')
            .help("Do not print newline")
            .takes_value(false)
        )
        .get_matches();
    let text= matches
        .get_many::<String>("text")
        .map(|v| v.map(|v| v.to_owned()).collect::<Vec<_>>())
        .unwrap();
    let ending = match matches.is_present("omit_newline") {
        true => "",
        false => "\n"
    };
    print!("{}{}", text.join(" "), ending);
}
