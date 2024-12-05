use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct LevelValidity {
  asc: bool,
  desc: bool,
  diff: bool,
}

fn main() {
    let filename = env::args().nth(1).expect("No filename provided");
    let reader = read_file(filename).expect("Could not read file");

    let safe_reports_acc = reader.lines().map(|line| {
      let vals = line.expect("Could not read line");

      let mut parts = vals.split_whitespace()
        .map(|x| x.parse::<i32>().expect("Could not parse int"))
        .collect::<Vec<i32>>();

      println!("Parts: {:?}", parts);
      let invalid_index = check_sequence(&parts);
      if let Some(i) = invalid_index {
        parts.remove(i);
        if let Some(j) = check_sequence(&parts) {
          println!("Invalid index: {}", j);
          return false;
        }
      }
      true
    }).filter(|x| *x).count();
    println!("Safe reports: {}", safe_reports_acc);
}

fn check_sequence(seq: &Vec<i32>) -> Vec<usize> {
  let mut invalid_indices = None;
  let (mut only_asc, mut only_desc) = (true, true);
  for i in 0..seq.len() {
    let j = i + 1;
    if j >= seq.len() {
      break;
    }
    let qual = validate_levels(seq[i], seq[j]);
    only_asc &= qual.asc;
    only_desc &= qual.desc;
    if (!only_asc && !only_desc) || qual.diff {
      invalid_index = Some(i);
      break;
    }
  }
  invalid_index
}

fn validate_levels(a: i32, b: i32) -> LevelValidity {
  let diff = (a - b).abs();
  LevelValidity {
    asc: a < b,
    desc: a > b,
    diff: diff < 1 || diff > 3,
  }
}

fn read_file<P>(filename: P) -> io::Result<io::BufReader<File>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}
