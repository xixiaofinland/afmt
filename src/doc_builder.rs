use crate::{
    data_model::DocBuild,
    doc::{Doc, DocRef, PrettyConfig},
};
use typed_arena::Arena;

pub struct DocBuilder<'a> {
    arena: Arena<Doc<'a>>,
    config: PrettyConfig,
}

impl<'a> DocBuilder<'a> {
    pub fn new(config: PrettyConfig) -> Self {
        Self {
            arena: Arena::new(),
            config,
        }
    }

    pub fn nil(&'a self) -> DocRef<'a> {
        self.txt("")
    }

    pub fn sep_single_line(&'a self, elems: &[DocRef<'a>], separator: &str) -> DocRef<'a> {
        elems.iter().skip(1).fold(
            if let Some(&first) = elems.get(0) {
                self.flat(first)
            } else {
                self.nil()
            },
            |acc, &elem| self.concat(vec![acc, self.txt(separator), self.flat(elem)]),
        )
    }

    pub fn sep_multi_line(&'a self, elems: &[DocRef<'a>], separator: &str) -> DocRef<'a> {
        elems.iter().skip(1).fold(
            if let Some(&first) = elems.get(0) {
                first
            } else {
                self.nil()
            },
            |acc, &elem| self.concat(vec![acc, self.txt(separator), self.nl(), elem]),
        )
    }

    pub fn separated_choice(
        &'a self,
        elems: &[DocRef<'a>],
        single_sep: &str,
        multi_sep: &str,
    ) -> DocRef<'a> {
        if elems.is_empty() {
            return self.nil();
        }

        let single_line = self.sep_single_line(elems, single_sep);

        let multi_line = self.add_indent_level(self.sep_multi_line(elems, multi_sep));

        self.choice(single_line, multi_line)
    }

    pub fn surrounded(
        &'a self,
        elems: &[DocRef<'a>],
        single_sep: &str,
        multi_sep: &str,
        open: &str,
        closed: &str,
    ) -> DocRef<'a> {
        if elems.is_empty() {
            return self.txt(format!("{}{}", open, closed));
        }

        let single_line = self.concat(vec![
            self.txt(open),
            self.sep_single_line(elems, single_sep),
            self.txt(closed),
        ]);

        let multi_line = self.concat(vec![
            self.txt(open),
            self.add_indent_level(self.sep_multi_line(elems, multi_sep)),
            self.nl(),
            self.txt(closed),
        ]);

        self.choice(single_line, multi_line)
    }

    pub fn pretty_surrounded(
        &'a self,
        elems: &[DocRef<'a>],
        single_sep: &str,
        multi_sep: &str,
        open: &str,
        closed: &str,
    ) -> DocRef<'a> {
        if elems.is_empty() {
            return self.txt(format!("{}{}", open, closed));
        }

        let single_line = self.concat(vec![
            self.txt(open),
            self.sep_single_line(elems, single_sep),
            self.txt(closed),
        ]);

        let multi_line = self.concat(vec![
            self.txt(open),
            self.add_indent_level(self.nl()),
            self.add_indent_level(self.sep_multi_line(elems, multi_sep)),
            self.nl(),
            self.txt(closed),
        ]);

        self.choice(single_line, multi_line)
    }

    //pub fn join_with_doc_sep(&'a self, elems: &[DocRef<'a>], separator: DocRef<'a>) -> DocRef<'a> {
    //    if elems.is_empty() {
    //        return self.nil();
    //    }
    //
    //    elems.iter().skip(1).fold(elems[0], |acc, &elem| {
    //        self.concat(vec![acc, separator, elem])
    //    })
    //}

    pub fn build_docs<'b, T: DocBuild<'a>>(
        &'a self,
        items: impl IntoIterator<Item = &'b T>,
    ) -> Vec<DocRef<'a>>
    where
        T: DocBuild<'a> + 'b,
    {
        items.into_iter().map(|item| item.build(self)).collect()
    }

    pub fn nl(&'a self) -> DocRef<'a> {
        self.arena.alloc(Doc::Newline)
    }

    pub fn txt(&'a self, text: impl ToString) -> DocRef<'a> {
        let s = text.to_string();
        let width = s.len() as u32;
        self.arena.alloc(Doc::Text(s, width))
    }

    pub fn _txt(&'a self, text: impl ToString) -> DocRef<'a> {
        let s = text.to_string();
        let space_s = format!(" {}", s);
        self.txt(space_s)
    }

    pub fn txt_(&'a self, text: impl ToString) -> DocRef<'a> {
        let s = text.to_string();
        let s_space = format!("{} ", s);
        self.txt(s_space)
    }

    pub fn _txt_(&'a self, text: impl ToString) -> DocRef<'a> {
        let s = text.to_string();
        let space_s_space = format!(" {} ", s);
        self.txt(space_s_space)
    }

    pub fn flat(&'a self, doc_ref: DocRef<'a>) -> DocRef<'a> {
        self.arena.alloc(Doc::Flat(doc_ref))
    }

    pub fn add_indent_level(&'a self, doc_ref: DocRef<'a>) -> DocRef<'a> {
        let relative_indent = self.config.indent_size;
        self.arena.alloc(Doc::Indent(relative_indent, doc_ref))
    }

    //pub fn add_indent_level(&'a self, levels: u32, doc_ref: DocRef<'a>) -> DocRef<'a> {
    //    let relative_indent = levels * self.config.indent_size;
    //    self.arena.alloc(Doc::Indent(relative_indent, doc_ref))
    //}

    pub fn concat(&'a self, doc_refs: impl IntoIterator<Item = DocRef<'a>>) -> DocRef<'a> {
        let n_vec = doc_refs.into_iter().collect::<Vec<_>>();
        self.arena.alloc(Doc::Concat(n_vec))
    }

    pub fn choice(&'a self, first: DocRef<'a>, second: DocRef<'a>) -> DocRef<'a> {
        self.arena.alloc(Doc::Choice(first, second))
    }
}
