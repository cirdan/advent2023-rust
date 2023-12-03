use std::fs;
use std::str::{Lines};

pub fn day2() {
    crate::advent::day_intro(2);
    const FILE_PATH: &str = "/usr/src/myapp/src/day2.txt";

    let _contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("Sum of possible games ids : {}", sum_possible_games_ids(_contents.lines()));
    println!("Sum of powers of games : {}", sum_games_powers(_contents.lines()));
}

pub fn sum_possible_games_ids(lines: Lines) -> u32 {
    lines.map(|line| {
        RGBGame::from_string(line).score(RGBCubeSet { red: 12, green: 13, blue: 14 })
    }).sum()
}

pub fn sum_games_powers(lines: Lines) -> u32 {
    lines.map(|line| RGBGame::from_string(line).smallest_possible_bag().power()).sum()
}

#[derive(PartialEq, Debug, Clone)]
pub struct RGBCubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(PartialEq, Debug)]
pub struct RGBGame {
    id: u32,
    draws: Vec<RGBCubeSet>,
}

trait CubeSet {
    fn power(&self) -> u32;
}


trait Game {
    fn from_string(string: &str) -> RGBGame;
    fn score(&self, bag: RGBCubeSet) -> u32;
    fn smallest_possible_bag(&self) -> RGBCubeSet;
}

trait CubesBag {
    fn can_provide(&self, draw: RGBCubeSet) -> bool;
}

trait Draw {
    fn from_string(draw_string: &str) -> Self;
}


impl Game for RGBGame {
    fn from_string(string: &str) -> Self {
        let mut split = string.split(":");
        let id = split.next().unwrap().replace("Game ", "").parse::<u32>().unwrap();
        let draws = split.next().unwrap()
            .split(";")
            .filter(|draw_as_string| !draw_as_string.trim().is_empty())
            .map(|draw_as_string| Draw::from_string(draw_as_string))
            .collect();
        RGBGame { id, draws }
    }
    fn score(&self, bag: RGBCubeSet) -> u32 {
        let draws = self.draws.iter().cloned();
        for draw in draws {
            if !bag.can_provide(draw) {
                return 0;
            }
        }
        self.id
    }
    fn smallest_possible_bag(&self) -> RGBCubeSet {
        let fewest = RGBCubeSet { red: 0, green: 0, blue: 0 };
        self.draws.iter().cloned()
            .reduce(
                |acc, current| RGBCubeSet {
                    red: acc.red.max(current.red),
                    green: acc.green.max(current.green),
                    blue: acc.blue.max(current.blue),
                })
            .unwrap_or(fewest)
    }
}

impl CubeSet for RGBCubeSet {
    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

impl CubesBag for RGBCubeSet {
    fn can_provide(&self, draw: RGBCubeSet) -> bool {
        self.red >= draw.red && self.green >= draw.green && self.blue >= draw.blue
    }
}

impl Draw for RGBCubeSet {
    fn from_string(draw_string: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        //TODO: functionalize
        for cube_def in draw_string.trim().split(',') {
            let mut split = cube_def.trim().split(" ");
            let count = split.next().unwrap().parse::<u32>().unwrap();
            let color = split.next().unwrap();
            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => panic!("Unknown color {}", color)
            }
        }
        RGBCubeSet { red, green, blue }
    }
}


#[cfg(test)]
mod tests {
    use std::ops::Not;
    use crate::day2::{RGBCubeSet, Draw, Game, RGBGame, CubesBag, CubeSet};

    #[test]
    fn parse_draw_from_string() {
        let draw = " 4 blue, 7 red, 2 green";
        assert_eq!(RGBCubeSet { red: 7, green: 2, blue: 4 }, Draw::from_string(draw))
    }

    #[test]
    fn parse_game_from_string() {
        let line = "Game 5: 4 blue, 7 red, 2 green; 1 blue, 1 red, 1 green;";
        assert_eq!(RGBGame {
            id: 5,
            draws: Vec::from([
                RGBCubeSet { red: 7, green: 2, blue: 4 },
                RGBCubeSet { red: 1, green: 1, blue: 1 },
            ]),
        }, RGBGame::from_string(line));
    }

    #[test]
    fn possible_draw_3_colors() {
        let draw = Draw::from_string("1 blue, 1 red, 1 green");
        let bag = RGBCubeSet { red: 1, green: 1, blue: 1 };
        assert!(bag.can_provide(draw));
    }

    #[test]
    fn possible_draw_2_colors() {
        let draw = Draw::from_string("1 blue");
        let bag = RGBCubeSet { red: 1, green: 1, blue: 1 };
        assert!(bag.can_provide(draw));
    }

    #[test]
    fn if_color_not_present_then_3_colors_draw_is_impossible() {
        let draw = Draw::from_string("2 blue");
        let bag = RGBCubeSet { red: 1, green: 1, blue: 1 };
        assert!(bag.can_provide(draw).not());
    }

    #[test]
    fn possible_draw_returns_id_as_score() {
        let game = <RGBGame as Game>::from_string("Game 8: 6 red, 11 green; 5 red, 2 blue, 7 green; 7 red, 6 green");
        let bag = RGBCubeSet { red: 100, green: 100, blue: 100 };
        assert!(game.score(bag) == 8);
    }

    #[test]
    fn fewest_cubeset_for_game_is_0_when_draw_is_all_0() {
        let game = <RGBGame as Game>::from_string("Game 8: 0 red, 0 green; 0 blue");
        assert!(game.smallest_possible_bag() == RGBCubeSet { red: 0, green: 0, blue: 0 });
    }

    #[test]
    fn fewest_cubeset_for_game_is_draw_when_one_draw_in_game() {
        let game = <RGBGame as Game>::from_string("Game 8: 1 red, 1 green; 1 blue");
        assert!(game.smallest_possible_bag() == RGBCubeSet { red: 1, green: 1, blue: 1 });
    }

    #[test]
    fn power_of_empty_cubeset_is_0() {
        let set = RGBCubeSet { red: 0, green: 0, blue: 0 };
        assert!(set.power() == 0);
    }

    #[test]
    fn power_of_cubeset_multiplies() {
        let set = RGBCubeSet { red: 2, green: 2, blue: 2 };
        assert!(set.power() == 8);
    }
}
