#![allow(unused)]
use std::fs::read_to_string;

fn main() 
{
    let input: Vec<String> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    day_four_part_one(&input);
    day_four_part_two( &input);
}

pub fn day_four_part_one(input:&Vec<String>)
{
    let row_len = input.len();
    let col_len = input[0].len();

    let mut grid: Vec<char> = vec![' '; row_len * col_len];

    let idx = |x: usize, y: usize| y * col_len + x;
    for (y, line) in input.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[idx(x,y)] = ch;
        }
    }
    let  mut girp_print = grid.clone();  //only for printing the gird without harming acctual data
    let mut c_t: i32 = 0;
    for (y, line) in input.iter().enumerate() 
    {
        for (x, ch) in line.chars().enumerate() 
        {
           if grid[idx(x, y)] == '@' 
           {
                    let mut count = 0;
                    //nebours
                    let dirs = [
                        (-1, 0), (1, 0), (0, -1), (0, 1),
                        (-1, -1), (1, -1), (-1, 1), (1, 1),
                    ];

                    for (dx, dy) in dirs 
                    {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && nx < col_len as isize && ny >= 0 && ny < row_len as isize 
                        {
                            if grid[idx(nx as usize, ny as usize)] == '@' 
                            {
                                count += 1;
                            }
                        }
                    }

                    if count < 4 
                    {
                        girp_print[idx(x, y)] = 'x';
                        c_t += 1;
                    }
                }
             
        }
        
    }

    println!("c_t = {}",c_t);//1516

}

pub fn day_four_part_two(input: &Vec<String>) 
{
    let mut c_t: i32 = 0;
    let row_len = input.len();
    let col_len = input[0].len();

    let mut grid: Vec<char> = vec![' '; row_len * col_len];

    let idx = |x: usize, y: usize| y * col_len + x;
    for (y, line) in input.iter().enumerate() 
    {
        for (x, ch) in line.chars().enumerate() 
        {
            grid[idx(x,y)] = ch;
        }
    }
    let idx = |x: usize, y: usize| y * col_len + x;
    //loop untill we can not update gird
    loop {
        let mut girp_print = grid.clone();
        let mut updated = false;

        for (y, line) in input.iter().enumerate() 
        {
            for (x, _) in line.chars().enumerate() 
            {
                if grid[idx(x, y)] == '@' 
                {
                    let mut count = 0;

                    let dirs = [
                        (-1, 0), (1, 0), (0, -1), (0, 1),
                        (-1, -1), (1, -1), (-1, 1), (1, 1),
                    ];

                    for (dx, dy) in dirs {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && nx < col_len as isize && ny >= 0 && ny < row_len as isize {
                            if grid[idx(nx as usize, ny as usize)] == '@' {
                                count += 1;
                            }
                        }
                    }

                    if count < 4 {
                        girp_print[idx(x, y)] = 'x';
                        unsafe { c_t += 1 };
                        updated = true;
                    }
                }
            }
        }

        if !updated {
            break;
        }
        grid = girp_print;
    }

    println!("c_t = {}", c_t);//9122
}
