use axum::{
    routing::get,
    Router,
};
use chrono::prelude::*;

struct Time {
    hour: u32,
    minute: u32,
    second: u32
}

impl Time {
    fn new() -> Time {
        let now = Utc::now();
        // Extract hour, minute, and second directly using chrono's functions
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();

        Time { hour, minute, second }
    }
}

struct Date {
    day: u32,
    month: u32,
    year: i32
}

impl Date {
    fn new() -> Date {
        let now = Utc::now();
        let day = now.day();
        let month = now.month();
        let year = now.year();

        Date { day,month,year}
    }
}

#[derive(Parser)]
#[command(version = "1.0", about = "An Axum application with timezone offset handling", long_about = None)]
struct Cli {
    /// Offset for timezone adjustment in hours
    #[arg(short, long)]
    offset: i32,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Access the offset from the CLI
    let offset_value = cli.offset;
    // build our application with a single route
    let app = Router::new()
        .route("/", get(schedule));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn schedule() -> String {
    let utc: DateTime<Utc> = Utc::now();
    let human = utc.to_rfc3339();
    // Extract the date part from the RFC 3339 formatted string
    // RFC 3339 format: YYYY-MM-DDTHH:MM:SS+00:00, so we take the first 10 characters
    let date_only = &human[..10]; // This slices the string to get the first 10 characters (YYYY-MM-DD)
    date_only.to_string() // Convert the slice back into a String to return
}

fn get_season(date:Date) -> String {
    let season = match date.month {
        12 | 1 | 2 => "winter",
        3 | 4 | 5 => "spring",
        6 | 7 | 8 => "summer",
        9 | 10 | 11 => "fall",
        _ => "Improper Month"
    };
    return season.to_string()
}
fn select_curve(season: &str, is_dst:bool) {
    //curves spring = 1 summer = 2 fall = 3 winter = 4
    let current_season:u16 = match season {
        "spring" => 1,
        "summer" => 2,
        "fall" => 3,
        "winter" => 4,
        _ => 999
    };
    let dst:u16 = match is_dst {
        true => 10,
        false => 20
    };
    let curve_selector = current_season+dst;
    let curve:Vec<u32> = match curve_selector {
        11 => vec![],
        12 => vec![],
        13 => vec![],
        14 => vec![],
        21 => vec![],
        23 => vec![],
        _ => vec![],
    };
}
fn is_dst(date:Date) -> bool {
    if (date.month > 3 || (date.month == 3 && date.day >= 14)) && (date.month < 11 || (date.month == 11 && date.day < 7)){
        true
    }
    else {
        false
    }
}

fn handle_timezone(time:Time, offset:i8) -> Time{
    let mut hour:u32 = time.hour;
    let minute:u32 = time.minute;
    let second:u32 = time.second;
    if offset > 0 {
        hour = hour + offset as u32;
    }
    else {
        let offset = offset.abs() as u32;
        hour = hour - offset;
    }
    let tz_adjusted = Time {hour,minute,second};
    return tz_adjusted
}
