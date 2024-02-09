use axum::{
    routing::get,
    Router,
};
use chrono::prelude::*;

#[tokio::main]
async fn main() {
    
    // build our application with a single route
    let app = Router::new()
        .route("/values", get(values))
        .route("/schedule", get(schedule));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn values()-> String{
    let utc: DateTime<Utc> = Utc::now();
    let human = utc.to_rfc3339();
    // Extract the time part from the RFC 3339 formatted string
    // The format is YYYY-MM-DDTHH:MM:SS+00:00, so we start from index 11 to skip the date part and the 'T'
    let time_only = &human[11..19]; // This slices the string from 'HH:MM:SS+00:00' onwards
    time_only.to_string() // Convert the slice back into a String to return
}

async fn schedule() -> String {
    let utc: DateTime<Utc> = Utc::now();
    let human = utc.to_rfc3339();
    // Extract the date part from the RFC 3339 formatted string
    // RFC 3339 format: YYYY-MM-DDTHH:MM:SS+00:00, so we take the first 10 characters
    let date_only = &human[..10]; // This slices the string to get the first 10 characters (YYYY-MM-DD)
    date_only.to_string() // Convert the slice back into a String to return
}

/*
//Values
[ ] select curve based on date
[ ] determine weather or not were in DST
[ ] adjust for DST if needed
[ ] serve values as json
//Schedule
[ ] get daily schedule from list of schedules based on date
[ ] serve todays schedule based on date.
*/   
fn get_season(){
    //date vs date_range    
}
fn select_curve(){
    //spring
    //summer
    //fall
    //winter
}
// time = [hr: i32, min: i32, sec: i32]
fn handle_dst(time:Vec<i32>) {
}
fn get_daily_schedule(){
    //season_schedule +-DST
}