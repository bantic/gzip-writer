fn main() {
  let mut e = Encoder::new(
    "
GREEN EGGS AND HAM (by Doctor Seuss) 

I AM SAM. I AM SAM. SAM I AM.

THAT SAM-I-AM! THAT SAM-I-AM! I DO NOT LIKE THAT SAM-I-AM!

DO WOULD YOU LIKE GREEN EGGS AND HAM?

I DO NOT LIKE THEM,SAM-I-AM.
I DO NOT LIKE GREEN EGGS AND HAM.

WOULD YOU LIKE THEM HERE OR THERE?

I WOULD NOT LIKE THEM HERE OR THERE.
I WOULD NOT LIKE THEM ANYWHERE.
I DO NOT LIKE GREEN EGGS AND HAM.
I DO NOT LIKE THEM, SAM-I-AM.

WOULD YOU LIKE THEM IN A HOUSE?
WOULD YOU LIKE THEN WITH A MOUSE?

I DO NOT LIKE THEM IN A HOUSE.
I DO NOT LIKE THEM WITH A MOUSE.
I DO NOT LIKE THEM HERE OR THERE.
I DO NOT LIKE THEM ANYWHERE.
I DO NOT LIKE GREEN EGGS AND HAM.
I DO NOT LIKE THEM, SAM-I-AM.
",
  );
  e.encode();

  print_chunks(e.chunks);
}

use colored::*;

pub fn print_chunks(chunks: Vec<Chunk>) {
  for chunk in chunks {
    let s = chunk.bytes.into_iter().collect::<String>();
    if s.len() > 1 {
      // TODO: Obvious gaps between non-matches
      print!("{}", s.green().bold());
    } else {
      print!("{}", s);
    }
  }
}

#[derive(Debug)]
pub struct Encoder<'a> {
  input: &'a str,
  chunks: Vec<Chunk>,
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
  bytes: Vec<char>,
}

impl<'a> Encoder<'a> {
  pub fn new(input: &'a str) -> Encoder {
    Encoder {
      input,
      chunks: vec![],
    }
  }

  pub fn encode(&mut self) {
    let mut iter = self.input.chars().enumerate();

    while let Some((idx, c)) = iter.next() {
      let chunk = self.scan(c, idx);
      for _ in 0..(chunk.bytes.len() - 1) {
        iter.next();
      }
      self.chunks.push(chunk);
    }
  }

  fn scan(&self, c: char, idx: usize) -> Chunk {
    let mut possible_chunks = vec![vec![c]];

    for scan_idx in 0..idx {
      let mut possible_chunk = vec![];
      let mut scan_chars = self.input[scan_idx..idx].chars();
      let mut chars = self.input[idx..].chars();

      loop {
        match (scan_chars.next(), chars.next()) {
          (Some(x), Some(y)) if x == y => {
            possible_chunk.push(x);
          }
          _ => {
            if possible_chunk.len() > 0 {
              possible_chunks.push(possible_chunk);
            }
            break;
          }
        }
      }
    }

    // TODO: Is there a way to do this without all the to-vec-ing?
    let longest_chunk = possible_chunks
      .iter()
      .fold(possible_chunks[0].to_vec(), |acc, item| {
        if item.len() > acc.len() {
          item.to_vec()
        } else {
          acc
        }
      });

    Chunk {
      bytes: longest_chunk.to_vec(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Chunk, Encoder};

  fn encode(s: &str) -> Vec<Chunk> {
    let mut e = Encoder::new(s);
    e.encode();
    e.chunks
  }

  fn chunks(cs: Vec<&str>) -> Vec<Chunk> {
    let mut chunks = vec![];
    for c in cs {
      chunks.push(Chunk {
        bytes: c.chars().collect(),
      });
    }
    chunks
  }

  #[test]
  fn no_reps() {
    assert_eq!(encode("abc"), chunks(vec!["a", "b", "c"]));
  }

  #[test]
  fn rep_of_len_1() {
    assert_eq!(encode("abca"), chunks(vec!["a", "b", "c", "a"]));
  }

  #[test]
  fn rep_of_len_2_end() {
    assert_eq!(encode("abcab"), chunks(vec!["a", "b", "c", "ab"]));
  }

  #[test]
  fn rep_of_len_2_mid() {
    assert_eq!(encode("abcabd"), chunks(vec!["a", "b", "c", "ab", "d"]));
  }

  #[test]
  fn rep_of_len_2_mid_count_3() {
    assert_eq!(
      encode("abcabdabe"),
      chunks(vec!["a", "b", "c", "ab", "d", "ab", "e"])
    );
  }

  #[test]
  fn rep_of_inc_lengths() {
    assert_eq!(
      encode("ababcabcd"),
      chunks(vec!["a", "b", "ab", "c", "abc", "d"])
    );
  }
}
