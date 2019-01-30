fn main() {
  let mut e = Encoder::new("abcab");
  e.encode();
  dbg!(e);

  let s = "abc def abc def";
  let idx = dbg!(s.find('b')).unwrap();
  dbg!(idx + s[(idx + 1)..].find('b').unwrap());

  // let mut iter = "abcdef".chars();
  // while let Some(ch) = iter.next() {
  //   dbg!(ch);
  //   iter.next();
  //   iter.next();
  //   iter.next();
  // }
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
    let mut possible_chunks = vec![];

    dbg!((c, idx));

    for scan_idx in 0..idx {
      // scan in parallel, from scan_idx and idx, forward to the end of the length of the input
      let mut possible_chunk = vec![];
      let mut scan_chars = self.input[scan_idx..idx].chars();
      let mut chars = self.input[idx..].chars();

      while true {
        let next_scan_char = scan_chars.next();
        let next_char = chars.next();
        dbg!((scan_idx, idx, next_scan_char, next_char));
        match (next_scan_char, next_char) {
          (Some(x), Some(y)) if x == y => {
            possible_chunk.push(x);
          }
          _ => {
            if possible_chunk.len() > 1 {
              possible_chunks.push(possible_chunk);
            }
            break;
          }
        }
      }
    }

    if possible_chunks.len() > 0 {
      Chunk {
        bytes: possible_chunks[0].clone(),
      }
    } else {
      Chunk { bytes: vec![c] }
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
