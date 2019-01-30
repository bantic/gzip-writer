fn main() {
  let mut e = Encoder::new("abcab");
  e.encode();
  dbg!(e);

  let s = "abc def abc def";
  let idx = dbg!(s.find('b')).unwrap();
  dbg!(idx + s[(idx + 1)..].find('b').unwrap());
}

#[derive(Debug)]
pub struct Encoder<'a> {
  input: &'a str,
  chunks: Vec<Chunk>,
}

#[derive(Debug)]
struct Chunk {
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
    for (idx, c) in self.input.chars().enumerate() {
      self.chunks.push(self.scan(c, idx));
    }
  }

  fn scan(&self, c: char, idx: usize) -> Chunk {
    let remaining_len = self.input.len() - idx;
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
