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

fn solution_part_2(file_name: &str) -> i128 {
    let (_timestamp, bus_data): (i32, Vec<(i32, i32)>) = read_input_data(file_name);
    let mut product: i128 = 1;
    for (_bus_index, bus_id) in bus_data.clone() {
        product *= bus_id as i128;
    }
    let mut e: i128 = 0;
    for (bus_index, bus_id) in bus_data {
        let (s, _t) = erw_eukl_alg(bus_id as i128, product / (bus_id as i128));
        e += -bus_index as i128 * s * product / (bus_id as i128)
    }
    while e < 0 {
        e += product;
    }
    e = e % product;
    return e;
}

fn erw_eukl_alg(num_1: i128, num_2: i128) -> (i128, i128) {
    let mut a: i128;
    let mut b: i128;
    let mut r: i128;
    let mut qs: Vec<i128> = Vec::new();
    if num_1 < num_2 {
        a = num_2;
        b = num_1;
    } else {
        a = num_1;
        b = num_2
    }
    qs.push(a / b);
    r = a % b;
    while r != 0 {
        a = b;
        b = r;
        qs.push(a / b);
        r = a % b;
    }
    debug!("{:?}, {}, {}", qs, a, b);
    let  mut s = 1;
    let mut t = 0;
    let mut tmp;
    qs.reverse();
    for q in qs {
        tmp = s;
        s = t;
        t = tmp - t * q;
    }
    return (s, t)
}

fn main() {
    env_logger::init();
    info!("{:?}", solution_part_1("inputData.txt"));
    info!("{:?}", solution_part_2("inputData.txt"));
}
