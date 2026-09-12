use crate::context::CommentMap;
use crate::utility::{
    clear_thread_comment_map, clear_thread_source_code, clear_thread_source_origin,
    collect_comments, set_thread_comment_map, set_thread_source_code, set_thread_source_origin,
};
use tree_sitter::Tree;

#[cfg(test)]
use std::cell::Cell;

#[cfg(test)]
thread_local! {
    static PANIC_AFTER_SETUP: Cell<bool> = const { Cell::new(false) };
}

pub(crate) struct FormattingSession;

impl FormattingSession {
    /// `origin` names where the source came from — a path, `<stdin>` — and is
    /// used only to locate diagnostics. `None` when the caller has no name for
    /// it.
    pub(crate) fn new(source_code: &str, ast_tree: &Tree, origin: Option<&str>) -> Self {
        clear_thread_source_code();
        clear_thread_comment_map();
        clear_thread_source_origin();
        let session = Self;

        set_thread_source_code(source_code.to_string());
        set_thread_source_origin(origin.map(str::to_string));

        let mut cursor = ast_tree.walk();
        let mut comment_map = CommentMap::new();
        collect_comments(&mut cursor, &mut comment_map);

        set_thread_comment_map(comment_map);

        session
    }

    #[cfg(test)]
    pub(crate) fn panic_after_setup_if_requested() {
        PANIC_AFTER_SETUP.with(|requested| {
            if requested.replace(false) {
                panic!("test panic after formatting-session setup");
            }
        });
    }

    #[cfg(test)]
    pub(crate) fn request_panic_after_setup() {
        PANIC_AFTER_SETUP.with(|requested| requested.set(true));
    }
}

impl Drop for FormattingSession {
    fn drop(&mut self) {
        clear_thread_source_code();
        clear_thread_comment_map();
        clear_thread_source_origin();
    }
}
