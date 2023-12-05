// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
pub type GameNumber = usize;

pub struct Record {
  count_red: usize,
  count_green: usize,
  count_blue: usize,
}

pub struct Game {
  pub game_number: GameNumber,
  counts: Vec<Record>
}

// =============================================== AUXILIARY FUNCTIONS ===============================================
fn parse_record(record_info: &str) -> Record {
  let mut count_red: usize = 0;
  let mut count_green: usize = 0;
  let mut count_blue: usize = 0;

  record_info.split(", ")
    .into_iter()
    .for_each(|count_info| {
      let mut count_info_split: Vec<&str> = count_info.split(" ").collect();
      let count_number: usize = count_info_split.remove(0).parse().unwrap(); 
      
      match count_info_split.remove(0) {
        "red" => count_red = count_number,
        "green" => count_green = count_number,
        "blue" => count_blue = count_number,
        _ => panic!("🚨 Unrecognized ball color")
      }
    });

  Record { count_red, count_green, count_blue }

}

pub fn parse_game(line: String) -> Game {
  let mut split_header: Vec<&str> = line.split(": ").collect();
  let game_number: GameNumber = split_header.remove(0)
    .strip_prefix("Game ").unwrap()
    .parse().unwrap();

  let records: Vec<Record> = split_header.remove(0)
    .split("; ")
    .map(|record_info| parse_record(record_info))
    .collect();

  Game { game_number, counts: records }
}

// ================================================= IMPLEMENTATIONS =================================================
impl Record {
  fn check_possible_contain_only(&self, red_cubes: usize, green_cubes: usize, blue_cubes: usize) -> bool {
    self.count_red <= red_cubes &&
      self.count_green <= green_cubes &&
      self.count_blue <= blue_cubes
  }
}

impl Game {

  pub fn check_possible_contain_only(&self, red_cubes: usize, green_cubes: usize, blue_cubes: usize) -> bool {
    self.counts.iter()
      .map(|record| record.check_possible_contain_only(red_cubes, green_cubes, blue_cubes))
      .all(|possible| possible)
  }

  fn get_minimum_playable(&self) -> (usize, usize, usize) {
    let mut minimum_red: Option<usize> = None;
    let mut minimum_green: Option<usize> = None;
    let mut minimum_blue: Option<usize> = None;

    fn set_maximum_optional(curr: Option<usize>, value: usize) -> Option<usize> {
      match curr {
        Some(curr_value) if value > curr_value => Some(value),
        Some(curr_value) => Some(curr_value),
        None => Some(value)
      }
    }

    self.counts.iter()
      .for_each(|count| {
        minimum_red = set_maximum_optional(minimum_red, count.count_red);
        minimum_green = set_maximum_optional(minimum_green, count.count_green);
        minimum_blue = set_maximum_optional(minimum_blue, count.count_blue);
      });

    (minimum_red.unwrap(), minimum_green.unwrap(), minimum_blue.unwrap())
  }

  pub fn get_minimum_power_game(&self) -> usize {
    let (min_red, min_green, min_blue) = self.get_minimum_playable();
    min_red * min_green * min_blue
  }
}