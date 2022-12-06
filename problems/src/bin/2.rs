use parser;

fn main() {
    let input = parser::parse("2");
    // println!("{:?}", input);
    
    let mut total_part_1: u32 = 0;
    let mut total_part_2: u32 = 0;

    for l in input {
        let mut chars = l.chars();
        let first = chars.next().unwrap();
        chars.next();
        let second = chars.next().unwrap();

        total_part_1 += outcome(second, first);
        total_part_1 += value(second);

        total_part_2 += value_for_outcome(second);
        let played = needed_for_outcome(first, second);
        total_part_2 += value(played);
    }

    println!("1. total value: {}", total_part_1);
    println!("2. total value under new interpretation: {}", total_part_2);
}

fn needed_for_outcome(them: char, outcome: char) -> char {
    match outcome {
        'X' => match them { // need to lose
            'A' => 'Z',
            'B' => 'X',
            'C' => 'Y',
            _ => ' ',
        },
        'Y' => match them { // need to lose
            'A' => 'X',
            'B' => 'Y',
            'C' => 'Z',
            _ => ' ',
        },
        'Z' => match them { // need to win
            'A' => 'Y',
            'B' => 'Z',
            'C' => 'X',
            _ => ' ',
        },
        _ => ' ',
    }
}

fn value(played: char) -> u32 {
    match played {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    }
}

fn value_for_outcome(outcome: char) -> u32 {
    match outcome {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    }
}

fn outcome(me: char, them: char) -> u32 {
    match me {
        'X' => match them {
                'A' => 3,
                'B' => 0,
                'C' => 6,
                _ => 0,
            },
        'Y' => match them {
                'A' => 6,
                'B' => 3,
                'C' => 0,
                _ => 0,
            },
        'Z' => match them {
                'A' => 0,
                'B' => 6,
                'C' => 3,
                _ => 0,
            },
        _ => 0,
    }
}