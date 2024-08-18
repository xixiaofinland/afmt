#[derive(Debug)]
pub enum TNode<'a> {
    Const(&'a ast::AnonConst),
    LifeTime(&'a ast::Lifetime),
    Type(&'a ast::Ty),
    Binding(&'a ast::AssocItemConstraint),
}
