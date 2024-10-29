//use crate::accessor::Accessor;
//use crate::rich_def::*;
//use crate::utility::*;
//
//impl<'t> ClassNode<'t> {
//    pub fn enrich_data(&mut self, shape: &mut EShape, context: &EContext) {
//        self.content = self.rewrite(shape, context);
//        let offset = get_length_before_brace(&self.content);
//
//        self.format_info = FormatInfo {
//            offset,
//            wrappable: true,
//            indent_level: shape.indent_level,
//            force_break_after: true,
//            has_new_line_before: false,
//        };
//    }
//
//    pub fn rewrite(&mut self, shape: &mut EShape, context: &EContext) -> String {
//        let (node, mut result, source_code, config, children) = self.prepare(context);
//
//        if let Some(c) = node.try_c_by_k("modifiers") {
//            let modifiers = Modifiers::build(c, shape, context);
//            result.push_str(&modifiers.content);
//            children.push(ASTNode::Modifiers(modifiers));
//        }
//
//        result.push_str("class ");
//        result.push_str(node.cv_by_n("name", source_code));
//
//        //if let Some(ref c) = node.try_c_by_n("type_parameters") {
//        //    //result.push_str(&rewrite_shape::<TypeParameters>(c, shape, false, context));
//        //}
//
//        //if let Some(ref c) = node.try_c_by_n("superclass") {
//        //    //result.push_str(&rewrite_shape::<SuperClass>(c, shape, false, context));
//        //}
//
//        if let Some(ref c) = node.try_c_by_n("interfaces") {
//            //result.push_str(&rewrite_shape::<Interfaces>(c, shape, false, context));
//        }
//
//        result.push_str(" {\n");
//
//        let body_node = node.c_by_n("body");
//        //result.push_str(&body_node.apply_to_standalone_children(
//        //    shape,
//        //    context,
//        //    |c, c_shape, c_context| c._visit(c_shape, c_context),
//        //));
//
//        result
//    }
//}
//
//impl<'t> Modifiers<'t> {
//    pub fn enrich_data(&mut self, shape: &mut EShape, context: &EContext) {
//        self.content = self.rewrite(shape, context);
//        let offset = get_length_before_brace(&self.content);
//
//        self.format_info = FormatInfo {
//            offset,
//            wrappable: false,
//            indent_level: shape.indent_level,
//            //force_break_before: false,
//            force_break_after: false,
//            has_new_line_before: false,
//        };
//    }
//
//    pub fn rewrite(&mut self, shape: &mut EShape, context: &EContext) -> String {
//        let (node, mut result, source_code, config, children) = self.prepare(context);
//        result
//    }
//}
