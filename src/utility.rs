use crate::data_model::*;
use colored::Colorize;
#[allow(unused_imports)]
use log::debug;
use std::cell::Cell;
use tree_sitter::{Node, Tree, TreeCursor};

thread_local! {
    static THREAD_SOURCE_CODE: Cell<Option<&'static str>> = Cell::new(None);
}

/// Sets the source code for the current thread.
/// This should be called once per thread before processing.
pub fn set_thread_source_code(code: String) {
    // Leak the `String` to obtain a `&'static str`
    let leaked_code: &'static str = Box::leak(code.into_boxed_str());
    THREAD_SOURCE_CODE.with(|sc| {
        sc.set(Some(leaked_code));
    });
}

/// Retrieves the source code for the current thread.
/// Panics if the source code has not been set.
pub fn source_code() -> &'static str {
    THREAD_SOURCE_CODE.with(|sc| sc.get().expect("Source code not set for this thread"))
}

pub fn collect_comments(cursor: &mut TreeCursor, comments: &mut Vec<Comment>) {
    loop {
        let node = cursor.node();
        if node.is_named() && node.is_extra() {
            comments.push(Comment::from_node(node));
        }

        if cursor.goto_first_child() {
            collect_comments(cursor, comments);
            cursor.goto_parent();
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

pub fn enrich(ast_tree: &Tree) -> Root {
    let root_node = ast_tree.root_node();
    Root::new(root_node)
    //eprintln!("Root={:#?}", std::mem::size_of::<Root>());
    //eprintln!("Class={:#?}", std::mem::size_of::<FieldDeclaration>());
}

pub fn assert_check(node: Node, expected_kind: &str) {
    assert!(
        node.kind() == expected_kind,
        "Expected node kind '{}', found '{}'",
        expected_kind.yellow(),
        node.kind().red()
    );
}

pub fn has_trailing_new_line(node: &Node) -> bool {
    let source_code = source_code();
    let index = node.end_byte();

    // Ensure the index is within bounds
    if index >= source_code.len() {
        return false;
    }

    let remaining_code = &source_code[index..];
    let mut newline_count = 0;

    for char in remaining_code.chars() {
        match char {
            '\n' => {
                newline_count += 1;
                if newline_count >= 2 {
                    break; // Found two consecutive newlines
                }
            }
            ' ' | '\t' | '\r' => continue, // Skip other whitespace
            _ => break,                    // Encountered a non-whitespace character
        }
    }

    newline_count >= 2 // Return true if there are two or more consecutive newlines
}

pub fn get_precedence(op: &str) -> u8 {
    match op {
        "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>=" | ">>>=" => 1, // Assignment
        "?" | ":" => 2,                              // Ternary
        "||" => 3,                                   // Logical OR
        "??" => 3, // Null-coalescing
        "&&" => 5,                                   // Logical AND
        "|" => 6,                                    // Bitwise OR
        "^" => 7,                                    // Bitwise XOR
        "&" => 8,                                    // Bitwise AND
        "==" | "!=" | "===" | "!==" | "<>" => 9,     // Equality
        ">" | "<" | ">=" | "<=" | "instanceof" => 10, // Relational
        "<<" | ">>" | ">>>" => 11,                   // Shift
        "+" | "-" => 12,                             // Additive
        "*" | "/" | "%" => 13,                       // Multiplicative
        "!" | "~" | "++" | "--" => 14,               // Unary operators
        _ => panic!("## Not supported operator: {}", op),
    }
}

pub fn isBinaryNode(node: Node) -> bool {
    node.kind() == "binary_expression"
}
