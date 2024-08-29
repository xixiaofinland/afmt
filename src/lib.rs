pub mod config;
mod extension;
mod macros;
mod node_struct;
mod utility;
mod visitor;

use anyhow::Result;
use config::{Args, Config, Session};

pub fn format(args: Args) -> Vec<Result<String>> {
    let config = Config::new(120);
    let source_files = vec![args.path];
    let session = Session::new(config, source_files);
    session.format()
}

//pub fn format_code(source_code: &str) -> Result<String> {
//
//}

/// The content of the [`node-types.json`][] file for this grammar.
//pub const NODE_TYPES: &str = include_str!("node-types.json");

// Uncomment these to include any queries that this grammar contains
// pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");
// pub const INJECTIONS_QUERY: &str = include_str!("../../queries/injections.scm");
// pub const LOCALS_QUERY: &str = include_str!("../../queries/locals.scm");
// pub const TAGS_QUERY: &str = include_str!("../../queries/tags.scm");

#[cfg(test)]
mod tests {
    use crate::config::language;

    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(language())
            .expect("Error loading Apex grammar");
    }
}
