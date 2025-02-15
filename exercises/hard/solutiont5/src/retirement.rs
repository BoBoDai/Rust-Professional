use std::cmp;
use std::str::FromStr;

pub fn retire_time(time: &str, tp: &str) -> String {
    let retreat_age =
        if tp == "男职工" {
            60
        } else if tp == "原法定退休年龄55周岁女职工" {
            55
        } else if tp == "原法定退休年龄50周岁女职工" {
            50
        } else {
            return "".to_string();
        };
    let date: Vec<&str> = time.split("-").collect();
    let year = u32::from_str(date[0]).unwrap();
    let month = u32::from_str(date[1]).unwrap();
    let original_retreat_time = year + retreat_age;
    if original_retreat_time < 2025 {
        let retreat_date = format!("{}-{:02}", original_retreat_time, month);
        return format!("{},{},{}", retreat_date, retreat_age, 0);
    }

    let remaining_month = (original_retreat_time - 2025) * 12 + month;
    let late_month: u32 = if retreat_age == 60 || retreat_age == 55 {
        let late = (remaining_month as f64 / 4f64).ceil() as u32;
        let max_late = 36;
        cmp::min(max_late, late)
    } else {
        let late = (remaining_month as f64 / 2f64).ceil() as u32;
        let max_late = 60;
        cmp::min(max_late, late)
    };

    let late_age = retreat_age as f64 + late_month as f64 / 12f64;

    let month = month + late_month % 12;
    let year = original_retreat_time + (late_month) / 12 + if month > 12 { 1 } else { 0 };
    let late_date = format!("{}-{:02}", year, if month > 12 { month - 12 } else { month });

    if late_age.fract() == 0.0 {
        format!("{},{:.0},{}", late_date, late_age, late_month)
    } else {
        format!("{},{:.2},{}", late_date, late_age, late_month)
    }
}
