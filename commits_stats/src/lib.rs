use std::collections::HashMap;

fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

// Day of the week for a date, Monday = 1 ... Sunday = 7.
fn weekday(year: i32, month: u32, day: u32) -> u32 {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if month < 3 { year - 1 } else { year };
    let w = (y + y / 4 - y / 100 + y / 400 + t[month as usize - 1] + day as i32) % 7;
    if w == 0 { 7 } else { w as u32 }
}

fn day_of_year(year: i32, month: u32, day: u32) -> u32 {
    let days = [31, if is_leap(year) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    days[..month as usize - 1].iter().sum::<u32>() + day
}

fn weeks_in_year(year: i32) -> u32 {
    let jan1 = weekday(year, 1, 1);
    if jan1 == 4 || (jan1 == 3 && is_leap(year)) { 53 } else { 52 }
}

fn iso_week(date: &str) -> Option<String> {
    let mut parts = date.get(..10)?.split('-');
    let year: i32 = parts.next()?.parse().ok()?;
    let month: u32 = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;

    let doy = day_of_year(year, month, day) as i32;
    let week = (doy - weekday(year, month, day) as i32 + 10) / 7;

    let (iso_year, iso_week) = if week < 1 {
        (year - 1, weeks_in_year(year - 1))
    } else if week as u32 > weeks_in_year(year) {
        (year + 1, 1)
    } else {
        (year, week as u32)
    };

    Some(format!("{}-W{}", iso_year, iso_week))
}

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut result = HashMap::new();
    for commit in data.members() {
        if let Some(week) = commit["commit"]["author"]["date"].as_str().and_then(iso_week) {
            *result.entry(week).or_insert(0) += 1;
        }
    }
    result
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut result = HashMap::new();
    for commit in data.members() {
        if let Some(login) = commit["author"]["login"].as_str() {
            *result.entry(login.to_owned()).or_insert(0) += 1;
        }
    }
    result
}
