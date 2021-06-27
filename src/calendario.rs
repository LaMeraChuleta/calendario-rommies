use prettytable::{ Table, Row, Cell };
use chrono::{ Datelike, Local, NaiveDate, Weekday };
const WEEKDAY_SPANISH: [&'static str; 7] = [
    "LUNES", 
    "MARTES", 
    "MIERCOLES", 
    "JUEVES", 
    "VIERNES", 
    "SABADO", 
    "DOMINGO"
];
pub fn create_calendar_month(date_for_calendar: Option<NaiveDate>){
    
    let mut table_activity_month = Table::new();
    //layout tabla actividades
    create_header_calendar(&mut table_activity_month);    
    //Body tabla actividades
    create_body_calendar(&mut &mut table_activity_month, date_for_calendar);
    table_activity_month.printstd();
}
fn create_header_calendar(table_activity: &mut Table) {
    table_activity.set_titles(Row::new(vec![
        Cell::new("CALENDARIO DE ACTIVIDADES").style_spec("H7")]));    

    let mut cells_weekday: Vec<Cell> = vec![];    
    WEEKDAY_SPANISH.iter().for_each(|&week| {
        cells_weekday.push(Cell::new(week));        
    });
    table_activity.add_row(Row::new(cells_weekday));
}
fn create_body_calendar(table_activity: &mut Table, date_for_calendar: Option<NaiveDate>){

    let day_now = match date_for_calendar {
        Some(date) =>  date,
        None => {
            let now = Local::now();
            NaiveDate::from_ymd(now.year(), now.month(), now.day())
        }
    };    
    let count_days = count_days_in_month(day_now);

    let index_weekday = match NaiveDate::from_ymd(day_now.year(), day_now.month(), 1).weekday()  {
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 7          
    };

    let mut cells_weekday: Vec<Cell> = vec![];    
    
    for _day in 0..count_days { 
        let day = _day + 1;               
        if day == 1 {            
            for _ in 1..index_weekday {
                cells_weekday.push(Cell::new(""));
            }
            cells_weekday.push(Cell::new(day.to_string().as_str()));
        }
        else {
            if cells_weekday.len() < 7 {
                cells_weekday.push(Cell::new(day.to_string().as_str()));
            }
            else {                
                table_activity.add_row(Row::new(cells_weekday.clone()));
                cells_weekday.clear();
                cells_weekday.push(Cell::new(day.to_string().as_str()));
            }
        }
    }

    for _ in 1..(7 - cells_weekday.len()){
        cells_weekday.push(Cell::new(""));
    }
    table_activity.add_row(Row::new(cells_weekday.clone()));
}
fn count_days_in_month(day_now: NaiveDate) -> i64{
    let year_month = match day_now.month() {
        12 => (day_now.year() + 1, 1),
        _ => (day_now.year(), day_now.month() + 1)
    };    
    NaiveDate::from_ymd(year_month.0, year_month.1, 1)
        .signed_duration_since(NaiveDate::from_ymd(day_now.year(), day_now.month(), 1))
        .num_days()
}
pub fn parse_name_month_to_number(month_name: &str) -> Option<NaiveDate>{

    let number_month: i32 = match month_name {
        "ene" => 1,
        "feb" => 2,
        "mar" => 3,
        "abr" => 4,
        "may" => 5,
        "jun" => 6,
        "jul" => 7,
        "ago" => 8,
        "sep" => 9,
        "oct" => 10,      
        "nov" => 11,
        "dic" => 12,
        _ => -1
    };

    if number_month.is_positive() {
        let day_now = Local::now();        
        return Some(NaiveDate::from_ymd(day_now.year(),number_month as u32, 1));
    }
    None    
}