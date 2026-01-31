#![allow(unused)]
//*imports
use std::fs::read_to_string;

#[derive(Debug)]
enum DialDirection
{
    RIGHT,
    LEFT
}
const  IMPUT_FILE_NAME:&str = "input-1.txt";

#[derive(Debug,Default)]
struct Dial
{
    direction:Option<DialDirection>,
    value:Option<u32>,
} 
 
fn day_one_part_one()
{
   let input = read_to_string(IMPUT_FILE_NAME).expect("unable to read input file");
    let mut dial:Vec<Dial> = Vec::new();
    let mut dial_start:i32 = 50;
    let mut dial_petter:u32 = 0;
    //extrating data to array_input
    {
        let mut array_input:Vec<String> = Vec::new();
        let chars: Vec<char> = input.chars().collect(); // collect once
        let mut i = 0;
        while  (i < chars.len())
        {
            if(chars[i].is_alphanumeric())
            {
                let mut string_builder = String::new();
                while(i < input.len() && chars[i].is_alphanumeric() && chars[i] != '\n' )
                {
                    string_builder.push(chars[i]);
                    i+=1;
                }
                array_input.push(string_builder);
            }else { 
                i+=1;
            }

        }
    

        for k in 0..array_input.len()
        {
            let mut argstr: Vec<char> = (array_input[k]).chars().collect();
            let mut dl:Dial = Dial::default();
            if argstr[0] == 'L'
            {
                dl.direction = Some(DialDirection::LEFT);
            }else{
                dl.direction = Some(DialDirection::RIGHT);
            }
            let number_str_ch = argstr.split_off(1);
            let mut number_str = String::new();
            for g in number_str_ch
            {
                number_str.push(g);
            }
            dl.value = Some(number_str.parse::<u32>().unwrap());
            dial.push(dl);
        }
    }

    for d in &dial
    {
        let value = d.value.unwrap();
         match d.direction { 
            Some(DialDirection::LEFT) => { 
                let mut exv:i32 = dial_start as i32 - value as i32;
                while exv < 0
                {
                     exv += 100;
                    //   if exv == 0
                    //     {
                    //     dial_petter +=1;
                    //     }
                }
                dial_start = exv;
            } Some(DialDirection::RIGHT) => { 
                let mut exv:i32 = dial_start as i32 + value as i32;
                while exv > 99
                {
                     exv -= 100;
        //               if exv == 0
        // {
        //             dial_petter +=1;
        // }
                }
                dial_start = exv;
             } None => { 
                println!("no value in direction ");
            }
        }
        if dial_start == 0
        {
                    dial_petter +=1;
        }
    }
        
    println!("PETTEN: {}",dial_petter); //my input ans-> 1097
}


fn day_one_part_two()
{
   let input = read_to_string( IMPUT_FILE_NAME).expect("unable to read input file");
    let mut dial:Vec<Dial> = Vec::new();
    let mut dial_start:i32 = 50;
    let mut dial_petter:u32 = 0;
    //extrating data to array_input
    {
        let mut array_input:Vec<String> = Vec::new();
        let chars: Vec<char> = input.chars().collect(); // collect once
        let mut i = 0;
        while  (i < chars.len())
        {
            if(chars[i].is_alphanumeric())
            {
                let mut string_builder = String::new();
                while(i < input.len() && chars[i].is_alphanumeric() && chars[i] != '\n' )
                {
                    string_builder.push(chars[i]);
                    i+=1;
                }
                array_input.push(string_builder);
            }else { 
                i+=1;
            }

        }
    

        for k in 0..array_input.len()
        {
            let mut argstr: Vec<char> = (array_input[k]).chars().collect();
            let mut dl:Dial = Dial::default();
            if argstr[0] == 'L'
            {
                dl.direction = Some(DialDirection::LEFT);
            }else{
                dl.direction = Some(DialDirection::RIGHT);
            }
            let number_str_ch = argstr.split_off(1);
            let mut number_str = String::new();
            for g in number_str_ch
            {
                number_str.push(g);
            }
            dl.value = Some(number_str.parse::<u32>().unwrap());
            dial.push(dl);
        }
    }

    for d in &dial
    {
        let value = d.value.unwrap();
         match d.direction { 
            Some(DialDirection::LEFT) => { 
               // let mut exv:i32 = dial_start as i32 - value as i32;
               for i in 0..value
               {
                    dial_start -= 1;
                    if dial_start < 0 {
                         dial_start = 99; // wrap-around
                    }
                    if (dial_start == 0)
                    {
                         
                        dial_petter +=1;
                    }
               }
             } Some(DialDirection::RIGHT) => { 
               for i in 0..value
               {
                    dial_start += 1;
                    if dial_start == 100 {
                         dial_start = 0; // wrap-around 
                    } if dial_start == 0 { dial_petter += 1; }
               }
              } None => { 
                println!("no value in direction ");
            }
        }
        
    }
        
    println!("PETTEN: {}",dial_petter); // my input ans-> 7101
}

fn main() 
{
    day_one_part_one();
    day_one_part_two();
}