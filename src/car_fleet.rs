pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars = position.iter().zip(speed.iter()).collect::<Vec<_>>();
    cars.sort_by_key(|&(pos, _)| -pos);

    let mut fleets = 0;
    let mut time = 0.0;

    for (pos, spd) in cars {
        let arrival = (target - pos) as f64 / *spd as f64;
        if arrival > time {
            time = arrival;
            fleets += 1;
        }
    }

    fleets
}
