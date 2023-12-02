use std::fs::read_to_string;

/// https://adventofcode.com/2023/day/2

fn main() {
    // Read input from file.
    let input = read_to_string("input.txt").unwrap();

    // Split input into lines and filter out empty lines.
    let lines = input.split('\n').filter(|line| !line.is_empty());

    // Collect a list of games containing their respective cubes (color and amount). The concept of
    // a set is thrown out completely after this map since it's not needed for any of the solutions.
    let games = lines.map(|line| {
        // Separate the line into the game ID and the subsets of cubes.
        let (game, sets) = line.split_once(": ").unwrap();

        // Create a flat iterator over all cubes in each set.
        let cubes = sets.split("; ").flat_map(|set| {
            set.split(", ").map(|cubes| {
                let (amount, color) = cubes.split_once(' ').unwrap();
                let amount = amount.parse::<usize>().unwrap();

                (color, amount)
            })
        });

        // Parse the game ID.
        let game_id = game
            .strip_prefix("Game ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        // Return cubes and game ID.
        (cubes, game_id)
    });

    // Part 1 solution: Get the sum of all possible games.
    let sum_of_possible_games = games
        .clone()
        .filter_map(|(mut cubes, game_id)| {
            const NUMBER_OF_RED_CUBES: usize = 12;
            const NUMBER_OF_GREEN_CUBES: usize = 13;
            const NUMBER_OF_BLUE_CUBES: usize = 14;

            // The game is considered possible if every set of cubes can be shown with our
            // predefined number of colored cubes.
            let is_possible = cubes.all(|(color, amount)| match color {
                "red" => amount <= NUMBER_OF_RED_CUBES,
                "green" => amount <= NUMBER_OF_GREEN_CUBES,
                "blue" => amount <= NUMBER_OF_BLUE_CUBES,
                _ => panic!(),
            });

            // If it is possible, return the game id.
            is_possible.then_some(game_id)
        })
        .sum::<usize>();

    // Part 2 solution: Get the sum of the power of all sets.
    let power_of_all_sets = games
        .map(|(cubes, _game_id)| {
            // Matches on the color of cubes and applies [`std::cmp::max`] with the accumulator and the amount of cubes.
            let collect_cubes = |[red, green, blue]: [usize; 3], (color, amount)| match color {
                "red" => [red.max(amount), green, blue],
                "green" => [red, green.max(amount), blue],
                "blue" => [red, green, blue.max(amount)],
                _ => panic!(),
            };

            // Calculate the minimum number of cubes needed for each color, then take their product.
            cubes
                .fold([0, 0, 0], collect_cubes)
                .into_iter()
                .product::<usize>()
        })
        .sum::<usize>();

    // Solution for part 1.
    println!(
        "The sum of all possible game IDs is {}",
        sum_of_possible_games
    );

    // Solution for part 2.
    println!("The power of all sets combined is {}", power_of_all_sets);
}
