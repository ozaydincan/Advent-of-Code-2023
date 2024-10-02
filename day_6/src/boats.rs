pub struct Boats {
    pub button_time: Vec<i64>,
}

pub trait ChargeBoat {
    fn charge_button(&mut self, distance: &i64, total_time: &i64);
}

impl ChargeBoat for Boats {
    fn charge_button(&mut self, distance: &i64, total_time: &i64) {
        self.button_time.push(quadratic_diff(total_time, distance));
    }
}

pub fn quadratic_diff(time: &i64, distance: &i64) -> i64 {
    let b = -(*time) as f64;
    let c = *distance as f64;
    let delta = (b * b - 4.0 * c).sqrt();

    let r1 = ((-b + delta) / 2.0).ceil() - 1.0;
    let r2 = ((-b - delta) / 2.0).floor() + 1.0;

    let diff = (r1 - r2).abs() + 1.0;
    diff as i64
}

pub fn read_input(time_list: &str, distance_list: &str) -> Vec<(i64, i64)> {
    let input: Vec<(i64, i64)> = time_list
        .split_ascii_whitespace()
        .zip(distance_list.split_ascii_whitespace())
        .map(|(time, distance)| {
            (
                time.parse::<i64>().unwrap(),
                distance.parse::<i64>().unwrap(),
            )
        })
        .collect();
    input
}

pub fn input_2(time_list: &str, distance_list: &str) -> Vec<(i64, i64)> {
    let time = time_list
        .replace(" ", "")
        .parse::<i64>()
        .expect("wrong time input");
    let distance = distance_list
        .replace(" ", "")
        .parse::<i64>()
        .expect("wrong distance input");
    let new_input: Vec<(i64, i64)> = vec![(time, distance)];
    new_input
}
