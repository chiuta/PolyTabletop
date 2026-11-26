//! `polytabletop-rpg` — interactive tabletop-RPG toolkit.
//!
//!   polytabletop-rpg                 interactive prompt
//!   polytabletop-rpg roll 4d6kh3     run a single command and exit

use polytabletop_rpg::cli::Session;
use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut s = Session::new_from_clock();
    if !args.is_empty() {
        match s.exec(&args.join(" ")) {
            Ok(o) => print!("{}", o.text),
            Err(e) => {
                eprintln!("eroare: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    println!("PolyTabletop-RPG — trusă generică pentru jocuri de rol. Scrie `help`, `q` iese.");
    let stdin = io::stdin();
    loop {
        print!("rpg> ");
        let _ = io::stdout().flush();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
        match s.exec(&line) {
            Ok(o) => {
                print!("{}", o.text);
                if o.quit {
                    break;
                }
            }
            Err(e) => println!("eroare: {e}"),
        }
    }
}
