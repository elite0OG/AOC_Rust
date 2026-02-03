#![allow(unused)]
use std::fs::read_to_string;

fn main() 
{
    let input_string = read_to_string("input.txt").unwrap();
    let data: Vec<String> = input_string
                        .lines() // splits by '\n' 
                        .map(|line| line.to_string()) .collect(); 
    day_three_part_one(&data);
    day_three_part_two(&data);
}
//helper function
fn get_largest(data: &[i32]) -> usize {
    let mut max_idx = 0;
    for (i, &v) in data.iter().enumerate() {
        if v > data[max_idx] {
            max_idx = i;
        }
    }
    max_idx
}

//code -> 
fn day_three_part_one(data:&Vec<String>)
{
    let mut voltage_added =0;
    for i in 0..data.len()
    {
        let mut data_core: Vec<i32> = data[i].chars().filter_map(|c| 
                            c.to_digit(10)).map(|d| d as i32).collect();
        let mut ten_digit =0;
        let mut once_digit =0;
        for j in 0..data_core.len()
        {
            let idx = get_largest(&data_core);
            if idx == data_core.len()-1
            {
                let edx = get_largest(&data_core[0..idx].to_vec());
                let number = (data_core[0..idx].to_vec()[edx] * 10) + data_core[idx];
                                //println!("line no: {},voiltage: {}",i+1,number);
                voltage_added +=number;
                break;
            }else {
                if idx+1 < data_core.len()
                {
                let edx = get_largest(&data_core[idx+1..data_core.len()].to_vec());
                let number = (data_core[idx] * 10) + data_core[idx+1..data_core.len()].to_vec()[edx];
                                //println!("line no: {},voiltage: {}",i+1,number);

                voltage_added +=number;
                break;
                } 
            
            }
        }
         
    }             
    println!("VOLTAGE:part-1: {}", voltage_added);//17554 YES 
}

fn day_three_part_two(data:&Vec<String>)
{
    let mut voltage_added =0;
    for i in 0..data.len()
    {
        let mut data_core: Vec<i32> = data[i].chars().filter_map(|c| 
                            c.to_digit(10)).map(|d| d as i32).collect();
        let mut digits: [i32; 12] = [0; 12];
        let len = digits.len();
        let cols = data_core.len();
        let mut k:usize =0;
        for j in 0..len
        {
            let end = cols - (len - j) + 1;

            let m = get_largest(&data_core[k..end]);
              
            let abs_idx = k + m;

            digits[j] = data_core[abs_idx];

            k = abs_idx + 1;
        }
        let mut digits_number: i64 = 0;
        for d in digits {
         digits_number = digits_number * 10 + d as i64;
        }
        //println!("{}",digits_number); //testing only
       voltage_added += digits_number;  
    }
    println!("VOLTAGE:part-2: {}", voltage_added);//175053592950232

}