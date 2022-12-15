use std::fs;
use std::collections::HashSet;
fn task1(input: &str) -> i32 {

    let mut set: HashSet<(usize, usize)> = HashSet::new();

    let horizontal_size = {
        let mut sum = 0;
        for c in input.chars() {
            if c == '\n' { break; }
            sum += 1;
        }
        sum
    };

    let mut vertical_size = 0;

    let mut vertical_high = vec!(-1; horizontal_size);
    let mut horizontal_high;

    for (row, l) in input.lines().enumerate() {
        horizontal_high = -1;
        vertical_size += 1;
        for (col, c) in l.chars().enumerate() {

            let temp = c as u8 - '0' as u8;

            if temp as i32 > vertical_high[col] {
               set.insert( (row, col) );
               vertical_high[col] = temp as i32;
            }
            if temp as i32> horizontal_high {
               set.insert( (row, col) );
               horizontal_high = temp as i32;
            }
        }
    }

    vertical_high = vec!(-1; horizontal_size);

    for (row, l) in input.lines().rev().enumerate() {
        horizontal_high = -1;
        for (col, c) in l.chars().rev().enumerate() {

            let temp = c as u8 - '0' as u8;

            if temp as i32 > vertical_high[col] {
               set.insert( (vertical_size - row -1, horizontal_size - col -1) );
               vertical_high[col] = temp as i32;
            }
            if temp as i32 > horizontal_high {
               set.insert( (vertical_size - row -1, horizontal_size - col -1) );
               horizontal_high = temp as i32;
            }
        }
    }
    return set.len() as i32;
}

fn task2(input: &str) -> i32 {

    let horizontal_size = {
        let mut sum = 0;
        for c in input.chars() {
            if c == '\n' { break; }
            sum += 1;
        }
        sum
    };

    let vertical_size = {
        let mut sum = 0;
        for c in input.chars() {
            if c == '\n' { sum += 1; }
        }
        sum
    };

    let mut new_input: Vec<Vec<i32>> = vec![];

    for l in input.lines() {
        let temp = l.chars().map(|c| c as i32 - '0' as i32).collect();
        new_input.push(temp);
    }
    let mut result_matrix = vec![vec![1; horizontal_size]; vertical_size];

    let mut trees: [i32; 10] =  [-1; 10];
    for row in 0..vertical_size {
        trees =  [-1; 10];
        for col in 0..horizontal_size {
            
            let temp = new_input[row][col];

            let mut dist = col as i32;
            for n in 0..10 {
                if trees[n] != -1 {
                    trees[n] += 1;
                }
            }
            for n in (temp as usize)..10 {

                if trees[n] > 0 && trees[n] < dist {
                    dist = trees[n];
                } 
            }
            result_matrix[row][col] *= dist;
            trees[temp as usize] = 0;

        }
    }

    let mut trees: [i32; 10] =  [-1; 10];
    for col in 0..vertical_size {
        trees =  [-1; 10];
        for row in 0..horizontal_size {

            let temp = new_input[row][col];

            let mut dist = row as i32;
            for n in 0..10 {
                if trees[n] != -1 {
                    trees[n] += 1;
                }
            }
            for n in (temp as usize)..10 {

                if trees[n] > 0 && trees[n] < dist {
                    dist = trees[n];
                } 
            }
            result_matrix[row][col] *= dist;
            trees[temp as usize] = 0;

        }
    }
    let mut trees: [i32; 10] =  [-1; 10];
    for row in (0..vertical_size).rev() {
        trees =  [-1; 10];
        for col in (0..horizontal_size).rev() {

            let temp = new_input[row][col];

            let mut dist = horizontal_size as i32 - col as i32 -1;

            for n in 0..10 {
                if trees[n] != -1 {
                    trees[n] += 1;
                }
            }
            for n in (temp as usize)..10 {

                if trees[n] > 0 && trees[n] < dist {
                    dist = trees[n];
                } 
            }
            result_matrix[row][col] *= dist;
            trees[temp as usize] = 0;

        }
    }
    let mut trees: [i32; 10] =  [-1; 10];
    for col in (0..vertical_size).rev() {
        trees =  [-1; 10];
        for row in (0..horizontal_size).rev() {

            let temp = new_input[row][col];

            let mut dist = vertical_size as i32 - row as i32 -1;
            for n in 0..10 {
                if trees[n] != -1 {
                    trees[n] += 1;
                }
            }
            for n in (new_input[row][col] as usize)..10 {

                if trees[n] > 0 && trees[n] < dist {
                    dist = trees[n];
                } 
            }
            result_matrix[row][col] *= dist;
            trees[new_input[row][col] as usize] = 0;

        }
    }

    let mut rv = 0;
    for i in &result_matrix{
        for j in i {

            if j > &rv {

                rv = *j;
            }
        }
    }



    return rv;
}


fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file!");
 
    println!("task_1: {}", task1(&input));
    println!("task_2: {}", task2(&input));
}

#[cfg(test)]
mod test {
    use super::{task1, task2};
    use std::fs;

    #[test]
    fn test1() {
        let test = fs::read_to_string("test").expect("Couldn't read file!");
        assert_eq!(task1(&test), 21);
    }
    #[test]
    fn test2() {
        let test = fs::read_to_string("test").expect("Couldn't read file!");
        assert_eq!(task2(&test), 8);
    }
}
