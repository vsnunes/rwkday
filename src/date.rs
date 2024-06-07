pub fn weekday(year: u16, month: u8, day: u8) -> f32 {
    let month_partial: f32 = f32::from((13 * (month + 1)) / 5).floor();
    let year_century: f32 = f32::from(year % 100);
    let zero_based_century: f32 = year_century.floor();
    let weekday: f32 = (f32::from(day)
        + month_partial
        + year_century
        + (year_century / 4.0).floor()
        + (zero_based_century / 4.0).floor()
        - 2.0 * zero_based_century)
        % 7.0;

    //Convert weekday to ISO week day
    return ((weekday + 5.0) % 7.0) + 1.0;
}
