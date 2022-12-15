use std::fs;

#[derive(Debug)]
enum Op<'a> {
   PLUS(&'a str),
   TIMES(&'a str),
   ERROR,
}

// "item" in items was changed from i32 to Vec<i32> of what it is divisible by. 
#[derive(Debug)]
struct Monkey<'a> {
    items: Vec<Vec<i32>>,
    op: Op<'a>,
    test: usize,
    cond: (usize, usize),
    inspections: i32
}

fn make_monkey( info: &str ) -> Monkey {

    let mut info = info.split("\n").skip(1);
    let mut items = vec![];
    for i in info.next().unwrap().split(": ").nth(1).unwrap().split(", ") {
        let mut item = vec![0; 30];
        let tmp = i.parse().unwrap(); 
        div_by(&mut item, '+', tmp);
        items.push(item);
    }
    
    let (o, v) = info.next().unwrap().split("old ").nth(1).unwrap().split_once(' ').unwrap();
    let op = {
        match o {

            "+" => { Op::PLUS(v) }
            "*" => { Op::TIMES(v) }
            _ => { Op::ERROR }
            
        }
    };
    let test = info.next().unwrap().split("by ").nth(1).unwrap().parse().unwrap();

    let t = info.next().unwrap().split("monkey ").nth(1).unwrap().parse().unwrap();
    let f = info.next().unwrap().split("monkey ").nth(1).unwrap().parse().unwrap();

    return Monkey {
        items,
        op,
        test,
        cond: (t, f),
        inspections: 0
    }
}

fn div_by ( item: &mut Vec<i32>, operator: char, number: i32 ) {
    for i in 1..item.len() {
        match operator {
            '+' => { item[i] = (item[i] + number) % i as i32; }
            '*' => { item[i] = (item[i] * number) % i as i32; }
            '/' => { if i as i32 == 0 { item[i] = 0; } else {} } // this part is broken.
                                                                 
            'o' => { item[i] = (item[i] * item[i]) % i as i32; } // it should probably not be
                                                                 // prosumed that 'o' always comes
                                                                 // as a multiplication.
            _ => {}
        }

    }
}

fn round<'a>( monkeys: &'a mut Vec<Monkey>, task: u8) {

    for i in 0..monkeys.len() {
        if monkeys[i].items.len() < 1 { continue; }
        for _ in 0..monkeys[i].items.len() {
            let cond = monkeys[i].cond;
            let mut tmp = monkeys[i].items.remove(0);
            monkeys[i].inspections += 1;
            match monkeys[i].op {
                Op::PLUS(a) => { if a == "old" {} else { div_by(&mut tmp, '+', a.parse::<i32>().unwrap()); } }
                Op::TIMES(a) => { if a == "old" { div_by(&mut tmp, 'o', 0) } else { div_by(&mut tmp, '*', a.parse::<i32>().unwrap()); } }
                Op::ERROR => { eprintln!("error in round: match monkeys.op") }
            }
            if task == 1 { div_by(&mut tmp, '/', 3) }
            if tmp[monkeys[i].test] == 0 {
                monkeys[cond.0].items.push(tmp);
            } else {
                monkeys[cond.1].items.push(tmp)
            }
        }
    }
}

// Task 1 was broken in the div by three step when i rewrote Monkey struct for task 2.
fn task1(input: &str) -> i32 {

    let mut monkeys: Vec<Monkey> = vec![];

    for info in input.split("\n\n") {
        monkeys.push(make_monkey(&info))
    }

    for _ in 0..20 {
        round(&mut monkeys, 1)
    }

    let mut first = 0;
    for m in &monkeys {
        if m.inspections > first { first = m.inspections }
    }

    let mut second = 0;
    for m in &monkeys {
        if m.inspections > second && m.inspections < first { second = m.inspections }
    }

    return first*second;
}

fn task2(input: &str) -> i64 {

    let mut monkeys: Vec<Monkey> = vec![];

    for info in input.split("\n\n") {
        monkeys.push(make_monkey(&info))
    }

    for _ in 0..10000 {
        round(&mut monkeys, 2)
    }

    let mut first: i64 = 0;
    for m in &monkeys {
        if m.inspections  as i64 > first { first = m.inspections as i64}
    }

    let mut second: i64 = 0;
    for m in &monkeys {
        if m.inspections as i64 > second && (m.inspections as i64) < first { second = m.inspections as i64 }
    }

    return first as i64 * second as i64;
}

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file!");

    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

#[cfg(test)]
mod test {
    use super::{task1, task2};
    use std::fs;

    #[test]
    fn test1() {
        let test = fs::read_to_string("test").expect("Couldn't read file!");
        assert_eq!(task1(&test), 10605);
    }
    #[test]
    fn test2() {
        let test = fs::read_to_string("test").expect("Couldn't read file!");
        assert_eq!(task2(&test), 2713310158);
    }
}
