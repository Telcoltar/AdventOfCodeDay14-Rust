mod test_main;

use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};
use regex::Regex;
use std::collections::HashMap;

fn get_input_data(file_name: &str) -> Vec<(&str, i64, String)> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);

    let re = Regex::new(r"\[(\d+)]").unwrap();

    let mut input_data: Vec<(&str, i64, String)> = Vec::new();

    for line in f.lines().map(|l| l.unwrap()) {
        debug!("{}", line);
        let mut split_line = line.split("=");
        let com = split_line.next().unwrap().trim().to_owned();
        if com == "mask" {
            input_data.push(("mask", -1, split_line.next().unwrap().trim().to_owned()))
        } else {
            input_data.push(("mem",
                             re.captures(&com).unwrap()[1].to_owned().parse().unwrap(),
                             format!("{:036b}",
                                     split_line.next().unwrap().trim().parse::<i64>().unwrap())))
        }
    }
    return input_data
}

fn convert_str_to_int(string: &mut Vec<char>, base: i64) -> i64 {
    string.reverse();
    let mut pow = 0;
    let mut res_num: i64 = 0;
    for c in string {
        if *c == '1' {
            res_num += base.pow(pow);
        }
        pow += 1;
    }
    return res_num;
}

fn apply_mask(mask: &str, num: &str) -> i64{
    let mut res_num = Vec::new();
    debug!("{}", num);
    for (digit_m, digit_p) in mask.chars().zip(num.chars()) {
        if digit_m == 'X' {
            res_num.push(digit_p);
        } else {
            res_num.push(digit_m);
        }
    }
    debug!("{}", res_num.iter().collect::<String>());
    return convert_str_to_int(&mut res_num, 2);
}

fn gen_mask_add(mask: &str, add: i64) -> Vec<i64> {
    let bin_add = format!("{:036b}", add);
    let mut add_list: Vec<String> = vec![String::new()];
    let mut tmp_list: Vec<String>;
    let mut tmp_str: String;
    for (data_m, data_a) in mask.chars().zip(bin_add.chars()) {
        if data_m == '0' {
            for s in &mut add_list {
                s.push(data_a);
            }
        } else if data_m == '1' {
            for s in &mut add_list {
                s.push('1');
            }
        } else {
            tmp_list = Vec::new();
            for s in add_list {
                tmp_str = s.clone();
                tmp_str.push('0');
                tmp_list.push(tmp_str);
                tmp_str = s.clone();
                tmp_str.push('1');
                tmp_list.push(tmp_str);
            }
            add_list = tmp_list;
        }
    }
    return add_list.iter().
        map(|s| convert_str_to_int(&mut s.chars().collect(),2)).collect();
}

fn solution_part_1(file_name: &str) -> i64 {
    let input_data = get_input_data(file_name);
    let mut cur_mask= String::new();
    let mut mem: HashMap<i64, i64> = HashMap::new();
    for data_p in input_data {
        if data_p.0 == "mask" {
            cur_mask = data_p.2.to_owned();
            debug!("{}", cur_mask);
        } else {
            mem.insert(data_p.1, apply_mask(&cur_mask, &data_p.2));
            debug!("{:?}", mem);
        }
    }
    return mem.values().sum();
}

fn solution_part_2(file_name: &str) -> i64 {
    let input_data = get_input_data(file_name);
    let mut cur_mask= String::new();
    let mut mem: HashMap<i64, i64> = HashMap::new();
    for data_p in input_data {
        if data_p.0 == "mask" {
            cur_mask = data_p.2.to_owned();
            debug!("{}", cur_mask);
        } else {
            for add in gen_mask_add(&cur_mask, data_p.1) {
                mem.insert(add, convert_str_to_int(&mut data_p.2.chars().collect(),2));
            }
            debug!("{:?}", mem);
        }
    }
    return mem.values().sum();
}

fn main() {
    env_logger::init();
    info!("{:?}", solution_part_1("inputData.txt"));
    info!("{:?}", solution_part_2("inputData.txt"))
}
