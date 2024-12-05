use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = env::args().nth(1).expect("No filename provided");
    let reader = read_file(filename).expect("Could not read file");

    let safe_reports_acc = reader
      .lines()
      .into_iter()
      .map(|line| {
        let seq: Vec<i32> = line
          .expect("Could not read line")
          .split_whitespace()
          .map(|x| x.parse::<i32>().expect("Could not parse int"))
          .collect();
        is_safe_with_fault_tolerance(&seq)
    }).filter(|x| *x).count();
    println!("Safe reports: {}", safe_reports_acc);
}

fn is_safe_with_fault_tolerance(seq: &Vec<i32>) -> bool {
  if is_safe(seq) {
    return true;
  }
  for i in 0..seq.len() {
    let mut seq_copy = seq.clone();
    seq_copy.remove(i);
    if is_safe(&seq_copy) {
      return true;
    }
  }
  false
}

fn is_safe(seq: &Vec<i32>) -> bool {
  if seq.len() < 2 {
    return false;
  }
  let is_first_asc = seq[0] < seq[1];
  for i in 0..(seq.len()-1) {
    let diff = (seq[i] - seq[i+1]).abs();
    let is_asc = seq[i] < seq[i+1];
    if diff < 1 || diff > 3 || is_asc != is_first_asc {
      return false;
    }
  }
  true
}

fn read_file<P>(filename: P) -> io::Result<io::BufReader<File>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}
