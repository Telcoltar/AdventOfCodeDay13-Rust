mod test_main;

use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};

fn read_input_data(file_name: &str) -> (i32, Vec<(i32, i32)>) {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);

    let file_data: Vec<String> = f.lines().map(|s| s.unwrap()).collect();

    let mut bus_info: (i32, Vec<(i32, i32)>) = (file_data[0].parse().unwrap(), Vec::new());

    for (i, bus_id) in file_data[1].split(",").enumerate() {
        if bus_id.trim() != "x" {
            bus_info.1.push((i as i32, bus_id.trim().parse().unwrap()));
        }
    }

    return bus_info;
}

fn solution_part_1(file_name: &str) -> i32 {
    let (timestamp, bus_data): (i32, Vec<(i32, i32)>) = read_input_data(file_name);
    let mut waiting_times: Vec<(i32, i32)> = Vec::new();
    for (_, bus_id) in bus_data {
        waiting_times.push((bus_id, bus_id - (timestamp % bus_id)));
    }
    debug!("{:?}", waiting_times);
    waiting_times.sort_by_key(|bus_time| bus_time.1);
    return waiting_times[0].0 * waiting_times[0].1;
}

fn main() {
    env_logger::init();
    debug!("{:?}", solution_part_1("testData.txt"));
}
