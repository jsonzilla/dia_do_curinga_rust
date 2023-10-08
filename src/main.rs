#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate ddc;

#[macro_use]
extern crate human_panic;

use docopt::Docopt;

use ddc::{long_version, short_version};

fn show_output(day: i32, month: i32, year: i32) {
	println!("\n\tCalendario de Paciencia de Frode");
	println!("\t---------------------------------");
	println!("{}",long_version(day, month, year));
	println!("\n\tSimples -- {}", short_version(day, month, year));
}

const USAGE: &str = "
DiaDoCuringa

O calendário de Frode
Entre com dia mes e ano (separados por espaço)

Usage:
  ddc <dia> <mes> <ano>
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_dia: Option<i32>,
    arg_mes: Option<i32>,
    arg_ano: Option<i32>,
}

fn main() {
    setup_panic!();
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
	let d = args.arg_dia.expect("Dia inválido");
    let m = args.arg_mes.expect("Mes inválido");
    let y = args.arg_ano.expect("Ano inválido");

    show_output(d, m, y);
}
