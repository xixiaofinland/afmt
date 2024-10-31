use crate::struct_def::FieldDeclaration;

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
