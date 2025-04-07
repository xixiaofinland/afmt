use crate::{
    accessor::Accessor,
    context::{Comment, CommentBucket, CommentMap, NodeContext},
    data_model::*,
    doc::{Doc, DocRef},
    doc_builder::DocBuilder,
    enum_def::{Comparison, SetValue, SoqlLiteral, ValueComparedWith},
    message_helper::{red, yellow},
};
use std::{cell::Cell, collections::HashMap};
use tree_sitter::{Node, Tree, TreeCursor};

const SNIPPET_MAX_LEN: usize = 80;

pub fn truncate_snippet(snippet: &str) -> String {
    if snippet.len() <= SNIPPET_MAX_LEN {
        snippet.to_string()
    } else {
        format!("{}…", &snippet[..SNIPPET_MAX_LEN])
    }
}

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

pub fn get_comment_map() -> &'static CommentMap {
    THREAD_COMMENT_MAP.with(|cm| cm.get().expect("## CommentMap not set for this thread"))
}

#[allow(dead_code)]
pub fn print_comment_map(tree: &Tree) {
    let comment_map = get_comment_map();
    let node_map = build_id_node_map(tree);

    let filtered_map: HashMap<usize, &CommentBucket> = comment_map
        .iter()
        .filter(|(_, bucket)| {
            !bucket.pre_comments.is_empty()
                || !bucket.post_comments.is_empty()
                || !bucket.dangling_comments.is_empty()
        })
        .map(|(k, v)| (*k, v))
        .collect();

    for (node_id, bucket) in &filtered_map {
        if let Some(node) = node_map.get(node_id) {
            eprintln!(
                "{}, {} ({}) : CommentBucket {{",
                node_id,
                yellow(node.kind()),
                yellow(&node.value().chars().take(8).collect::<String>())
            );
        } else {
            eprintln!("{} (Unknown Node) : CommentBucket {{", node_id);
        }
        eprintln!("pre_comments: {:#?},", bucket.pre_comments);
        eprintln!("post_comments: {:#?},", bucket.post_comments);
        eprintln!("dangling_comments: {:#?},", bucket.dangling_comments);
        eprintln!("--------------------");
    }
}

fn build_id_node_map(ast_tree: &Tree) -> HashMap<usize, Node> {
    let mut cursor = ast_tree.walk();
    let mut node_map = HashMap::new();

    loop {
        let node = cursor.node();
        node_map.insert(node.id(), node);

        if cursor.goto_first_child() {
            continue;
        }

        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return node_map;
            }
        }
    }
}

pub fn assert_no_missing_comments() {
    let missing_comments: Vec<&'static Comment> = get_comment_map()
        .values()
        .flat_map(|bucket| {
            bucket
                .pre_comments
                .iter()
                .chain(bucket.post_comments.iter())
                .chain(bucket.dangling_comments.iter())
        })
        .filter(|comment| !comment.is_printed())
        .collect();

    if !missing_comments.is_empty() {
        for comment in missing_comments {
            eprintln!("Erased comment: {}", red(&comment.value));
        }
        panic!("## There are erased comment node(s)");
    }
}

pub fn is_punctuation_node(node: &Node) -> bool {
    matches!(node.kind(), "," | ";")
}

fn is_associable_unnamed_node(node: &Node) -> bool {
    is_punctuation_node(node) || matches!(node.kind(), "else")
}

