use getopts::{self, Options, Occur, HasArg};
use std::env;
mod rommies;
use rommies::calendario;

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
    opts.opt("c", "calendario", "muestra el calendario actual", "MES", HasArg::Maybe, Occur::Multi);
    opts.optflagopt ("i", "insertar", "insertar actividad en un dia del mes", "MES, DIA");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => { std::panic::panic_any(e.to_string()) }
    };

    if matches.opt_present("a") {
        print_usage(&program, opts);        
        return
    } 
    if matches.opt_present("c") {                
        match matches.opt_str("c") {            
            Some(param) => {                
                calendario::create_calendar_month(calendario::parse_name_month_to_date(&param));                    
            },
            None => {
                calendario::create_calendar_month(calendario::parse_name_month_to_date(""));  
            }  
        }        
    }
    if matches.opt_present("i") {        
        if let true = args[3..].len() == 2 {
            println!("{:?}",args);
        }                  
    }
}
