use typed_arena::Arena;

pub type DocRef<'a> = &'a Doc<'a>;

#[derive(Debug)]
pub enum Doc<'a> {
    Newline,
    Text(String, u32), // The given text should not contain line breaks
    Flat(DocRef<'a>),
    Indent(u32, DocRef<'a>),
    Concat(Vec<DocRef<'a>>),
    Choice(DocRef<'a>, DocRef<'a>),
}

pub struct DocBuilder<'a>(Arena<Doc<'a>>);

impl<'a> DocBuilder<'a> {
    pub fn new() -> DocBuilder<'a> {
        DocBuilder(Arena::new())
    }

    pub fn nl(&'a self) -> DocRef<'a> {
        self.0.alloc(Doc::Newline)
    }

    pub fn txt(&'a self, text: impl ToString) -> DocRef<'a> {
        let string = text.to_string();
        let width = string.len() as u32;
        self.0.alloc(Doc::Text(string, width))
    }

    pub fn flat(&'a self, n_ref: DocRef<'a>) -> DocRef<'a> {
        self.0.alloc(Doc::Flat(n_ref))
    }

    pub fn indent(&'a self, indent: u32, n_ref: DocRef<'a>) -> DocRef<'a> {
        self.0.alloc(Doc::Indent(indent, n_ref))
    }

    pub fn concat(&'a self, n_refs: impl IntoIterator<Item = DocRef<'a>>) -> DocRef<'a> {
        let n_vec = n_refs.into_iter().collect::<Vec<_>>();
        self.0.alloc(Doc::Concat(n_vec))
    }

    pub fn choice(&'a self, first: DocRef<'a>, second: DocRef<'a>) -> DocRef<'a> {
        self.0.alloc(Doc::Choice(first, second))
    }
}
