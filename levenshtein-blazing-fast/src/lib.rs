#![deny(clippy::all)]
extern crate levenshtein;

use levenshtein::levenshtein;

#[macro_use]
extern crate napi_derive;


#[napi]
pub fn find_most_similar(array: Vec<String>, target: String) -> String {
  let mut most_similar = String::new();
  let mut min_distance = usize::MAX;
  for s in array {
      let distance = levenshtein(&target, &s);
      if distance < min_distance {
          min_distance = distance;
          most_similar = s;
      }
  }

  return  most_similar.to_string();
}
#[napi]
pub fn levenshtein_distance(a: String, b: String) -> String {
  let a_str = a.to_string();
  let b_str = b.to_string();
  return  levenshtein(&a_str, &b_str).to_string();
}