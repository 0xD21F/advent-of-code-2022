#![feature(iter_array_chunks)]

use std::collections::HashMap;

pub fn main() {
    // Create a hashmap of all the possible Rucksack contents indexed by their score for later reference
    let scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, char)| (char, index + 1))
        .collect::<HashMap<char, usize>>();

    // Part 1
    let input = include_str!("../input.txt");

    let res = input
        .lines()
        .map(|line| {
                let rucksack_size = line.len();
                let compartment_a = &line[0..rucksack_size / 2];
                let compartment_b = &line[rucksack_size / 2..];

                let common_items = compartment_a
                    .chars()
                    .find(|c| compartment_b.contains(*c))
                    .unwrap();
                scores.get(&common_items).unwrap()
            }
        )
        .sum::<usize>();

    println!("Part 1: {}", res);

    
    // Part 2
    let input = include_str!("../input.txt");
    
    let res = input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let common_items = a
                .chars()
                .find(|a_char| {
                    b.contains(*a_char)
                        && c.contains(*a_char)
                })
                .unwrap();
            scores.get(&common_items).unwrap()
        })
        .sum::<usize>();

    println!("Part 2: {}", res);
}
