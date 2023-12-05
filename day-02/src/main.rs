// Load Local Modules
mod read;

// Imports
use day_02::Game;
use day_02::parse_game;


fn main() {

    let input = read::read_lines("input.txt".to_owned());
    let games: Vec<Game> = input.into_iter()
        .map(|line| parse_game(line))
        .collect();
    
    // Part 1
    let possible_games: usize = games.iter()
        .map(|game| (game.game_number, game.check_possible_contain_only(12, 13, 14)))
        .filter(|&(_, possible)| possible)
        .map(|(game_number, _)| game_number)
        .sum();
    println!("\r🧊 Possible games: '{}' (Part 1)", possible_games);
    
    // Part 2
    let minimum_power: usize = games.iter()
        .map(|game| game.get_minimum_power_game())
        .sum();
    println!("\r🧊 Minimum power required: '{}' (Part 2)", minimum_power);
}