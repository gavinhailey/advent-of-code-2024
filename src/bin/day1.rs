use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let filename = env::args().nth(1).expect("No filename provided");
    let reader = read_file(filename).expect("Could not read file");
    let mut lines = reader.lines();

    let (vec1, vec2) = get_sorted_columns(&mut lines).unwrap_or_else(|| {
      panic!("Could not get lists");
    });

    let freq_map2 = get_frequency_map(&vec2);

    let (mut similarity_acc, mut distance_acc) = (0, 0);
    for i in 0..vec1.len() {
      distance_acc += (vec1[i] - vec2[i]).abs();
      similarity_acc += vec1[i] * freq_map2.get(&vec1[i]).unwrap_or(&0);
    }
    println!("Total distance: {}", distance_acc);
    println!("Similarity score: {}", similarity_acc);
}

fn get_sorted_columns<I>(lines: I) -> Option<(Vec<i32>, Vec<i32>)>
  where I: Iterator<Item = io::Result<String>>,
{
  let (mut vec1, mut vec2): (Vec<i32>, Vec<i32>) = lines
    .flat_map(|line| {
      let vals = line.expect("Could not read line");
      let parts = vals.split_whitespace()
        .map(|x| x.parse::<i32>().expect("Could not parse int"))
        .collect::<Vec<i32>>();
      match parts.len() {
        2 => Some((parts[0], parts[1])),
        _ => None,
      }
    }).unzip();
  vec1.sort();
  vec2.sort();
  Some((vec1, vec2))
}

fn get_frequency_map(vec: &Vec<i32>) -> std::collections::HashMap<i32, i32> {
  vec.into_iter().fold(HashMap::new(), |mut map, val| {
    let count = map.entry(*val).or_insert(0);
    *count += 1;
    map
  })
}

fn read_file<P>(filename: P) -> io::Result<io::BufReader<File>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}
