

pub fn main() {
    // Part 1
    let input = include_str!("../input.txt");
    let res = input
        .lines()
        .map(|line| line
            .split(" ")
            .collect::<Vec<&str>>()
        )
        .map(|single_game| {
            let game: [&str; 2] = single_game.try_into().unwrap();
            match game[0] {
                // Opponent Rock
                "A" => {
                    match game[1] {
                        // Rock/Rock - 1 point for rock, 3 for a draw (4)
                        "X" => 4,
                        // Paper/Rock - 2 points for paper, 6 for a win (8)
                        "Y" => 8,
                        // Scissors/Rock - 3 points for scissors, 0 for a loss (3)
                        "Z" => 3,
                        _ => panic!("Invalid RPS designation"),
                    }
                },
                // Opponent Paper
                "B" => {
                    match game[1] {
                        // Rock/Paper - 1 point for rock, 0 for a loss (1)
                        "X" => 1,
                        // Paper/Paper - 2 points for paper, 3 for a draw (5)
                        "Y" => 5,
                        // Scissors/Paper - 3 points for scissors, 6 for a win (9)
                        "Z" => 9,
                        _ => panic!("Invalid RPS designation"),
                    }
                },
                // Opponent Scissors
                "C" => {
                    match game[1] {
                        // Rock/Scissors - 1 point for rock, 6 for a win (7)
                        "X" => 7,
                        // Paper/Scissors - 2 points for paper, 0 for a loss (2)
                        "Y" => 2,
                        // Scissors/Scissors - 3 points for scissors, 3 for a draw (6)
                        "Z" => 6,
                        _ => panic!("Invalid RPS designation"),
                    }
                },
                _ => panic!("Invalid RPS designation"),
            }
        })
        .sum::<i32>();
    println!("Part 1: {}", res);
    
    // Part 2
    let input = include_str!("../input.txt");
    let res = input
        .lines()
        .map(|line| line
            .split(" ")
            .collect::<Vec<&str>>()
        )
        .map(|single_game| {
            let game: [&str; 2] = single_game.try_into().unwrap();
            match game[0] {
                // Opponent Rock
                "A" => {
                    match game[1] {
                        // Lose - Pick Scissors - (3 points for scissors, 0 for a loss (3))
                        "X" => 3,
                        // Draw - Pick Rock - (1 point for rock, 3 for a draw (4))
                        "Y" => 4,
                        // Win - Pick Paper - (2 points for paper, 6 for a win (8))
                        "Z" => 8,
                        _ => panic!("Invalid RPS designation"),
                    }
                },
                // Opponent Paper
                "B" => {
                    match game[1] {
                        // Lose - Pick Rock - (1 point for rock, 0 for a loss (1))
                        "X" => 1,
                        // Draw - Pick Paper - (2 points for paper, 3 for a draw (5))
                        "Y" => 5,
                        // Win - Pick Scissors - (3 points for scissors, 6 for a win (9))
                        "Z" => 9,
                        _ => panic!("Invalid RPS designation"),
                    }
                },
                // Opponent Scissors
                "C" => {
                    match game[1] {
                        // Lose - Pick Paper - (2 points for paper, 0 for a loss (2))
                        "X" => 2,
                        // Draw - Pick Scissors - (3 points for scissors, 3 for a draw (6))
                        "Y" => 6,
                        // Win - Pick Rock - (1 point for rock, 6 for a win (7))
                        "Z" => 7,
                        _ => panic!("Invalid RPS designation"),
                    }
                },
                _ => panic!("Invalid RPS designation"),
            }
        })
        .sum::<i32>();
    println!("Part 1: {}", res);
    
}
