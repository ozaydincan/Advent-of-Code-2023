pub struct Boats {
    button_time: Vec<i64>,
}

trait ChargeBoat {
    fn charge_button(&mut self, distance: &i64, total_time: &i64);
}

impl ChargeBoat for Boats {
    fn charge_button(&mut self, distance: &i64, total_time: &i64) {
        self.button_time.push(quadratic_diff(total_time, distance));
    }
}

fn quadratic_diff(time: &i64, distance: &i64) -> i64 {
    let b = -(*time) as f64;
    let c = *distance as f64;
    let delta = (b * b - 4.0 * c).sqrt();

    let r1 = ((-b + delta) / 2.0).ceil() - 1.0;
    let r2 = ((-b - delta) / 2.0).floor() + 1.0;

    let diff = (r1 - r2).abs() + 1.0;
    diff as i64
}

fn read_input(time_list: &str, distance_list: &str) -> Vec<(i64, i64)> {
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

fn input_2(time_list: &str, distance_list: &str) -> Vec<(i64, i64)> {
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

fn main() -> Result<(), std::io::Error> {
    let test_input = include_str!("races.txt");
    let mut boats = Boats {
        button_time: vec![],
    };
    let mut boats_2 = Boats {
        button_time: vec![],
    };
    let time_list: &str = test_input
        .split('\n')
        .next()
        .expect("Invalid test input")
        .strip_prefix("Time: ")
        .expect("Incorrect prefix");
    let distance_list: &str = test_input
        .split('\n')
        .nth(1)
        .expect("Invalid distance input")
        .strip_prefix("Distance: ")
        .expect("Incorrect prefix");
    let input_vec = read_input(time_list, distance_list);
    let part2_vec = input_2(time_list, distance_list);
    println!("{:?}", input_vec);
    for (time, distance) in &input_vec {
        println!("time:{time}, distance:{distance}");
        ChargeBoat::charge_button(&mut boats, distance, time)
    }
    ChargeBoat::charge_button(&mut boats_2, &part2_vec[0].1, &part2_vec[0].0);
    println!("{:?}", boats.button_time.iter().product::<i64>());
    println!("{:?}", boats_2.button_time.iter().product::<i64>());
    Ok(())
}
