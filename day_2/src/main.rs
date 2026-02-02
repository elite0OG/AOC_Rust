#![allow(unused)]
use core::num;
use std::{fs::read_to_string};
//demo input is form AOC's website. not my input given by AOC. it is for verifing my output to AOC's output. 
//performence is poor becuse of three loops time complexcity O(t^3)
const INPUT:&str = "input.txt";

fn day_two_part_one()
{
    let binding = read_to_string( INPUT).unwrap();
    let input: Vec<&str> = binding.split(',').collect();    
    let mut added_inveled_ids:u64 = 0;
    let mut r_vec:Vec<i64> = Vec::new();
    let mut l_vec:Vec<i64> = Vec::new();

    for mut s in &input
    {
        let s_s:Vec<&str> = s.split('-').collect();
        r_vec.push((s_s[0]).parse().unwrap());
        l_vec.push((s_s[1]).parse().unwrap());
    }
    
    for i in 0..r_vec.len() 
    { 
        for g in r_vec[i]..=l_vec[i]
        {
            let str_g:Vec<char> = g.to_string().chars().collect();
            let g_len = str_g.len() -1;
            if(str_g.len() == 2)
            {
                for k in 0..str_g.len()-1
                {
                    if str_g[k] == str_g[k+1]
                    {
                        let value:String = str_g.clone().into_iter().collect();
                        added_inveled_ids += value.parse::<u64>().unwrap();
                        //println!("{}",value);
                    }
                }
            }if(str_g.len() % 2 == 0 && str_g.len() != 2)
            {
                let harf = str_g.len()/2;
                for k in 0..str_g.len()
                {
                    if(str_g[0..harf] == str_g[harf..str_g.len()] && str_g[0] == '0' && str_g[harf] == '0')
                    {
                        break;
                    }
                    if(str_g[0..harf] == str_g[harf..str_g.len()])
                    {
                        let value:String = str_g.clone().into_iter().collect();
                        added_inveled_ids += value.parse::<u64>().unwrap();
                        //println!("{}",value);
                         break;
                    }

                }
            }
        }
    }
    
    println!("Sum of inveled ID's: {}",added_inveled_ids);//30608905813 YES
    
}


fn day_two_part_two()
{
    let binding = read_to_string( INPUT).unwrap();
    let input: Vec<&str> = binding.split(',').collect();    
    let mut added_inveled_ids:u64 = 0;
    let mut r_vec:Vec<i64> = Vec::new();
    let mut l_vec:Vec<i64> = Vec::new();
   
    for mut s in &input
    {
        let s_s:Vec<&str> = s.split('-').collect();
        r_vec.push((s_s[0]).parse().unwrap());
        l_vec.push((s_s[1]).parse().unwrap());
    }
    for k in 0..r_vec.len()
    {
        for j in r_vec[k]..=l_vec[k]
        {
            let number:Vec<char> = j.to_string().chars().collect();
            if number.len() == 1
            {
                continue;
            }
            {
                let petten = number[0];
                let mut num =0;
                for g in 0..number.len()
                {
                    if number[g] == petten
                    {
                        num+=1;
                    }
                }
                if num == number.len()
                {
                    let value:String = number.clone().into_iter().collect();
                    //println!("{}",value);
                    added_inveled_ids += value.parse::<u64>().unwrap();
                    continue;
                }
            }

            {
                let  harf  = &number[0..(number.len()/2)];
                let seond_harf = &number[(number.len()/2)..number.len()];
                if harf == seond_harf
                {
                    let value:String = number.clone().into_iter().collect();
                    //println!("{}",value);
                    added_inveled_ids += value.parse::<u64>().unwrap();
                    continue;
                }
            }
            {
                let ono_third = &number[0..(number.len()/3)];
                let two_third = &number[(number.len()/3)..2* (number.len()/3)];
                let end_third = &number[ 2* (number.len()/3)..number.len()];
                if ono_third == two_third && ono_third == end_third
                {
                    let value:String = number.clone().into_iter().collect();
                    //println!("{}",value);
                    added_inveled_ids += value.parse::<u64>().unwrap();
                    continue;
                }
            }
            {
                let ono_fort = &number[0..(number.len()/4)];
                let two_fort = &number[(number.len()/4)..2* (number.len()/4)];
                let three_fort = &number[ 2* (number.len()/4)..3* (number.len()/4)];
                let end_fort = &number[ 3* (number.len()/4).. (number.len())];
                if ono_fort == two_fort && three_fort == end_fort && three_fort == ono_fort
                {
                    let value:String = number.clone().into_iter().collect();
                    //println!("{}",value);
                    added_inveled_ids += value.parse::<u64>().unwrap();
                    continue;
                }
            }
            {
                let ono_fivet = &number[0..(number.len()/5)];
                let two_fivet = &number[(number.len()/5)..2* (number.len()/5)];
                let three_fivet = &number[ 2* (number.len()/5)..3* (number.len()/5)];
                let four_fivet = &number[ 3* (number.len()/5).. 4*(number.len()/5)];
                let end_fivet = &number[ 4* (number.len()/5).. (number.len())];
                if ono_fivet == two_fivet && two_fivet == three_fivet &&three_fivet == four_fivet && four_fivet == end_fivet
                {
                    let value:String = number.clone().into_iter().collect();
                    //println!("{}",value);
                    added_inveled_ids += value.parse::<u64>().unwrap();
                    continue;
                }
            }
        }
    }

    println!("Sum of inveled ID's: {}",added_inveled_ids); //31898925685 YES
    
}


fn main() 
{
    day_two_part_one();
    day_two_part_two();
}