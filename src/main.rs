#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate ddc;

#[macro_use]
extern crate human_panic;

use docopt::Docopt;

use ddc::short_version;

fn show_output(day: u32, month: u32, year: i32) {
	println!("\n\tFrode calendar converter");
	println!("\t---------------------------------");
	println!("{}",short_version(day, month, year));
}

const USAGE: &str = r#"
DiaDoCuringa

Enter with day month and year (separated by space)
Entre com dia mes e ano (separados por espaço)

Usage:
  ddc <day> <month> <year>
  ddc <dia> <mes> <ano>
"#;

#[derive(Debug, Deserialize)]
struct Args {
    arg_dia: Option<u32>,
    arg_mes: Option<u32>,
    arg_ano: Option<i32>,
}

fn main() {
    setup_panic!();
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
	let d = args.arg_dia.expect("Invalid day");
    let m = args.arg_mes.expect("Invalid month");
    let y = args.arg_ano.expect("Invalid year");

    show_output(d, m, y);
}
