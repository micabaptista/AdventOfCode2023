// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
pub type CalibrationLine = Vec<char>;
pub type CalibrationDigit = u32;

pub struct CalibrationValue {
  pub first_digit: CalibrationDigit,
  pub second_digit: CalibrationDigit,
}

// =============================================== AUXILIARY FUNCTIONS ===============================================
fn find_pattern<'a>(string: &String, patterns: &'a Vec<(&str, &str)>) -> Option<(&'a str, &'a str, usize)> {
  patterns.iter()
    .map(|&pattern| (pattern.0, pattern.1, string.find(pattern.0)))
    .filter(|pattern| pattern.2.is_some())
    .map(|pattern| (pattern.0, pattern.1, pattern.2.unwrap()))
    .min_by_key(|elem| elem.2)
}

fn fix_calibration_values(line: CalibrationLine) -> CalibrationLine {
  let patterns: Vec<(&str, &str)> = vec![
    ("zero", "0"), ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"),
    ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")
  ];

  let mut string: String = line.iter().collect();
  while let Some(occurrence) = find_pattern(&string, &patterns) {
    let range_start = occurrence.2;
    let range_end = occurrence.2 + 1;

    string.replace_range(range_start..range_end, occurrence.1);
  }

  string.chars().into_iter().collect()
}

pub fn retrieve_calibration_value(line: CalibrationLine, fix_written: bool) -> Option<CalibrationValue> {
  let mut line = line;
  if fix_written { line = fix_calibration_values(line) }
  
  line.into_iter()
    .filter_map(|char| char.to_digit(10))
    .fold(None, |acc, elem| {
      match acc {
        None => Some(CalibrationValue::new(elem, elem)),
        Some(prev_value) => Some(CalibrationValue::new(prev_value.first_digit, elem))
      }
    })
}

// ================================================= IMPLEMENTATIONS =================================================

impl CalibrationValue {

  fn new(first_value: CalibrationDigit, second_value: CalibrationDigit) -> Self {
    Self { first_digit: first_value, second_digit: second_value }
  }

  pub fn get_value(&self) -> u32 {
    self.first_digit * 10 + self.second_digit
  } 
}
