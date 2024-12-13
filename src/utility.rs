use crate::{
    accessor::Accessor,
    data_model::*,
    enum_def::{Comparison, PropertyNavigation, SetValue, SoqlLiteral, ValueComparedWith},
};
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
        "?" | ":" => 2,                               // Ternary
        "||" => 3,                                    // Logical OR
        "??" => 3,                                    // Null-coalescing
        "&&" => 5,                                    // Logical AND
        "|" => 6,                                     // Bitwise OR
        "^" => 7,                                     // Bitwise XOR
        "&" => 8,                                     // Bitwise AND
        "==" | "!=" | "===" | "!==" | "<>" => 9,      // Equality
        ">" | "<" | ">=" | "<=" | "instanceof" => 10, // Relational
        "<<" | ">>" | ">>>" => 11,                    // Shift
        "+" | "-" => 12,                              // Additive
        "*" | "/" | "%" => 13,                        // Multiplicative
        "!" | "~" | "++" | "--" => 14,                // Unary operators
        _ => panic!("## Not supported operator: {}", op),
    }
}

pub fn is_method_invocation(node: &Node) -> bool {
    node.kind() == "method_invocation"
}

pub fn is_binary_exp(node: &Node) -> bool {
    node.kind() == "binary_expression"
}

pub fn is_query_expression(node: &Node) -> bool {
    node.kind() == "query_expression"
}

// TODO: AST use a comparison concrete node so this can be moved into Comparison::new()
// TODO: get rid of next_named()?
pub fn get_comparsion(node: &Node) -> Comparison {
    if let Some(operator_node) = node.try_c_by_k("value_comparison_operator") {
        let next_node = operator_node.next_named();
        let compared_with = match next_node.kind() {
            "bound_apex_expression" => {
                ValueComparedWith::Bound(BoundApexExpression::new(next_node))
            }
            _ => ValueComparedWith::Literal(SoqlLiteral::new(next_node)),
        };

        Comparison::Value(ValueComparison {
            operator: operator_node.value(source_code()),
            compared_with,
        })
    } else if let Some(operator_node) = node.try_c_by_k("set_comparison_operator") {
        let next_node = operator_node.next_named();
        Comparison::Set(SetComparison {
            operator: operator_node.value(source_code()),
            set_value: SetValue::new(next_node),
        })
    } else {
        unreachable!()
    }
}

pub fn get_property_navigation(parent_node: &Node) -> PropertyNavigation {
    if parent_node.try_c_by_k("safe_navigation_operator").is_some() {
        PropertyNavigation::SafeNavigationOperator
    } else {
        PropertyNavigation::Dot
    }
}

pub fn build_chaining_context(node: &Node) -> Option<ChainingContext> {
    let parent_node = node
        .parent()
        .expect("node must have parent node in build_chaining_context()");

    let is_parent_a_chaining_node = is_a_possible_chaining_node(&parent_node);

    let mut has_a_chaining_child = false;

    let has_a_chaining_child = node
        .try_c_by_n("object")
        .map(|n| is_a_possible_chaining_node(&n))
        .unwrap_or(false);

    if !is_parent_a_chaining_node && !has_a_chaining_child {
        return None;
    }

    let is_top_most_in_nest = has_a_chaining_child && !is_parent_a_chaining_node;

    Some(ChainingContext {
        is_top_most_in_nest,
        is_parent_a_chaining_node,
    })
}

fn is_a_possible_chaining_node(node: &Node) -> bool {
    [
        "method_invocation",
        "array_access",
        "field_access",
        "query_expression",
    ]
    .contains(&node.kind())
}
