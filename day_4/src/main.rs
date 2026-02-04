#![allow(unused)]
use std::fs::read_to_string;

fn main() {
    let input: Vec<String> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    day_four_part_one(&input);
    day_four_part_two(&input);
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
    let mut c_t = 0;
    for (y, line) in input.iter().enumerate() 
    {
        for (x, ch) in line.chars().enumerate() 
        {
           if grid[idx(x, y)] == '@' 
           {
                let mut count = 0;
                // left
                if x > 0 && grid[idx(x - 1, y)] == '@' {
                    count += 1;
                }
                // right
                if x + 1 < col_len && grid[idx(x + 1, y)] == '@' {
                    count += 1;
                }
                // up
                if y > 0 && grid[idx(x, y - 1)] == '@' {
                    count += 1;
                }
                // down
                if y + 1 < row_len && grid[idx(x, y + 1)] == '@' {
                    count += 1;
                }
                // up-left
                if x > 0 && y > 0 && grid[idx(x - 1, y - 1)] == '@' {
                    count += 1;
                }
                // up-right
                if x + 1 < col_len && y > 0 && grid[idx(x + 1, y - 1)] == '@' {
                    count += 1;
                }
                // down-left
                if x > 0 && y + 1 < row_len && grid[idx(x - 1, y + 1)] == '@' {
                    count += 1;
                }
                // down-right
                if x + 1 < col_len && y + 1 < row_len && grid[idx(x + 1, y + 1)] == '@' {
                    count += 1;
                }

                if count < 4 {
                    girp_print[idx(x, y)] = 'x';
                    c_t +=1;
                }
            }

             
        }
        
    }

    //*printing the gird
    // {
    //     println!("ROW: {}, COL: {}", row_len, col_len);
    //     for (y, line) in input.iter().enumerate() {
    //         for (x, ch) in line.chars().enumerate() {
    //             print!("{}", girp_print[idx(x,y)]);
             
    //         }
    //         println!();
    //     }
    // }
    println!("{}",c_t);//1516

}

//TODO compelete it. 
pub fn day_four_part_two(input:&Vec<String>)
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
}