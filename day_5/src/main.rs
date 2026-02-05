use std::fs::read_to_string;


 fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    // Sort ranges by start
    ranges.sort_by_key(|r| r.0);
    let mut merged: Vec<(i64, i64)> = Vec::new();

    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 { // overlap or touching
                last.1 = last.1.max(end); // extend the last range
                continue;
            }
        }
        merged.push((start, end));
    }

    merged
}
 


fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut fresh_id_range: Vec<(i64, i64)> = Vec::new();
    let mut available_id: Vec<i64> = Vec::new();

    // Split input into lines
    let mut lines = input.lines();

    // First block: ranges
    for line in &mut lines {
        if line.trim().is_empty() {
            break; // stop when blank line
        }
        let parts: Vec<&str> = line.split('-').collect();
        if parts.len() == 2 {
            let left = parts[0].parse::<i64>().unwrap();
            let right = parts[1].parse::<i64>().unwrap();
            fresh_id_range.push((left, right));
        }
    }

    // Remaining lines: available IDs
    for line in lines {
        if let Ok(num) = line.trim().parse::<i64>() {
            available_id.push(num);
        }
    }

    // Count how many available IDs fall into any range
    let mut fresh_count = 0;
    for &id in &available_id {
        if fresh_id_range.iter().any(|&(l, r)| id >= l && id <= r) {
            fresh_count += 1;
        }
    }
 
    println!("COUNT: {}", fresh_count);
     let merged = merge_ranges(fresh_id_range);

    // Compute unique length
    let total_len: i64 = merged.iter().map(|(s,e)| e - s + 1).sum();
 println!("SIZE (raw sum): {}", total_len);
}

 

 