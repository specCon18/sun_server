use axum::{
    routing::get,
    Router, Json, response::IntoResponse,
};
use chrono::{DateTime, Utc, Timelike, Datelike, Duration};
use clap::Parser;
use serde_json::json;

#[cfg(test)]
mod tests;

struct Time {
    hour: u32,
    minute: u32,
    second: u32,
}

impl Time {
    fn new(offset_hours: i32) -> Self {
        let now = Utc::now() + Duration::hours(offset_hours.into());
        Time {
            hour: now.hour(),
            minute: now.minute(),
            second: now.second(),
        }
    }
}

struct Date {
    day: u32,
    month: u32,
    year: i32,
}

impl Date {
    fn new(offset_hours: i32) -> Self {
        let now = Utc::now() + Duration::hours(offset_hours.into());
        Date {
            day: now.day(),
            month: now.month(),
            year: now.year(),
        }
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

    let app = Router::new()
        .route("/", get(schedule(cli.offset)));

    let addr = "0.0.0.0:3000".parse().unwrap();
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn schedule(offset: i32) -> impl Fn() -> Json<Value> + Clone {
    move || {
        let date = Date::new(offset);
        let season = get_season(&date);
        let dst = is_dst(&date);
        let curve = select_curve(&season, dst);
        Json(curve)
    }
}

fn get_season(date: &Date) -> String {
    match date.month {
        12 | 1 | 2 => "winter",
        3 | 4 | 5 => "spring",
        6 | 7 | 8 => "summer",
        9 | 10 | 11 => "fall",
        _ => unreachable!(),
    }.to_string()
}

fn is_dst(date: &Date) -> bool {
    (date.month > 3 && date.month < 11) || (date.month == 3 && date.day >= 14) || (date.month == 11 && date.day < 7)
}

fn select_curve(season: &str, is_dst: bool) -> Value {
    // Example JSON for winter, implement similarly for other seasons
    let curve_json = match (season, is_dst) {
        ("winter", _) => json!({
            "morning": {"temp": [2000,3000], "brightness": [10,60], "time": [6,8]},
            "daytime": {"temp": [5000,6500], "brightness": [60,100], "time": [8,16]},
            "evening": {"temp": [4000,3000], "brightness": [60,30], "time": [16,20]},
            "night": {"temp": [2700,1800], "brightness": [30,5], "time": [20,22]},
            "nocturn": {"temp": [1800,1800], "brightness": [5,5], "time": [22,6]},
        }),
        ("spring", _) => json!({
            "morning": {"temp": [2500,3500],"brightness": [20,70],"time": [6,8]},
            "daytime" : {"temp": [5500,6500],"brightness": [70,100],"time": [8,18]},
            "evening": {"temp": [4000,3000],"brightness": [70,40],"time": [18,20]},
            "night": {"temp": [2700,2000],"brightness": [40,10],"time": [20,22]},
            "nocturn": {"temp": [2000,2000],"brightness": [10,10],"time": [22,6]}
        }),
        ("spring", true) => json!({
            "morning": {"temp": [2500,3500],"brightness": [20,70],"time": [7,9]},
            "daytime" : {"temp": [5500,6500],"brightness": [70,100],"time": [9,19]},
            "evening": {"temp": [4000,3000],"brightness": [70,40],"time": [19,21]},
            "night": {"temp": [2700,2000],"brightness": [40,10],"time": [21,23]},
            "nocturn": {"temp": [2000,2000],"brightness": [10,10],"time": [23,7]}
        }),
        ("summer", true) => json!({
            "morning": {"temp": [3000,5000],"brightness": [30,100],"time": [7,9]},
            "daytime" : {"temp": [6500,6500],"brightness": [100,100],"time": [9,19]},
            "evening": {"temp": [4000,3000],"brightness": [100,50],"time": [19,22]},
            "night": {"temp": [2700,2200],"brightness": [50,5],"time": [22,23]},
            "nocturn": {"temp": [2200,2200],"brightness": [5,5],"time": [23,7]}
        }),
        ("fall", _) => json!({
            "morning": {"temp": [2500, 3500],"brightness": [20, 70],"time": [6, 8]},
            "daytime": {"temp": [5500, 6500],"brightness": [70, 100],"time": [8, 18]},
            "evening": {"temp": [4000, 3000],"brightness": [70, 30],"time": [18, 20]},
            "night": {"temp": [2700, 2000],"brightness": [30, 10],"time": [20, 22]},
            "nocturn": {"temp": [2000,2000],"brightness": [10,10],"time": [22,6]}
        }),

        ("fall", true) => json!({
            "morning": {"temp": [2500, 3500],"brightness": [20, 70],"time": [7,9]},
            "daytime": {"temp": [5500, 6500],"brightness": [70, 100],"time": [9,19]},
            "evening": {"temp": [4000, 3000],"brightness": [70, 30],"time": [19,21]},
            "night": {"temp": [2700, 2000],"brightness": [30, 10],"time": [21,23]},
            "nocturn": {"temp": [2000,2000],"brightness": [10,10],"time": [23,7]}
        }),
        _ => json!({}),
    };

    curve_json
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
