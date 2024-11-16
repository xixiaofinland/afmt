use crate::{
    data_model::DocBuild,
    doc::{Doc, DocRef, PrettyConfig},
    enum_def::BodyMember,
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

    pub fn surround_with_newline(
        &'a self,
        elems: &[DocRef<'a>],
        sep: &str,
        open: &str,
        close: &str,
    ) -> DocRef<'a> {
        if elems.is_empty() {
            return self.concat(vec![self.txt("{"), self.nl(), self.txt("}")]);
        }
        self.surround_with_sep_and_newline(elems, sep, open, close)
    }

    pub fn surround_with_trailing_newline_considered<M>(
        &'a self,
        elems: &[BodyMember<M>],
        open: &str,
        close: &str,
    ) -> DocRef<'a>
    where
        M: DocBuild<'a>,
    {
        if elems.is_empty() {
            return self.concat(vec![self.txt("{"), self.nl(), self.txt("}")]);
        }

        let multi_line = self.concat(vec![
            self.txt(open),
            self.add_indent_level(self.nl()),
            self.add_indent_level(self.split_with_trailing_newline_considered(elems)),
            self.nl(),
            self.txt(close),
        ]);
        multi_line
    }

    pub fn surround_with_softline(
        &'a self,
        elems: &[DocRef<'a>],
        sep: &str,
        open: &str,
        close: &str,
    ) -> DocRef<'a> {
        let single_sep = format!("{} ", sep);
        let single_line = self.surround_single_line(elems, &single_sep, open, close);
        let multi_line = self.surround_with_sep_and_softline(elems, sep, open, close);
        self.choice(single_line, multi_line)
    }

    pub fn surround_with_maybeline(
        &'a self,
        elems: &[DocRef<'a>],
        sep: &str,
        open: &str,
        close: &str,
    ) -> DocRef<'a> {
        let single_line = self.surround_single_line(elems, sep, open, close);
        let multi_line = self.surround_with_sep_and_maybeline(elems, sep, open, close);
        self.choice(single_line, multi_line)
    }

    fn surround_with_sep_and_newline(
        &'a self,
        elems: &[DocRef<'a>],
        sep: &str,
        open: &str,
        close: &str,
    ) -> DocRef<'a> {
        if elems.is_empty() {
            return self.txt(format!("{}{}", open, close));
        }

        let multi_line = self.concat(vec![
            self.txt(open),
            self.add_indent_level(self.softline()),
            self.add_indent_level(self.intersperse_with_sep_and_newline(elems, sep)),
            self.softline(),
            self.txt(close),
        ]);
        multi_line
    }

    fn surround_with_sep_and_softline(
        &'a self,
        elems: &[DocRef<'a>],
        sep: &str,
        open: &str,
        close: &str,
    ) -> DocRef<'a> {
        if elems.is_empty() {
            return self.txt(format!("{}{}", open, close));
        }

        let multi_line = self.concat(vec![
            self.txt(open),
            self.add_indent_level(self.softline()),
            self.add_indent_level(self.intersperse_with_sep_and_softline(elems, sep)),
            self.softline(),
            self.txt(close),
        ]);
        multi_line
    }

    fn surround_with_sep_and_maybeline(
        &'a self,
        elems: &[DocRef<'a>],
        sep: &str,
        open: &str,
        close: &str,
    ) -> DocRef<'a> {
        if elems.is_empty() {
            return self.txt(format!("{}{}", open, close));
        }

        let multi_line = self.concat(vec![
            self.txt(open),
            self.add_indent_level(self.softline()),
            self.add_indent_level(self.intersperse_with_sep_and_maybeline(elems, sep)),
            self.softline(),
            self.txt(close),
        ]);
        multi_line
    }

    fn surround_single_line(
        &'a self,
        elems: &[DocRef<'a>],
        sep: &str,
        open: &str,
        close: &str,
    ) -> DocRef<'a> {
        if elems.is_empty() {
            return self.txt(format!("{}{}", open, close));
        }

        let single_line = self.concat(vec![
            self.txt(open),
            self.intersperse_single_line(elems, sep),
            self.txt(close),
        ]);
        single_line
    }

    pub fn group_elems_with_softline(&'a self, elems: &[DocRef<'a>], sep: &str) -> DocRef<'a> {
        let choice = self.intersperse_with_sep_and_softline(&elems, &sep);
        self.add_indent_level(self.group(choice))
    }

    pub fn intersperse_single_line(&'a self, elems: &[DocRef<'a>], sep: &str) -> DocRef<'a> {
        if elems.is_empty() {
            return self.nil();
        }

        let mut parts = Vec::with_capacity(elems.len() * 2 - 1);
        for (i, &elem) in elems.iter().enumerate() {
            if i > 0 {
                parts.push(self.txt(sep));
            }
            parts.push(self.flat(elem));
        }
        self.concat(parts)
    }

    pub fn intersperse_with_sep_and_newline(
        &'a self,
        elems: &[DocRef<'a>],
        sep: &str,
    ) -> DocRef<'a> {
        if elems.is_empty() {
            return self.nil();
        }

        let mut parts = Vec::with_capacity(elems.len() * 2 - 1);
        for (i, &elem) in elems.iter().enumerate() {
            if i > 0 {
                parts.push(self.txt(sep));
                parts.push(self.nl());
            }
            parts.push(elem);
        }

        self.concat(parts)
    }

    fn intersperse_with_sep_and_softline(&'a self, elems: &[DocRef<'a>], sep: &str) -> DocRef<'a> {
        if elems.is_empty() {
            return self.nil();
        }

        let mut parts = Vec::with_capacity(elems.len() * 2 - 1);
        for (i, &elem) in elems.iter().enumerate() {
            if i > 0 {
                parts.push(self.txt(sep));
                parts.push(self.softline());
            }
            parts.push(elem);
        }

        self.concat(parts)
    }

    fn intersperse_with_sep_and_maybeline(&'a self, elems: &[DocRef<'a>], sep: &str) -> DocRef<'a> {
        if elems.is_empty() {
            return self.nil();
        }

        let mut parts = Vec::with_capacity(elems.len() * 2 - 1);
        for (i, &elem) in elems.iter().enumerate() {
            if i > 0 {
                parts.push(self.txt(sep));
                parts.push(self.maybeline());
            }
            parts.push(elem);
        }
        self.concat(parts)
    }

    pub fn split_with_trailing_newline_considered<'b, M>(
        &'a self,
        members: &[BodyMember<M>],
    ) -> DocRef<'a>
    where
        M: DocBuild<'a>,
    {
        if members.is_empty() {
            return self.nil();
        }

        let mut member_docs = Vec::new();
        for (i, m) in members.iter().enumerate() {
            member_docs.push(m.member.build(self));

            if i < members.len() - 1 {
                if m.has_trailing_newlines {
                    member_docs.push(self.nl_with_no_indent());
                }
                member_docs.push(self.nl());
            }
        }
        self.concat(member_docs)
    }

    pub fn to_docs<'b, T: DocBuild<'a>>(
        &'a self,
        items: impl IntoIterator<Item = &'b T>,
    ) -> Vec<DocRef<'a>>
    where
        T: DocBuild<'a> + 'b,
    {
        items.into_iter().map(|item| item.build(self)).collect()
    }

    pub fn nil(&'a self) -> DocRef<'a> {
        self.txt("")
    }

    pub fn nl(&'a self) -> DocRef<'a> {
        self.arena.alloc(Doc::Newline)
    }

    pub fn softline(&'a self) -> DocRef<'a> {
        self.arena.alloc(Doc::Softline)
    }

    pub fn maybeline(&'a self) -> DocRef<'a> {
        self.arena.alloc(Doc::Maybeline)
    }

    pub fn nl_with_no_indent(&'a self) -> DocRef<'a> {
        self.arena.alloc(Doc::NewlineWithNoIndent)
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

    pub fn concat(&'a self, doc_refs: impl IntoIterator<Item = DocRef<'a>>) -> DocRef<'a> {
        let n_vec = doc_refs.into_iter().collect::<Vec<_>>();
        self.arena.alloc(Doc::Concat(n_vec))
    }

    pub fn choice(&'a self, first: DocRef<'a>, second: DocRef<'a>) -> DocRef<'a> {
        self.arena.alloc(Doc::Choice(first, second))
    }

    pub fn group(&'a self, doc: DocRef<'a>) -> DocRef<'a> {
        self.choice(self.flat(doc), doc)
    }
}
