mod calendario;
use getopts::{self, Options, Occur, HasArg};
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    println!("{}", opts.usage(&brief));
}
fn main() {    

    let args: Vec<String> = env::args().collect();
    let program  = args[0].clone();
    
    //Declaracion de opciones
    let mut opts = Options::new();
    opts.optflag("a", "ayuda", "lista de opciones de la interfaz");
    opts.opt("c", "calendario", "muestra el calendario actual", "MES AÑO", HasArg::Maybe, Occur::Multi);
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => { std::panic::panic_any(e.to_string()) }
    };

    if matches.opt_present("a") {
        print_usage(&program, opts);        
        return;
    } 
    if matches.opt_present("c") {        
        match matches.opt_str("c") {
            Some(param) => {
                match calendario::parse_name_month_to_number(&param){
                    Some(date) => calendario::create_calendar_month(Some(date)),
                    None => return
                }
            }
            None => calendario::create_calendar_month(None)
        }
        return;
    }
}
