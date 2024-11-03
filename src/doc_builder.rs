use crate::doc::{Doc, DocRef};
use typed_arena::Arena;

pub struct DocBuilder<'a>(Arena<Doc<'a>>);

impl<'a> DocBuilder<'a> {
    pub fn new() -> DocBuilder<'a> {
        DocBuilder(Arena::new())
    }

    pub fn group(&'a self, doc_ref: DocRef<'a>) -> DocRef<'a> {
        let flat_doc = self.flat(doc_ref);
        self.choice(flat_doc, doc_ref)
    }

    pub fn softline(&'a self) -> DocRef<'a> {
        let space = self.txt(" ");
        let newline = self.nl();
        self.choice(space, newline)
    }

    pub fn maybeline(&'a self) -> DocRef<'a> {
        let empty = self.txt("");
        let newline = self.nl();
        self.choice(empty, newline)
    }

    fn comma_sep_single_line(&'a self, elems: &[DocRef<'a>]) -> DocRef<'a> {
        let mut list = self.flat(elems[0]);
        for elem in &elems[1..] {
            list = self.concat([list, self.txt(", "), self.flat(elem)]);
        }
        list
    }

    fn comma_sep_multi_line(&'a self, elems: &[DocRef<'a>]) -> DocRef<'a> {
        let mut list = elems[0];
        for elem in &elems[1..] {
            list = self.concat([list, self.txt(", "), self.nl(), elem]);
        }
        list
    }

    fn surrounded(&'a self, open: &str, elems: &[DocRef<'a>], closed: &str) -> DocRef<'a> {
        if elems.is_empty() {
            return self.txt(format!("{}{}", open, closed));
        }

        let single_line = self.concat([
            self.txt(open),
            self.comma_sep_single_line(elems),
            self.txt(closed),
        ]);
        let multi_line = self.concat([
            self.txt(open),
            self.indent(
                4,
                self.concat([self.nl(), self.comma_sep_multi_line(elems)]),
            ),
            self.nl(),
            self.txt(closed),
        ]);
        self.choice(single_line, multi_line)
    }

    // fundamental blocks

    pub fn nl(&'a self) -> DocRef<'a> {
        self.0.alloc(Doc::Newline)
    }

    pub fn txt(&'a self, text: impl ToString) -> DocRef<'a> {
        let string = text.to_string();
        let width = string.len() as u32;
        self.0.alloc(Doc::Text(string, width))
    }

    pub fn flat(&'a self, doc_ref: DocRef<'a>) -> DocRef<'a> {
        self.0.alloc(Doc::Flat(doc_ref))
    }

    pub fn indent(&'a self, indent: u32, doc_ref: DocRef<'a>) -> DocRef<'a> {
        self.0.alloc(Doc::Indent(indent, doc_ref))
    }

    pub fn concat(&'a self, doc_refs: impl IntoIterator<Item = DocRef<'a>>) -> DocRef<'a> {
        let n_vec = doc_refs.into_iter().collect::<Vec<_>>();
        self.0.alloc(Doc::Concat(n_vec))
    }

    pub fn choice(&'a self, first: DocRef<'a>, second: DocRef<'a>) -> DocRef<'a> {
        self.0.alloc(Doc::Choice(first, second))
    }
}
