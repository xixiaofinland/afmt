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

    pub fn surround(
        &'a self,
        elems: &[DocRef<'a>],
        sep: Insertable<'a>,
        open: Insertable<'a>,
        close: Insertable<'a>,
    ) -> DocRef<'a> {
        if elems.is_empty() {
            return self.concat(vec![
                self.txt(open.str.unwrap()),
                self.txt(close.str.unwrap()),
            ]);
        }

        let mut docs = Vec::new();

        if let Some(o_str) = open.str {
            docs.push(self.txt(o_str));
        }
        if let Some(o_doc) = open.doc {
            docs.push(self.indent(o_doc));
        }

        docs.push(self.indent(self.intersperse(elems, sep)));

        if let Some(c_doc) = close.doc {
            docs.push(c_doc);
        }
        if let Some(c_str) = close.str {
            docs.push(self.txt(c_str));
        }

        self.concat(docs)
    }

    pub fn surround_without_indent(
        &'a self,
        elems: &[DocRef<'a>],
        sep: Insertable<'a>,
        open: Insertable<'a>,
        close: Insertable<'a>,
    ) -> DocRef<'a> {
        if elems.is_empty() {
            return self.concat(vec![
                self.txt(open.str.unwrap()),
                self.txt(close.str.unwrap()),
            ]);
        }

        let mut docs = Vec::new();

        if let Some(o_str) = open.str {
            docs.push(self.txt(o_str));
        }
        if let Some(o_doc) = open.doc {
            docs.push(o_doc);
        }

        docs.push(self.intersperse(elems, sep));

        if let Some(c_doc) = close.doc {
            docs.push(c_doc);
        }
        if let Some(c_str) = close.str {
            docs.push(self.txt(c_str));
        }

        self.concat(docs)
    }

    pub fn group_then_indent(&'a self, doc: DocRef<'a>) -> DocRef<'a> {
        self.indent(self.group(doc))
    }

    pub fn group_iter(&'a self, doc_refs: impl IntoIterator<Item = DocRef<'a>>) -> DocRef<'a> {
        self.group(self.concat(doc_refs))
    }

    pub fn intersperse(&'a self, elems: &[DocRef<'a>], sep: Insertable<'a>) -> DocRef<'a> {
        if elems.is_empty() {
            return self.nil();
        }

        let mut parts = Vec::with_capacity(elems.len() * 2 - 1);
        for (i, &elem) in elems.iter().enumerate() {
            if i > 0 {
                if let Some(ref sep_str) = sep.str {
                    parts.push(self.txt(sep_str));
                }
                if let Some(sep_doc) = sep.doc {
                    parts.push(sep_doc);
                }
            }
            parts.push(elem);
        }
        self.concat(parts)
    }

    pub fn surround_body<M>(
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
            self.indent(self.nl()),
            self.indent(self.intersperse_body_members(elems)),
            self.nl(),
            self.txt(close),
        ]);
        multi_line
    }

    pub fn intersperse_body_members<'b, M>(&'a self, members: &[BodyMember<M>]) -> DocRef<'a>
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
                if m.has_trailing_newline {
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

    pub fn indent(&'a self, doc_ref: DocRef<'a>) -> DocRef<'a> {
        let relative_indent = self.config.indent_size;
        self.arena.alloc(Doc::Indent(relative_indent, doc_ref))
    }

    pub fn dedent(&'a self, doc_ref: DocRef<'a>) -> DocRef<'a> {
        let relative_indent = self.config.indent_size;
        self.arena.alloc(Doc::Dedent(relative_indent, doc_ref))
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

pub struct Insertable<'a> {
    pub str: Option<String>,
    pub doc: Option<DocRef<'a>>,
}

impl<'a> Insertable<'a> {
    pub fn new<S: ToString>(s: Option<S>, doc: Option<DocRef<'a>>) -> Self {
        Self {
            str: s.map(|s_value| s_value.to_string()),
            doc,
        }
    }
}

impl<'a> DocBuild<'a> for Insertable<'a> {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        if let Some(ref s) = self.str {
            result.push(b.txt(s));
        }
        if let Some(d) = self.doc {
            result.push(d);
        }
    }
}
