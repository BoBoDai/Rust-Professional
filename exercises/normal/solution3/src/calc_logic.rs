pub fn new_birthday_probability(n: u32) -> f64 {
    if n < 2 {
        panic!("Place input more than 2 people")
    }
    let mut brith_not_in_one_days = 1.0;
    let days = 365.0;
    for i in 0..n {
        brith_not_in_one_days *= (days - f64::from(i)) / days;
    }
    1.0 - brith_not_in_one_days
}
