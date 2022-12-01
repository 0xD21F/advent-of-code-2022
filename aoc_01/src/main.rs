use itertools::Itertools;

pub fn main() {
    // Part 1
    let input = include_str!("../input.txt");
    // Not sure about splitting this with \r\n\r\n... probably won't work in all cases but who cares?
    let max_total_calories = input.split("\r\n\r\n")
        .map(|elf_calorie_list| elf_calorie_list.lines()
            .filter_map(|elf_calorie_list_item| elf_calorie_list_item.parse::<i32>().ok()).sum::<i32>()
        )
        .max()
        .unwrap();
    println!("{}", max_total_calories);

    // Part 2
    let input = include_str!("../input.txt");
    let max_total_calories = input.split("\r\n\r\n")
        .map(|elf_calorie_list| elf_calorie_list.lines()
            .filter_map(|elf_calorie_list_item| elf_calorie_list_item.parse::<i32>().ok()).sum::<i32>()
        )
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum::<i32>();
    println!("{}", max_total_calories);
}
