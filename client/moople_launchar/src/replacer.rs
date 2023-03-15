use std::io::{self, Read, Write};

use memchr::memmem::{self, Finder};

fn read_up_to(r: &mut impl std::io::Read, mut buf: &mut [u8]) -> Result<usize, std::io::Error> {
    let buf_len = buf.len();

    while !buf.is_empty() {
        match r.read(buf) {
            Ok(0) => break,
            Ok(n) => {
                let tmp = buf;
                buf = &mut tmp[n..];
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(buf_len - buf.len())
}

#[derive(Debug)]
pub enum Patch {
    Replace {
        needle: &'static [u8],
        replace: &'static [u8],
    },
}

impl Patch {
    pub fn replace(needle: &'static [u8], replace: &'static  [u8]) -> Self {
        assert_eq!(needle.len(), replace.len());
        Self::Replace { needle, replace }
    }
}

pub struct Replacer<R, W> {
    rdr: R,
    wrt: W,
    patches: Vec<Patch>,
}

impl<R, W> Replacer<R, W>
where
    R: Read,
    W: Write,
{
    pub fn new(r: R, w: W, p: Vec<Patch>) -> Self {
        Self {
            rdr: r,
            wrt: w,
            patches: p,
        }
    }

    fn patch_block(block: &mut [u8], replacments: &[(Finder, &[u8])]) {
        for (finder, replace) in replacments.iter() {
            let pos: Vec<_> = finder.find_iter(block).collect();
            for p in pos {
                block[p..p + replace.len()].copy_from_slice(replace);
            }
        }
    }

    pub fn run<const BLOCK_SIZE: usize>(&mut self) -> io::Result<()> {
        let replacments: Vec<_> = self
            .patches
            .iter()
            .filter_map(|patch| match patch {
                Patch::Replace { needle, replace } => Some((memmem::Finder::new(needle), *replace)),
                //_ => None,
            })
            .collect();

        let max_overlap: usize = replacments
            .iter()
            .map(|(f, _)| f.needle().len())
            .max()
            .unwrap_or(0);

        assert!(max_overlap < BLOCK_SIZE);

        // Use double buffering to allow to work accross block boundaries
        let mut block = vec![0u8; BLOCK_SIZE * 2];

        //Read first block
        let mut block_ln = read_up_to(&mut self.rdr, &mut block[..BLOCK_SIZE])?;

        while block_ln == BLOCK_SIZE {
            // Read next block
            block_ln = read_up_to(&mut self.rdr, &mut block[BLOCK_SIZE..])?;

            let m = (BLOCK_SIZE + max_overlap).min(BLOCK_SIZE + block_ln);
            Self::patch_block(&mut block[..m], replacments.as_slice());

            // Write patched block
            self.wrt.write_all(&block[..BLOCK_SIZE])?;

            // Move peeked as the next block to process
            block.copy_within(BLOCK_SIZE.., 0);
        }

        Self::patch_block(&mut block[..block_ln], replacments.as_slice());
        self.wrt.write_all(&block[..block_ln])?;

        Ok(())
    }

    pub fn into_writer(self) -> W {
        self.wrt
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::{Patch, Replacer};

    #[test]
    fn replacer() {
        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut w: Vec<u8> = vec![];

        let patches = vec![
            Patch::Replace {
                needle: &[1],
                replace: &[11],
            },
            Patch::Replace {
                needle: &[0],
                replace: &[11],
            },
            Patch::Replace {
                needle: &[4, 5],
                replace: &[15, 14],
            },
            Patch::Replace {
                needle: &[7, 8],
                replace: &[17, 18],
            },
        ];

        let mut replacer = Replacer::new(Cursor::new(data), &mut w, patches);
        replacer.run::<4>().unwrap();
        let w = replacer.into_writer();

        assert_eq!(w.as_slice(), &[11, 2, 3, 15, 14, 6, 17, 18, 9]);
    }
}
