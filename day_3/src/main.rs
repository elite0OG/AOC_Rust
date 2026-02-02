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
fn get_largest(data:&Vec<i32>)->usize
{
    let mut current_largest = data[0];
    let mut idx =0;
    for i in 0..data.len()
    {
        if current_largest < data[i]
        {
            current_largest = data[i];
            idx = i;
        }
    }
    idx
}

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
    }
    println!("VOLTAGE:part-2: {}", voltage_added);//TLDR 

}