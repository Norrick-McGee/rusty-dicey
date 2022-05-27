use clap::{App, Arg};

fn main() {
    // Args
    let table_arg = Arg::new("table")
        .long("table")
        .takes_value(false)
        .help("you want to view this roll as a table?");

    let odds_arg = Arg::new("odds")
        .long("odds")
        .takes_value(true)
        .help("get the odds of a specific roll (or maybe higher?)");

    let roll_arg = Arg::new("roll")
        .takes_value(true)
        .help("how many dice are you rolling? (2d6? mayber 1d20 & 2d4)")
        .required(true);

    // init clapApp with Args
    let app = App::new("Rusty-Dicey")
        .version("0.1.0")
        .about("Rolls some dice")
        .author("Norrick McGee")
        .arg(table_arg)
        .arg(odds_arg)
        .arg(roll_arg);

    let matches = app.get_matches();

    let roll = matches.value_of("roll").expect("No Roll in app matches");
    let table_mode = matches.is_present("table");
    let odds_mode = matches.is_present("odds");
}
