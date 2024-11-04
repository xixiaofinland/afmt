use crate::{data_model::*, doc::DocRef, doc_builder::DocBuilder};

#[derive(Debug)]
pub enum ClassMember {
    Field(Box<FieldDeclaration>),
    NestedClass(Box<ClassDeclaration>),
    //Method(MethodDeclaration<'a>),
    //Interface(InterfaceDeclaration<'a>),
    //Enum(EnumDeclaration<'a>),
    //Block(Block<'a>),
    //StaticInitializer(StaticInitializer<'a>),
    //Constructor(ConstructorDeclaration<'a>),
    //EmptyStatement, // Represents the ";" case
}

impl<'a> DocBuild<'a> for ClassMember {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            ClassMember::Field(field_decl) => {
                result.push(field_decl.build(b));
            }
            ClassMember::NestedClass(class_decl) => {
                result.push(class_decl.build(b));
            }
        }
    }
}

//_unannotated_type: ($) => choice($._simple_type, $.array_type),
#[derive(Debug)]
pub enum UnnanotatedType {
    Identifier(Identifier),
}

impl<'a> DocBuild<'a> for UnnanotatedType {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            UnnanotatedType::Identifier(i) => {
                result.push(b.txt(&i.value));
            }
        }
    }
}

#[derive(Debug)]
pub enum VariableInitializer {
    Expression(Expression),
    //ArrayInitializer(ArrayInitializer),
}

impl<'a> DocBuild<'a> for VariableInitializer {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            VariableInitializer::Expression(exp) => {
                result.push(exp.build(b));
            }
        }
    }
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
    //Assignment(AssignmentExpression),
    Primary(PrimaryExpression),
}

impl<'a> DocBuild<'a> for Expression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            Expression::Primary(p) => {
                result.push(p.build(b));
            }
        }
    }
}

#[derive(Debug)]
pub enum PrimaryExpression {
    Identifier(Identifier),
}

impl<'a> DocBuild<'a> for PrimaryExpression {
    fn build_inner(&self, b: &'a DocBuilder<'a>, result: &mut Vec<DocRef<'a>>) {
        match self {
            PrimaryExpression::Identifier(i) => {
                result.push(b.txt(&i.value));
            }
        }
    }
}

#[derive(Debug)]
pub enum ModifierType {
    Identifier(Identifier),
}
