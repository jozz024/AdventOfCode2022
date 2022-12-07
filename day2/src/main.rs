use std::fs;

enum RockPaperScissorsResult {
    Won,
    Lost,
    Tie
}
fn get_losing(opponent: &str) -> String {
    match opponent {
        "A" => { 
            return "Z".to_string()
        },
        "B" => {
            return "X".to_string()
        },
        "C" => {
            return "Y".to_string()
        },
        _ => {
            return "".to_string()
        }
    }
}

fn get_same(opponent: &str) -> String {
    match opponent {
        "A" => { 
            return "X".to_string()
        },
        "B" => {
            return "Y".to_string()
        },
        "C" => {
            return "Z".to_string()
        },
        _ => {
            return "".to_string()
        }
    }
}

fn get_winning(opponent: &str) -> String {
    match opponent {
        "A" => { 
            return "Y".to_string()
        },
        "B" => {
            return "Z".to_string()
        },
        "C" => {
            return "X".to_string()
        },
        _ => {
            return "".to_string()
        }
    }
}

fn get_player(opponent: &str, type_of_play: &str) -> String {
    let mut result = "".to_string();
    match type_of_play {
        "X" => {
            result = get_losing(opponent)
        }
        "Y" => {
            result = get_same(opponent)
        },
        "Z" => {
            result = get_winning(opponent)
        },
        _ => {}
    }
    result
}
fn rock_paper_scissors(opponent: &str, player: &str) -> RockPaperScissorsResult {
    // Result meanings:
    // 0: Won
    // 1: Lost
    // 2: Tie
    if opponent == "A" {
        if player == "Y" {
            return RockPaperScissorsResult::Won
        }
        else if player == "X" {
            return RockPaperScissorsResult::Tie
        }
        else {
            return RockPaperScissorsResult::Lost
        }
    }
    else if opponent == "B" {
        if player == "Y" {
            return RockPaperScissorsResult::Tie
        }
        else if player == "X" {
            return RockPaperScissorsResult::Lost
        }
        else {
            return RockPaperScissorsResult::Won
        }
    }
    else if opponent == "C" {
        if player == "Y" {
            return RockPaperScissorsResult::Lost
        }
        else if player == "X" {
            return RockPaperScissorsResult::Won
        }
        else {
            return RockPaperScissorsResult::Tie
        }
    }
    else {
        return RockPaperScissorsResult::Lost
    }
}

fn main() {
    let file = fs::read_to_string("data.txt").unwrap();

    let mut points = 0;

    for line in file.lines() {
        let results: Vec<&str> = line.split(" ").collect();
        let opponent = results[0];
        let type_of_play = results[1];
        let mut total = 0;
        let player = get_player(opponent, type_of_play);
        let score: RockPaperScissorsResult = rock_paper_scissors(opponent, player.as_str());

        
        match score {
            RockPaperScissorsResult::Won => {
                total += 6
            },
            RockPaperScissorsResult::Lost => {
                total += 0
            },
            RockPaperScissorsResult::Tie => {
                total += 3
            }
        }
        match player.as_str() {
            "X" => {
                total += 1
            },
            "Y" => {
                total += 2
            },
            "Z" => {
                total += 3
            },
            _ => {}
        }
        points += total
    }
    println!("{}", points);
}
