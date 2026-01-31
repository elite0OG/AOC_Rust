#![allow(unused)]
use core::num;
use std::fs::read_to_string;
//demo input is form AOC's website. not my input given by AOC. it is for verifing my output to AOC's output. 
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
    
    println!("Sum of inveled ID's: {}",added_inveled_ids);
    
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
    
    println!("Sum of inveled ID's: {}",added_inveled_ids);
    
}


fn main() 
{
    day_two_part_one();
   
}