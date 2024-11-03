pub type DocRef<'a> = &'a Doc<'a>;

pub fn pretty_print(doc_ref: DocRef, max_width: u32) -> String {
    let mut printer = PrettyPrinter::new(doc_ref, max_width);
    printer.print()
}

#[derive(Debug)]
pub enum Doc<'a> {
    Newline,
    Text(String, u32), // The given text should not contain line breaks
    Flat(DocRef<'a>),
    Indent(u32, DocRef<'a>),
    Concat(Vec<DocRef<'a>>),
    Choice(DocRef<'a>, DocRef<'a>),
}

struct PrettyPrinter<'a> {
    max_width: u32,
    col: u32,
    chunks: Vec<Chunk<'a>>,
}

pub struct PrettyConfig {
    pub indent_size: u32,
}

impl PrettyConfig {
    pub fn new(indent_size: u32) -> Self {
        Self { indent_size }
    }
}

#[derive(Debug, Clone, Copy)]
struct Chunk<'a> {
    doc_ref: DocRef<'a>,
    indent: u32,
    flat: bool,
}

impl<'a> Chunk<'a> {
    fn with_doc(self, doc_ref: DocRef<'a>) -> Self {
        Chunk {
            doc_ref,
            indent: self.indent,
            flat: self.flat,
        }
    }

    fn indented(self, indent: u32, doc_ref: DocRef<'a>) -> Self {
        Chunk {
            doc_ref,
            indent: self.indent + indent,
            flat: self.flat,
        }
    }

    fn flat(self, doc_ref: DocRef<'a>) -> Self {
        Chunk {
            doc_ref,
            indent: self.indent,
            flat: true,
        }
    }
}

impl<'a> PrettyPrinter<'a> {
    fn new(doc_ref: DocRef<'a>, width: u32) -> Self {
        let chunk = Chunk {
            doc_ref,
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
            eprintln!("gopro[4]: doc.rs:85: chunk={:#?}", chunk);
            match chunk.doc_ref {
                Doc::Newline => {
                    result.push('\n');
                    for _ in 0..chunk.indent {
                        result.push(' ');
                    }
                    self.col = chunk.indent;
                }
                Doc::Text(text, width) => {
                    result.push_str(text);
                    self.col += width;
                }
                Doc::Flat(x) => self.chunks.push(chunk.flat(x)),
                Doc::Indent(i, x) => self.chunks.push(chunk.indented(*i, x)),
                Doc::Concat(seq) => {
                    for n in seq.iter().rev() {
                        self.chunks.push(chunk.with_doc(n));
                    }
                }
                Doc::Choice(x, y) => {
                    if chunk.flat || self.fits(chunk.with_doc(x)) {
                        self.chunks.push(chunk.with_doc(x));
                    } else {
                        self.chunks.push(chunk.with_doc(y));
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

            match chunk.doc_ref {
                Doc::Newline => return true,
                Doc::Text(_, text_width) => {
                    if *text_width <= remaining_width {
                        remaining_width -= text_width;
                    } else {
                        return false;
                    }
                }
                Doc::Flat(x) => stack.push(chunk.flat(x)),
                Doc::Indent(i, x) => stack.push(chunk.indented(*i, x)),
                Doc::Concat(seq) => {
                    for n in seq.iter().rev() {
                        stack.push(chunk.with_doc(n));
                    }
                }
                Doc::Choice(x, y) => {
                    if chunk.flat {
                        stack.push(chunk.with_doc(x));
                    } else {
                        // With assumption: for every choice `x | y`,
                        // the first line of `y` is no longer than the first line of `x`.
                        stack.push(chunk.with_doc(y));
                    }
                }
            }
        }
    }
}
