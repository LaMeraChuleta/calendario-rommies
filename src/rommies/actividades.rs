use std::collections::HashMap;
use std::path::PathBuf;
use rustbreak::{ PathDatabase, deser::Yaml };
//use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
enum Rommies {
    Gera,
    Gali,
    Emi
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ActivityHome {
    done_for: Rommies,
    date: (i32, u32, u32),
    description: String
}

pub fn insert_activity_calendar(){
    let db = PathDatabase::<HashMap<i32, ActivityHome>, Yaml>::load_from_path_or_default(PathBuf::from( "junio.yaml")).unwrap();
    db.write(|db| {
        db.insert(3, ActivityHome {
                done_for: Rommies::Gera,
                date: (2021,6,1),
                description: String::from("Hola Mundo")
            }
        );
        println!("{:#?}",db);
    }).unwrap();

    db.save().unwrap();
}