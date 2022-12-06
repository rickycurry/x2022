use parser;
use std::collections::HashSet;

fn main() {
    let input = parser::parse("3");
    // println!("{:?}", input);

    let mut shared: Vec<u8> = Vec::new();
    for s in &input {
        let bytes = s.as_bytes();
        let half_len = bytes.len() / 2;
        let first_half = &bytes[0 .. half_len];
        let second_half = &bytes[half_len .. bytes.len()];

        let mut first_set: HashSet<u8> = HashSet::new();
        let mut second_set: HashSet<u8> = HashSet::new();

        for b in first_half {
            first_set.insert(*b);
        }
        for b in second_half {
            second_set.insert(*b);
        }

        for b in first_set.intersection(&second_set) {
            shared.push(*b);
        }
    }

    assert_eq!(shared.len(), input.len());
    // println!("{:?}", shared);

    let total_priority: u32 = shared.into_iter()
                                   .map(value_from_byte)
                                   .reduce(|acc, x| acc + x)
                                   .unwrap();
    println!("1. total priority is {}", total_priority);

    let mut badges: Vec<u8> = Vec::new();
    let mut counter = 0u16;
    let mut sets: [HashSet<u8>; 3] = [HashSet::new(), HashSet::new(), HashSet::new()];
    for s in &input {
        let bytes = s.as_bytes();
        let set: HashSet<u8> = HashSet::from_iter(bytes.iter().cloned());
        sets[(counter % 3) as usize] = set;

        counter += 1;
        if counter % 3 == 0 {
            let mut intersection = sets[0].intersection(&sets[1]);
            let inter_set: HashSet<u8> = intersection.cloned().collect();
            intersection = inter_set.intersection(&sets[2]);
            badges.push(*intersection.next().unwrap());
        }
    }

    let new_total: u32 = badges.into_iter()
                                .map(value_from_byte)
                                .reduce(|acc, x| acc + x)
                                .unwrap();
    println!("2. new priority is {}", new_total);
}

fn value_from_byte(byte: u8) -> u32 {
    if byte >= 97 {
        return (byte - 96).into();
    } else {
        return (byte - 38).into();
    }
}