use typed_arena::Arena;

pub type NRef<'a> = &'a N<'a>;

// `Notation`, equals the `Doc` in Wadler's Printer
#[derive(Debug)]
pub enum N<'a> {
    Newline,
    Text(String, u32), // The given text should not contain line breaks
    Flat(NRef<'a>),
    Indent(u32, NRef<'a>),
    Concat(Vec<NRef<'a>>),
    Choice(NRef<'a>, NRef<'a>),
}

pub struct NBuilder<'a>(Arena<N<'a>>);

impl<'a> NBuilder<'a> {
    pub fn new() -> NBuilder<'a> {
        NBuilder(Arena::new())
    }

    pub fn nl(&'a self) -> NRef<'a> {
        self.0.alloc(N::Newline)
    }

    pub fn txt(&'a self, text: impl ToString) -> NRef<'a> {
        let string = text.to_string();
        let width = string.len() as u32;
        self.0.alloc(N::Text(string, width))
    }

    pub fn flat(&'a self, n_ref: NRef<'a>) -> NRef<'a> {
        self.0.alloc(N::Flat(n_ref))
    }

    pub fn indent(&'a self, indent: u32, n_ref: NRef<'a>) -> NRef<'a> {
        self.0.alloc(N::Indent(indent, n_ref))
    }

    pub fn concat(&'a self, n_refs: impl IntoIterator<Item = NRef<'a>>) -> NRef<'a> {
        let n_vec = n_refs.into_iter().collect::<Vec<_>>();
        self.0.alloc(N::Concat(n_vec))
    }

    pub fn choice(&'a self, first: NRef<'a>, second: NRef<'a>) -> NRef<'a> {
        self.0.alloc(N::Choice(first, second))
    }
}
