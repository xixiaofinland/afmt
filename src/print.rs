use crate::notation::{NRef, N};

pub fn pretty_print(n_ref: NRef, max_width: u32) -> String {
    let mut printer = PrettyPrinter::new(n_ref, max_width);
    printer.print()
}

struct PrettyPrinter<'a> {
    max_width: u32,
    col: u32,
    chunks: Vec<Chunk<'a>>,
}

#[derive(Debug, Clone, Copy)]
struct Chunk<'a> {
    n_ref: NRef<'a>,
    indent: u32,
    flat: bool,
}

impl<'a> Chunk<'a> {
    fn with_n(self, n_ref: NRef<'a>) -> Self {
        Chunk {
            n_ref,
            indent: self.indent,
            flat: self.flat,
        }
    }

    fn indented(self, indent: u32, n_ref: NRef<'a>) -> Self {
        Chunk {
            n_ref,
            indent: self.indent + indent,
            flat: self.flat,
        }
    }

    fn flat(self, n_ref: NRef<'a>) -> Self {
        Chunk {
            n_ref,
            indent: self.indent,
            flat: true,
        }
    }
}

impl<'a> PrettyPrinter<'a> {
    fn new(n_ref: NRef<'a>, width: u32) -> Self {
        let chunk = Chunk {
            n_ref,
            indent: 0,
            flat: false,
        };

        Self {
            max_width: width,
            col: 0,
            chunks: vec![chunk],
        }
    }

    fn print(&mut self) -> String {
        let mut result = String::new();

        while let Some(chunk) = self.chunks.pop() {
            match chunk.n_ref {
                N::Newline => {
                    result.push('\n');
                    for _ in 0..chunk.indent {
                        result.push(' ');
                    }
                    self.col = chunk.indent;
                }
                N::Text(text, width) => {
                    result.push_str(text);
                    self.col += width;
                }
                N::Flat(x) => self.chunks.push(chunk.flat(x)),
                N::Indent(i, x) => self.chunks.push(chunk.indented(*i, x)),
                N::Concat(seq) => {
                    for n in seq.iter().rev() {
                        self.chunks.push(chunk.with_n(n));
                    }
                }
                N::Choice(x, y) => {
                    if chunk.flat || self.fits(chunk.with_n(x)) {
                        self.chunks.push(chunk.with_n(x));
                    } else {
                        self.chunks.push(chunk.with_n(y));
                    }
                }
            }
        }
        result
    }

    fn fits(&self, chunk: Chunk<'a>) -> bool {
        let mut remaining_width = self.max_width.saturating_sub(self.col);
        let mut stack = vec![chunk];
        let mut chunks = &self.chunks as &[Chunk];

        loop {
            let chunk = if let Some(chunk) = stack.pop() {
                chunk
            } else if let Some((chunk, more_chunks)) = chunks.split_last() {
                chunks = more_chunks;
                *chunk
            } else {
                return true;
            };

            match chunk.n_ref {
                N::Newline => return true,
                N::Text(_, text_width) => {
                    if *text_width <= remaining_width {
                        remaining_width -= text_width;
                    } else {
                        return false;
                    }
                }
                N::Flat(x) => stack.push(chunk.flat(x)),
                N::Indent(i, x) => stack.push(chunk.indented(*i, x)),
                N::Concat(seq) => {
                    for n in seq.iter().rev() {
                        stack.push(chunk.with_n(n));
                    }
                }
                N::Choice(x, y) => {
                    if chunk.flat {
                        stack.push(chunk.with_n(x));
                    } else {
                        // With assumption: for every choice `x | y`,
                        // the first line of `y` is no longer than the first line of `x`.
                        stack.push(chunk.with_n(y));
                    }
                }
            }
        }
    }
}
