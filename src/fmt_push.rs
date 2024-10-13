use crate::shape::Shape;

pub trait Pushable {
    fn push_to_string(self, s: &mut String) -> usize;
}

impl Pushable for &str {
    #[inline]
    fn push_to_string(self, s: &mut String) -> usize {
        let len = self.len();
        s.push_str(self);
        len
    }
}

impl Pushable for char {
    #[inline]
    fn push_to_string(self, s: &mut String) -> usize {
        s.push(self);
        1
    }
}

impl Pushable for &String {
    #[inline]
    fn push_to_string(self, s: &mut String) -> usize {
        let len = self.len();
        s.push_str(self);
        len
    }
}

pub trait FmtPush {
    fn fmt_push<P: Pushable>(&mut self, item: P, shape: &mut Shape);
}

impl FmtPush for String {
    #[inline]
    fn fmt_push<P: Pushable>(&mut self, item: P, shape: &mut Shape) {
        let len = item.push_to_string(self);
        shape.offset += len;
    }
}
