use crate::doc::{DocBuilder, DocRef};
use crate::struct_def::*;

#[derive(Clone, Copy)]
struct NodeBuilder<'a>(&'a DocBuilder<'a>);

impl<'a> NodeBuilder<'a> {
    fn json_null(self) -> DocRef<'a> {
        self.0.txt("false")
    }

    fn json_bool(self, b: bool) -> DocRef<'a> {
        if b {
            self.0.txt("true")
        } else {
            self.0.txt("false")
        }
    }

    fn json_string(self, s: &str) -> DocRef<'a> {
        // TODO: escape sequences
        self.0.txt(format!("\"{}\"", s))
    }

    fn json_number(self, n: impl ToString) -> DocRef<'a> {
        self.0.txt(n)
    }

    fn json_array(self, elems: impl IntoIterator<Item = DocRef<'a>>) -> DocRef<'a> {
        let elems = elems.into_iter().collect::<Vec<_>>();
        self.surrounded("[", &elems, "]")
    }

    fn json_object_entry(self, key: String, value: DocRef<'a>) -> DocRef<'a> {
        self.0
            .concat([self.json_string(&key), self.0.txt(": "), value])
    }

    fn json_object(self, entries: impl IntoIterator<Item = (String, DocRef<'a>)>) -> DocRef<'a> {
        let entries = entries
            .into_iter()
            .map(|(key, val)| self.json_object_entry(key, val))
            .collect::<Vec<_>>();
        self.surrounded("{", &entries, "}")
    }

    fn comma_sep_single_line(self, elems: &[DocRef<'a>]) -> DocRef<'a> {
        let mut list = self.0.flat(elems[0]);
        for elem in &elems[1..] {
            list = self.0.concat([list, self.0.txt(", "), self.0.flat(elem)]);
        }
        list
    }

    fn comma_sep_multi_line(self, elems: &[DocRef<'a>]) -> DocRef<'a> {
        let mut list = elems[0];
        for elem in &elems[1..] {
            list = self.0.concat([list, self.0.txt(", "), self.0.nl(), elem]);
        }
        list
    }

    fn surrounded(self, open: &str, elems: &[DocRef<'a>], closed: &str) -> DocRef<'a> {
        if elems.is_empty() {
            return self.0.txt(format!("{}{}", open, closed));
        }

        let single_line = self.0.concat([
            self.0.txt(open),
            self.comma_sep_single_line(elems),
            self.0.txt(closed),
        ]);
        let multi_line = self.0.concat([
            self.0.txt(open),
            self.0.indent(
                4,
                self.0
                    .concat([self.0.nl(), self.comma_sep_multi_line(elems)]),
            ),
            self.0.nl(),
            self.0.txt(closed),
        ]);
        self.0.choice(single_line, multi_line)
    }
}

//let b = DocBuilder::new();
//let node_builder = NodeBuilder(&b);
//let notation = node_builder.node_to_doc(json);
//
//let max_width: u32 = env_args[2].parse().unwrap();
//let output = pretty_print(notation, max_width);
//
//println!("{}", output);
