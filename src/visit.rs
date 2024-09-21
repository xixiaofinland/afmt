use crate::child::Accessor;
use crate::context::FmtContext;
use crate::define_routing;
use crate::rewrite::Rewrite;
use crate::shape::Shape;
use crate::struct_def::*;
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

        define_routing!(self, result, context, shape;
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
            "enum_declaration" => EnumDeclaration,
            "enhanced_for_statement" => EnhancedForStatement,
            "assignment_expression" => AssignmentExpression,
            "if_statement" => IfStatement,
            "constructor_declaration" => ConstructorDeclaration,
            "explicit_constructor_invocation" => ExplicitConstructorInvocation,
            "while_statement" => WhileStatement,
            "binary_expression" => BinaryExpression,
            "run_as_statement" => RunAsStatement,
            "return_statement" => ReturnStatement,
            "dimensions_expr" => DimensionsExpr,
            "field_access" => FieldAccess,
            "array_access" => ArrayAccess,
            "array_type" => ArrayType,
            "do_statement" => DoStatement,
            "ternary_expression" => TernaryExpression,
            "string_literal" => Value,
            "boolean" => Value,
            "type_identifier" => Value,
            "identifier" => Value,
            "int" => Value
        );
        return result;
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