pub fn collect_comments(cursor: &mut TreeCursor, comment_map: &mut CommentMap) {
    let node = cursor.node();

    if (!node.is_named() || node.is_extra()) && !is_associable_unnamed_node(&node) {
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

    // We'll track comments that appear before the next assciable node in this vector
    let mut pending_pre_comments = Vec::new();
    // Track the last visited code node
    let mut last_associable_node_info: Option<(usize, usize)> = None;

    loop {
        let child = cursor.node();

        if child.is_extra() {
            // It's a comment node
            let comment = Comment::from_node(child);

            if let Some((last_id, last_row)) = last_associable_node_info {
                // We'll wrap the comment in an Option so we can move it exactly once
                let mut comment_opt = Some(comment);

                // Clone the cursor so we can safely peek siblings
                let mut peek_cursor = cursor.clone();

                // Continue until we either assign the comment or run out of siblings
                while let Some(c) = comment_opt.take() {
                    // If no next sibling, assign this comment to the last node's post_comments and stop
                    if !peek_cursor.goto_next_sibling() {
                        comment_map
                            .entry(last_id)
                            .or_insert_with(CommentBucket::new)
                            .post_comments
                            .push(c);
                        break;
                    }

                    let sibling = peek_cursor.node();

                    // If the sibling is another comment, skip it and put our comment back
                    if sibling.is_extra() {
                        comment_opt = Some(c);
                        continue;
                    }

                    // If the sibling is punctuation, assign as pre_comment of punctuation and stop
                    if is_punctuation_node(&sibling) {
                        let punc_id = sibling.id();
                        comment_map
                            .entry(punc_id)
                            .or_insert_with(CommentBucket::new)
                            .pre_comments
                            .push(c);
                        break;
                    } else {
                        // Otherwise, the sibling is a named node, no special treatment needed
                        if child.end_position().row == last_row {
                            comment_map
                                .entry(last_id)
                                .or_insert_with(CommentBucket::new)
                                .post_comments
                                .push(c);
                        } else {
                            pending_pre_comments.push(c);
                        }
                        break;
                    }
                }
            } else {
                // There's no "last associable node" yet, so keep it pending
                pending_pre_comments.push(comment);
            }
        } else if child.is_named() || is_associable_unnamed_node(&child) {
            // It's an associable node
            let child_id = child.id();

            // Assign any pending comments to the child's pre-comments
            if !pending_pre_comments.is_empty() {
                comment_map
                    .entry(child_id)
                    .or_insert_with(CommentBucket::new)
                    .pre_comments
                    .append(&mut pending_pre_comments);
            }

            // Recurse down into the child node
            collect_comments(cursor, comment_map);

            // After returning, we know child is fully processed
            last_associable_node_info = Some((child_id, child.end_position().row));
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }

    // After processing all children:
    if let Some((last_id, _)) = last_associable_node_info {
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

pub fn build_with_comments<'a, F>(
    b: &'a DocBuilder<'a>,
    node_context: &NodeContext,
    result: &mut Vec<DocRef<'a>>,
    handle_members: F,
) where
    F: FnOnce(&'a DocBuilder<'a>, &mut Vec<DocRef<'a>>),
{
    let bucket = get_comment_bucket(&node_context.id);
    handle_pre_comments(b, bucket, result);

    if bucket.dangling_comments.is_empty() {
        handle_members(b, result);
    } else {
        result.push(b.concat(handle_dangling_comments(b, bucket)));
        return;
    }

    handle_post_comments(b, bucket, result);
}

pub fn build_with_comments_core<'a, F>(
    b: &'a DocBuilder<'a>,
    node_context: &NodeContext,
    result: &mut Vec<DocRef<'a>>,
    handle_members: F,
) where
    F: FnOnce(&'a DocBuilder<'a>, &mut Vec<DocRef<'a>>),
{
    let bucket = get_comment_bucket(&node_context.id);
    handle_pre_comments(b, bucket, result);

    if bucket.dangling_comments.is_empty() {
        handle_members(b, result);
    } else {
        result.push(b.concat(handle_dangling_comments(b, bucket)));
    }
}

pub fn build_with_comments_and_punc<'a, F>(
    b: &'a DocBuilder<'a>,
    node_context: &NodeContext,
    result: &mut Vec<DocRef<'a>>,
    handle_members: F,
) where
    F: FnOnce(&'a DocBuilder<'a>, &mut Vec<DocRef<'a>>),
{
    build_with_comments_core(b, node_context, result, handle_members);

    let bucket = get_comment_bucket(&node_context.id);
    if bucket.dangling_comments.is_empty() {
        handle_post_comments(b, bucket, result);
    }

    if let Some(ref n) = node_context.punc {
        result.push(n.build(b));
    }
}

// fix: https://github.com/xixiaofinland/afmt/issues/114
pub fn build_with_comments_and_punc_attached<'a, F>(
    b: &'a DocBuilder<'a>,
    node_context: &NodeContext,
    result: &mut Vec<DocRef<'a>>,
    handle_members: F,
) where
    F: FnOnce(&'a DocBuilder<'a>, &mut Vec<DocRef<'a>>),
{
    build_with_comments_core(b, node_context, result, handle_members);

    let bucket = get_comment_bucket(&node_context.id);

    if let Some(ref n) = node_context.punc {
        result.push(n.build(b));
    }

    if bucket.dangling_comments.is_empty() {
        handle_post_comments(b, bucket, result);
    }
}

pub fn handle_dangling_comments_in_bracket_surround<'a>(
    b: &'a DocBuilder<'a>,
    bucket: &CommentBucket,
    result: &mut Vec<DocRef<'a>>,
) {
    result.push(b.txt("{"));
    result.push(b.indent(b.nl()));
    result.push(b.indent(b.concat(handle_dangling_comments(b, bucket))));
    result.push(b.nl());
    result.push(b.txt("}"));
}

pub fn handle_dangling_comments<'a>(
    b: &'a DocBuilder<'a>,
    bucket: &CommentBucket,
) -> Vec<&'a Doc<'a>> {
    if bucket.dangling_comments.is_empty() {
        panic!("handle_dangling_comments() should not have empty dangling_comments input")
    }

    let mut docs = Vec::new();
    for comment in &bucket.dangling_comments {
        if comment.has_leading_content() {
            docs.push(b.txt(" "));
        } else if comment.has_newline_above() {
            docs.push(b.empty_new_line());
        } else if comment.has_prev_node() {
            docs.push(b.nl());
        }

        docs.push(comment.build(b));

        //if comment.has_trailing_content() {
        //docs.push(b.txt(" "));
        //}

        comment.mark_as_printed();
    }
    docs
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
        } else {
            // if it's in group(), then multi-line mode is selected in fits()
            docs.push(b.force_break());

            // 1st element heading logic is handled in the preceding node;
            if i != 0 {
                if comment.has_newline_above() {
                    docs.push(b.empty_new_line());
                } else {
                    docs.push(b.nl());
                }
            }
        }

        docs.push(comment.build(b));

        if comment.has_trailing_content() {
            docs.push(b.txt(" "));
        } else if i == bucket.pre_comments.len() - 1 {
            if comment.has_newline_below() {
                docs.push(b.empty_new_line());
            } else {
                docs.push(b.nl());
            }
        }
        comment.mark_as_printed();
    }

    result.push(b.concat(docs));
}

