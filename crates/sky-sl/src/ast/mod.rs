#[derive(Debug)]
pub enum ItemKind {
    Function(FunctionKind),
    Struct(StructKind),
}

#[derive(Debug)]
pub struct FunctionKind {
    pub identifier: Identifier,
    // signature
    // generics
    // block
}

#[derive(Debug)]
pub struct StructKind {
    pub identifier: Identifier,
    // generics
    // fields
}

#[derive(Debug)]
pub struct Identifier {

}
