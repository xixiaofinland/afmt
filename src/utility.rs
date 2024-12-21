use crate::{
    accessor::Accessor,
    data_model::*,
    enum_def::{Comparison, PropertyNavigation, SetValue, SoqlLiteral, ValueComparedWith},
};
use colored::Colorize;
#[allow(unused_imports)]
use log::debug;
use std::cell::{Cell, RefCell};
use tree_sitter::{Node, Range, Tree, TreeCursor};

thread_local! {
    static THREAD_SOURCE_CODE: Cell<Option<&'static str>>
        = const{ Cell::new(None) };
}
thread_local! {
    static THREAD_COMMENTS: RefCell<Vec<Comment>> = const { RefCell::new(Vec::new()) };
}
thread_local! {
    static THREAD_COMMENT_INDEX: Cell<usize> = const { Cell::new(0) };
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
pub fn get_source_code() -> &'static str {
    THREAD_SOURCE_CODE.with(|sc| sc.get().expect("Source code not set for this thread"))
}

///// Sets the comments for the current thread.
//pub fn set_thread_comments(comments: Vec<Comment>) {
//    THREAD_COMMENTS.with(|tc| {
//        *tc.borrow_mut() = comments;
//    });
//}
//
//pub fn peek_next_comment() -> Option<Comment> {
//    THREAD_COMMENTS.with(|tc| {
//        let comments = tc.borrow();
//        THREAD_COMMENT_INDEX.with(|index| {
//            if index.get() < comments.len() {
//                Some(comments[index.get()].clone())
//            } else {
//                None
//            }
//        })
//    })
//}
//
//pub fn consume_next_comment() -> Option<Comment> {
//    THREAD_COMMENTS.with(|tc| {
//        let comments = tc.borrow();
//        THREAD_COMMENT_INDEX.with(|index| {
//            let current = index.get();
//            if current < comments.len() {
//                index.set(current + 1);
//                return Some(comments[current].clone());
//            }
//            None
//        })
//    })
//}

pub fn collect_comments(cursor: &mut TreeCursor, comments: &mut Vec<Comment>) {
    loop {
        let node = cursor.node();
        if node.is_extra() {
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
        "## Expected node kind '{}', found '{}'.\n## Source_code: {}",
        expected_kind.yellow(),
        node.kind().red(),
        node.value()
    );
}

pub fn has_trailing_new_line(node: &Node) -> bool {
    let source_code = get_source_code();
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
            operator: operator_node.value(),
            compared_with,
        })
    } else if let Some(operator_node) = node.try_c_by_k("set_comparison_operator") {
        let next_node = operator_node.next_named();
        Comparison::Set(SetComparison {
            operator: operator_node.value(),
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

    let is_parent_a_chaining_node = is_a_chaining_node(&parent_node);

    let has_a_chaining_child = node
        .try_c_by_n("object")
        .map(|n| is_a_chaining_node(&n))
        .unwrap_or(false);

    if !is_parent_a_chaining_node && !has_a_chaining_child {
        return None;
    }

    let is_top_most_in_a_chain = has_a_chaining_child && !is_parent_a_chaining_node;

    Some(ChainingContext {
        is_top_most_in_a_chain,
        is_parent_a_chaining_node,
    })
}

fn is_a_chaining_node(node: &Node) -> bool {
    [
        "method_invocation",
        "array_access",
        "field_access",
        "query_expression",
    ]
    .contains(&node.kind())
}

pub fn panic_unknown_node_with_source_code(node: Node, name: &str) -> ! {
    panic!(
        "## unknown node: {} in {}\n## Source_code: {}",
        node.kind().red(),
        name,
        node.value()
    );
}

//pub fn associate_comments(range: Range) -> Option<CommentBuckets> {
//    let mut buckets = CommentBuckets::default();
//    let mut has_comments = false;
//
//    while let Some(comment) = get_next_comment() {
//        if comment.range.end_byte < range.start_byte {
//            buckets.pre_comments.push(comment);
//            has_comments = true;
//        } else if comment.range.start_byte > range.end_byte {
//            if is_immediately_following_line(&comment, &range) {
//                buckets.post_comments.push(comment);
//                has_comments = true;
//            } else {
//                break;
//            }
//        } else {
//            buckets.dangling_comments.push(comment);
//            has_comments = true;
//        }
//    }
//
//    if has_comments {
//        Some(buckets)
//    } else {
//        None
//    }
//}
//
//fn is_immediately_following_line(comment: &Comment, range: &Range) -> bool {
//    comment.range.start_point.row == range.end_point.row
//        || comment.range.start_point.row == range.end_point.row + 1
//}