pub fn handle_post_comments<'a>(
    b: &'a DocBuilder<'a>,
    bucket: &CommentBucket,
    result: &mut Vec<DocRef<'a>>,
) {
    if bucket.post_comments.is_empty() {
        return;
    }

    let mut docs = Vec::new();
    for comment in &bucket.post_comments {
        if comment.has_leading_content() {
            docs.push(b.txt(" "));
        } else if comment.has_newline_above() {
            docs.push(b.empty_new_line());
        } else {
            docs.push(b.nl());
        }

        docs.push(comment.build(b));

        if comment.has_trailing_content() && !comment.is_followed_by_bracket_composite_node() {
            docs.push(b.txt(" "));
        }

        comment.mark_as_printed();
    }
    result.push(b.concat(docs));
}

pub fn enrich(ast_tree: &Tree) -> Root {
    let root_node = ast_tree.root_node();
    Root::new(root_node)
    // TODO: check enum size
    //eprintln!("Root={:#?}", std::mem::size_of::<Root>());
    //eprintln!("Class={:#?}", std::mem::size_of::<FieldDeclaration>());
}

pub fn assert_check(node: Node, expected_kind: &str) {
    assert!(
        node.kind() == expected_kind,
        "## Expected node kind '{}', found '{}'.\n## Source_code: {}",
        yellow(expected_kind),
        red(node.kind()),
        node.value()
    );
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
        red(node.kind()),
        name,
        node.value()
    );
}

pub fn is_bracket_composite_node(node: &Node) -> bool {
    matches!(
        node.kind(),
        "trigger_body" | "class_body" | "block" | "enum_body"
    )
}
