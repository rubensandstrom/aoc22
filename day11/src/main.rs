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
    items: Vec<i64>,
    op: Op<'a>,
    test: i64,
    cond: (usize, usize),
    inspections: i32,
}

fn make_monkey( info: &str ) -> Monkey {

    let mut info = info.split("\n").skip(1);
    let mut items = vec![];
    for i in info.next().unwrap().split(": ").nth(1).unwrap().split(", ") {
        let tmp = i.parse().unwrap(); 
        items.push(tmp);
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

fn round<'a>( monkeys: &'a mut Vec<Monkey>, task: u8, max_worry: i64) {

    for i in 0..monkeys.len() {
        if monkeys[i].items.len() < 1 { continue; }
        for _ in 0..monkeys[i].items.len() {
            let cond = monkeys[i].cond;
            let mut tmp = monkeys[i].items.remove(0);
            monkeys[i].inspections += 1;
            match monkeys[i].op {
                Op::PLUS(a) => { if a == "old" { tmp = ( tmp + tmp ) % max_worry } else { tmp = ( tmp + a.parse::<i64>().unwrap()) % max_worry; } }
                Op::TIMES(a) => { if a == "old" { tmp = ( tmp * tmp ) % max_worry } else { tmp = ( tmp * a.parse::<i64>().unwrap() ) % max_worry; } }
                Op::ERROR => { eprintln!("error in round: match monkeys.op") }
            }
            if task == 1 { tmp /= 3 }
            if tmp % monkeys[i].test == 0 {
                monkeys[cond.0].items.push(tmp);
            } else {
                monkeys[cond.1].items.push(tmp)
            }
        }
    }
}

fn task1(input: &str) -> i32 {

    let mut monkeys: Vec<Monkey> = vec![];

    for info in input.split("\n\n") {
        monkeys.push(make_monkey(&info))
    }

    let mut max_worry = 1;
    for m in &monkeys {
        max_worry *= m.test;
    }

    for _ in 0..20 {
        round(&mut monkeys, 1, max_worry)
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

    let mut max_worry = 1;
    for m in &monkeys {
        max_worry *= m.test;
    }

    for _ in 0..10000 {
        round(&mut monkeys, 2, max_worry)
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
