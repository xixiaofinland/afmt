use crate::child::Accessor;
use crate::context::FmtContext;
use crate::define_routing;
use crate::rewrite::Rewrite;
use crate::shape::Shape;
use crate::struct_and_enum::*;
use colored::Colorize;
#[allow(unused_imports)]
use log::debug;
use tree_sitter::Node;

#[allow(dead_code)]
pub trait Visitor<'tree> {
    fn visit(&self, context: &FmtContext, shape: &mut Shape) -> String;
    fn visit_standalone_children(&self, context: &FmtContext, shape: &Shape) -> String;
    fn visit_children_in_same_line(
        &self,
        delimiter: &str,
        context: &FmtContext,
        shape: &mut Shape,
    ) -> String;
    fn try_visit_cs_by_k(&self, kind: &str, context: &FmtContext, shape: &mut Shape)
        -> Vec<String>;
    fn try_visit_cs(&self, context: &FmtContext, shape: &mut Shape) -> Vec<String>;
}

impl<'tree> Visitor<'tree> for Node<'tree> {
    fn visit(&self, context: &FmtContext, shape: &mut Shape) -> String {
        if self.is_named() && self.grammar_name() == "operator" {
            return self.v(context.source_code).to_string();
        }

        let mut result = String::new();

        //println!("||| 1 routing");
        let route_name = "1 visitor";
        define_routing!(self, result, context, shape, route_name;
            "class_declaration" => ClassDeclaration,
            "method_declaration" => MethodDeclaration,
            "block" => Block,
            "local_variable_declaration" => LocalVariableDeclaration,
            "array_creation_expression" => ArrayCreationExpression,
            "array_initializer" => ArrayInitializer,
            "expression_statement" => Statement,
            "generic_type" => GenericType,
            "dml_type" => DmlType,
            "object_creation_expression" => ObjectCreationExpression,
            "instanceof_expression" => InstanceOfExpression,
            "annotation_argument_list" => AnnotationArgumentList,
            "for_statement" => ForStatement,
            "try_statement" => TryStatement,
            "line_comment" => LineComment,
            "method_invocation" => MethodInvocation,
            "scoped_type_identifier" => ScopedTypeIdentifier,
            "field_declaration" => FieldDeclaration,
            "unary_expression" => UnaryExpression,
            "dml_security_mode" => DmlSecurityMode,
            "map_creation_expression" => MapCreationExpression,
            "if_statement" => IfStatement,
            "binary_expression" => BinaryExpression,
            "run_as_statement" => RunAsStatement,
            "return_statement" => ReturnStatement,
            "dimensions_expr" => DimensionsExpr,
            "field_access" => FieldAccess,
            "array_access" => ArrayAccess,
            "do_statement" => DoStatement,
            "ternary_expression" => TernaryExpression,
            "string_literal" => Value,
            "boolean" => Value,
            "type_identifier" => Value,
            "identifier" => Value,
            "int" => Value
        );
        return result;

        let kind = NodeKind::from_kind(self.kind());
        match kind {
            NodeKind::MethodDeclaration => {
                let n = MethodDeclaration::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::FieldDeclaration => {
                let n = FieldDeclaration::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::WhileStatement => {
                let n = WhileStatement::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::Statement => {
                let n = Statement::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::EmptyNode => self.visit_standalone_children(context, shape),
            NodeKind::Block => {
                let n = Block::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::Interfaces => {
                let n = Interfaces::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::SuperClass => {
                let n = SuperClass::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::Expression => {
                let n = Expression::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::Value => {
                let n = Value::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::LocalVariableDeclaration => {
                let n = LocalVariableDeclaration::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::VariableDeclarator => {
                let n = VariableDeclarator::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::ArgumentList => {
                let n = ArgumentList::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::ArrayInitializer => {
                let n = ArrayInitializer::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::ForStatement => {
                let n = ForStatement::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::EnhancedForStatement => {
                let n = EnhancedForStatement::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::IfStatement => {
                let n = IfStatement::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::ParenthesizedExpression => {
                let n = ParenthesizedExpression::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::LineComment => {
                let n = LineComment::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::ReturnStatement => {
                let n = ReturnStatement::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::TypeArguments => {
                let n = TypeArguments::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::GenericType => {
                let n = GenericType::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::DimensionsExpr => {
                let n = DimensionsExpr::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::ArrayType => {
                let n = ArrayType::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::MapInitializer => {
                let n = MapInitializer::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::Annotation => {
                let n = Annotation::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::AnnotationArgumentList => {
                let n = AnnotationArgumentList::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::AnnotationKeyValue => {
                let n = AnnotationKeyValue::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::Modifiers => {
                let n = Modifiers::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::ConstructorDeclaration => {
                let n = ConstructorDeclaration::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::ConstructorBody => {
                let n = ConstructorBody::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::ExplicitConstructorInvocation => {
                let n = ExplicitConstructorInvocation::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::AssignmentExpression => {
                let n = AssignmentExpression::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::DmlExpression => {
                let n = DmlExpression::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::DmlType => {
                let n = DmlType::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::RunAsStatement => {
                let n = RunAsStatement::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::PrimaryExpression => {
                let n = PrimaryExpression::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::ArrayAccess => {
                let n = ArrayAccess::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::ScopedTypeIdentifier => {
                let n = ScopedTypeIdentifier::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::DmlSecurityMode => {
                let n = DmlSecurityMode::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::EnumDeclaration => {
                let n = EnumDeclaration::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::EnumConstant => {
                let n = EnumConstant::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::TryStatement => {
                let n = TryStatement::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::FieldAccess => {
                let n = FieldAccess::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::InstanceOfExpression => {
                let n = InstanceOfExpression::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::TernaryExpression => {
                let n = TernaryExpression::new(self);
                n.rewrite(context, shape)
            }
            NodeKind::QueryExpression => {
                let n = QueryExpression::new(self);
                n.rewrite(context, shape)
            }
            _ => {
                println!(
                    "{} {}",
                    "### Visitor: unknown self: ".yellow(),
                    self.kind().red()
                );
                panic!();
            }
        }
    }

    fn visit_standalone_children(&self, context: &FmtContext, shape: &Shape) -> String {
        let mut result = String::new();
        let shape = shape.copy_with_indent_block_plus(context.config);

        let mut cursor = self.walk();
        let children = self
            .named_children(&mut cursor)
            .map(|child| {
                let mut child_shape = shape.clone_with_stand_alone(true);
                child.visit(context, &mut child_shape)
            })
            .collect::<Vec<_>>()
            .join("\n");

        if !children.is_empty() {
            result.push_str(&children);
            result.push('\n');
        }
        result
    }

    fn visit_children_in_same_line(
        &self,
        delimiter: &str,
        context: &FmtContext,
        shape: &mut Shape,
    ) -> String {
        let mut result = String::new();
        let mut cursor = self.walk();
        let fields = self
            .named_children(&mut cursor)
            .map(|child| {
                let mut child_shape = shape.clone_with_stand_alone(false);
                child.visit(context, &mut child_shape)
            })
            .collect::<Vec<_>>()
            .join(delimiter);

        result.push_str(&fields);
        result
    }

    fn try_visit_cs(&self, context: &FmtContext, shape: &mut Shape) -> Vec<String> {
        let mut cursor = self.walk();
        self.named_children(&mut cursor)
            .map(|n| n.visit(context, shape))
            .collect::<Vec<_>>()
    }

    fn try_visit_cs_by_k(
        &self,
        kind: &str,
        context: &FmtContext,
        shape: &mut Shape,
    ) -> Vec<String> {
        self.try_cs_by_k(kind)
            .iter()
            .map(|n| n.visit(context, shape))
            .collect::<Vec<_>>()
    }
}
