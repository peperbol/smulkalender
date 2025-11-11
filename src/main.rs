use core::fmt;
use std::{fs, future::Future, ptr::read, sync::OnceLock};

use actix_web::{Error, HttpRequest, HttpResponse, Responder, get, http::header::ContentType, middleware::{self, Logger}, post, put, web};
use chrono::*;

use icalendar::*;
use reqwest::Client;
use serde::{Deserialize, Deserializer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .wrap(Logger::default())
            .app_data(web::PayloadConfig::new(1 * 1024 * 1024 * 1024))
            .wrap(middleware::NormalizePath::trim())
            .service(test)
            .service(get_calender)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/")]
async fn test(req: HttpRequest) -> impl Responder {
    if let Some(val) = req.connection_info().realip_remote_addr() {
        println!("Address {:?}", val);
    };
    "hier is niets"
}

static CLIENT: OnceLock<Client> = OnceLock::new();


#[get("/smullerijen.ics")]
async fn get_calender(req: HttpRequest) -> impl Responder {
    if let Some(val) = req.connection_info().realip_remote_addr() {
        println!("get_calender Address {:?}", val);
    }else{
    println!("get_calender");
    }

    let csv_content = get_sheet_data().await;
    let mut calendar = convert_to_calendar(csv_content).unwrap();

    HttpResponse::Ok()
        .content_type( "text/calendar")
        .body(calendar.done().to_string())
    
    // let mut my_calendar = Calendar::new().name("Salina en Pepijn in Mortsel").done();

    // // push(&mut my_calendar,"Pinenut",year, 0,1);

    //  my_calendar.done().to_string()
}
fn convert_to_calendar(csv_content: String)-> Result<Calendar, Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_reader(csv_content.as_bytes());

    let mut my_calendar = Calendar::new().name("Salina en Pepijn in Mortsel").done();

    for result in reader.records() {
        let record= result?;

        let pepijn_joins = record[2].eq("TRUE");

        let salina_joins = record[3].eq("TRUE");

        if pepijn_joins || salina_joins {
        
        let date_time = NaiveDateTime::parse_from_str(& format!("{} 18:00:00",&record[1]), "%a %d/%m/%Y %H:%M:%S")?;
        
        let message = match (pepijn_joins, salina_joins) {
            (true, true) => "Pepijn en Salina smullen mee!",
            (true, false) => "Pepijn smult mee",
            (false, true) => "Salina smult mee",
            (false, false) => "niemand smult mee",
        };

        my_calendar.push(
        Event::new()
            .summary(message)
            .location("Mortsel")
            .starts(date_time)
            .ends(date_time + Duration::hours(1)).done()
        );
    }
    }
    
    Ok(my_calendar)
}

async fn get_sheet_data() -> String {
    let doc_id = "1hXxSj2_yzoIiPC-RvPR9QWgiNFKdSvJ6Pic7gmbXdz8";
    let sheet_id = "0";
    let sheeturl = format!(
        "https://docs.google.com/spreadsheets/d/{0}/export?format=csv&id={0}&gid={1}",
        doc_id, sheet_id
    );

    let client = CLIENT.get_or_init(Client::new);
    let response = client.get(sheeturl).send().await;

    response.unwrap().text().await.unwrap()
}


// fn push(calendar:&mut Calendar, str:&str,year:i64, month:i64, week:i64)  {
//     calendar.push(
//         Event::new()
//             .summary(str)
//             .description(&format!("https://www.wurmpedia.com/index.php/{}",str))
//             .starts(WurmDate{
//                 minutes: 0,
//                 hours: 0,
//                 day: 0,
//                 week: week,
//                 month: month,
//                 years: year
//             }.to_real_day())
//             .ends(WurmDate{
//                 minutes: 0,
//                 hours: 0,
//                 day: 0,
//                 week: week+1,
//                 month: month,
//                 years: year
//             }.to_real_day()).done()
//         );
// }
