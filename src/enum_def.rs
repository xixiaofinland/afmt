use crate::struct_def::*;

#[derive(Debug)]
pub enum ClassMember {
    Field(FieldDeclaration),
    //Method(MethodDeclaration<'a>),
    //NestedClass(ClassDeclaration<'a>),
    //Interface(InterfaceDeclaration<'a>),
    //Enum(EnumDeclaration<'a>),
    //Block(Block<'a>),
    //StaticInitializer(StaticInitializer<'a>),
    //Constructor(ConstructorDeclaration<'a>),
    //EmptyStatement, // Represents the ";" case
}

//_unannotated_type: ($) => choice($._simple_type, $.array_type),
#[derive(Debug)]
pub enum UnnanotatedType {
    Identifier(Identifier),
}

#[derive(Debug)]
pub enum VariableInitializer {
    Expression(Expression),
    ArrayInitializer(ArrayInitializer),
}

//expression: ($) =>
//  choice(
//    $.assignment_expression,
//    $.binary_expression,
//    $.instanceof_expression,
//    $.ternary_expression,
//    $.update_expression,
//    $.primary_expression,
//    $.unary_expression,
//    $.cast_expression,
//    $.dml_expression
//  ),

#[derive(Debug)]
pub enum Expression {
    Assignment(AssignmentExpression),
    Primary(PrimaryExpression),
}

#[derive(Debug)]
pub enum PrimaryExpression {
    Identifier(Identifier),
}
