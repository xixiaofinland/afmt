use crate::{
    accessor::Accessor,
    context::{Comment, CommentBucket, CommentMap},
    data_model::*,
    doc::DocRef,
    doc_builder::DocBuilder,
    enum_def::{Comparison, PropertyNavigation, SetValue, SoqlLiteral, ValueComparedWith},
};
use colored::Colorize;
#[allow(unused_imports)]
use log::debug;
use std::cell::Cell;
use tree_sitter::{Node, Tree, TreeCursor};

thread_local! {
    static THREAD_SOURCE_CODE: Cell<Option<&'static str>>
        = const{ Cell::new(None) };
}

pub fn set_thread_source_code(source_code: String) {
    // TODO: use OnceCell to not leak?
    let leaked_code: &'static str = Box::leak(source_code.into_boxed_str());
    THREAD_SOURCE_CODE.with(|sc| {
        if sc.get().is_some() {
            panic!("Source code is already set for this thread");
        }
        sc.set(Some(leaked_code));
    });
}

pub fn get_source_code() -> &'static str {
    THREAD_SOURCE_CODE.with(|sc| sc.get().expect("Source code not set for this thread"))
}

thread_local! {
    static THREAD_COMMENT_MAP: Cell<Option<&'static CommentMap>> = const{ Cell::new(None) };
}

pub fn set_thread_comment_map(comment_map: CommentMap) {
    // TODO: use OnceCell to not leak?
    let leaked_map: &'static CommentMap = Box::leak(Box::new(comment_map));

    THREAD_COMMENT_MAP.with(|cm| {
        if cm.get().is_some() {
            panic!("CommentMap is already set for this thread");
        }
        cm.set(Some(leaked_map));
    });
}

pub fn get_comment_bucket(node_id: &usize) -> &CommentBucket {
    get_comment_map()
        .get(node_id)
        .unwrap_or_else(|| panic!("## comment_map missing bucket for node: {}", node_id))
}

fn get_comment_map() -> &'static CommentMap {
    THREAD_COMMENT_MAP.with(|cm| cm.get().expect("## CommentMap not set for this thread"))
}

pub fn collect_comments(cursor: &mut TreeCursor, comment_map: &mut CommentMap) {
    let node = cursor.node();

    if !node.is_named() || node.is_extra() {
        return;
    }

    let current_id = node.id();
    comment_map
        .entry(current_id)
        .or_insert_with(CommentBucket::new);

    // If this node has no children, we simply return
    if !cursor.goto_first_child() {
        return;
    }

    // We'll track comments that appear before the next code node in this vector
    let mut pending_pre_comments = Vec::new();
    // Track the last visited code node
    let mut last_code_node_id: Option<usize> = None;

    loop {
        let child = cursor.node();

        if child.is_named() {
            if child.is_extra() {
                // It's a comment node => treat as "pending pre-comment"
                pending_pre_comments.push(Comment::from_node(child));
            } else {
                // It's a child code node
                let child_id = child.id();

                // Assign any pending comments to the child's pre-comments
                if !pending_pre_comments.is_empty() {
                    comment_map
                        .entry(child_id)
                        .or_insert_with(CommentBucket::new)
                        .pre_comments
                        .append(&mut pending_pre_comments);
                }

                // Recurse down into the child code node
                collect_comments(cursor, comment_map);

                // After returning, we know child is fully processed
                last_code_node_id = Some(child_id);
            }
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }

    // After processing all children:
    if let Some(last_id) = last_code_node_id {
        // Assign remaining pending comments as "post" for the last code node
        comment_map
            .entry(last_id)
            .or_insert_with(CommentBucket::new)
            .post_comments
            .append(&mut pending_pre_comments);
    } else {
        // No code children => treat all as "dangling" for the current node
        comment_map
            .entry(current_id)
            .or_insert_with(CommentBucket::new)
            .dangling_comments
            .append(&mut pending_pre_comments);
    }

    // Step back up to the parent node
    cursor.goto_parent();
}

pub fn handle_dangling_comments<'a>(
    b: &'a DocBuilder<'a>,
    bucket: &CommentBucket,
    result: &mut Vec<DocRef<'a>>,
) -> bool {
    if bucket.dangling_comments.is_empty() {
        return false;
    }

    let docs: Vec<_> = bucket
        .dangling_comments
        .iter()
        .map(|comment_node| comment_node.build(b))
        .collect();
    result.push(b.concat(docs));
    true
}

pub fn handle_pre_comments<'a>(
    b: &'a DocBuilder<'a>,
    bucket: &CommentBucket,
    result: &mut Vec<DocRef<'a>>,
) {
    if bucket.pre_comments.is_empty() {
        return;
    }

    let mut docs = Vec::new();
    for (i, comment) in bucket.pre_comments.iter().enumerate() {
        if comment.has_leading_content() {
            docs.push(b.txt(" "));
        } else if comment.has_prev_node() {
            docs.push(b.nl());
        }

        if comment.print_newline_above() {
            docs.push(b.nl());
        }

        docs.push(comment.build(b));

        if i == bucket.pre_comments.len() - 1 {
            if comment.has_trailing_content() {
                docs.push(b.txt(" "));
            } else {
                docs.push(b.nl());
            }

            if comment.print_newline_below() {
                docs.push(b.nl());
            }
        }
    }

    result.push(b.concat(docs));
}

/// Handles post-comments by printing them after processing the node.
pub fn handle_post_comments<'a>(
    b: &'a DocBuilder<'a>,
    bucket: &CommentBucket,
    result: &mut Vec<DocRef<'a>>,
) {
    if bucket.post_comments.is_empty() {
        return;
    }

    let mut docs = Vec::new();
    for (i, comment) in bucket.post_comments.iter().enumerate() {
        if comment.has_leading_content() {
            docs.push(b.txt(" "));
        } else {
            docs.push(b.nl());
        }

        if comment.print_newline_above() {
            docs.push(b.nl());
        }

        docs.push(comment.build(b));

        if i == bucket.post_comments.len() - 1 && !comment.has_next_node() {
            if comment.has_trailing_content() {
                docs.push(b.txt(" "));
            }

            if comment.print_newline_below() {
                docs.push(b.nl());
                docs.push(b.nl());
            }
        }
    }

    result.push(b.concat(docs));
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

pub fn panic_unknown_node(node: Node, name: &str) -> ! {
    panic!(
        "## unknown node: {} in {}\n## Source_code: {}",
        node.kind().red(),
        name,
        node.value()
    );
}

pub fn get_comment_children(node: Node) -> Vec<Comment> {
    node.all_children_vec()
        .into_iter()
        .filter(|n| n.is_extra())
        .map(|n| Comment::from_node(n))
        .collect()
}
