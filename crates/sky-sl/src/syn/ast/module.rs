use super::{
    AstChildren, AstNode, FunctionDefinition, LayoutDefinition, ModuleDeclaration,
    StructDefinition, UseDeclaration,
};
use crate::syn::cst::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ModuleItem {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ModuleItem {
    fn can_cast_from(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::Fn
            | SyntaxKind::Struct
            | SyntaxKind::ModuleDeclaration
            | SyntaxKind::UseDeclaration
            | SyntaxKind::Layout => true,
            _ => false,
        }
    }

    fn cast_from(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        Self::can_cast_from(syntax.kind()).then(|| Self { syntax })
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}

impl ModuleItem {
    pub fn kind(&self) -> ModuleItemKind {
        match self.syntax.kind() {
            SyntaxKind::Fn => ModuleItemKind::FunctionDefinition(
                FunctionDefinition::cast_from(self.syntax.clone()).unwrap(),
            ),
            SyntaxKind::Struct => ModuleItemKind::StructDefinition(
                StructDefinition::cast_from(self.syntax.clone()).unwrap(),
            ),
            SyntaxKind::ModuleDeclaration => ModuleItemKind::ModuleDeclaration(
                ModuleDeclaration::cast_from(self.syntax.clone()).unwrap(),
            ),
            SyntaxKind::UseDeclaration => ModuleItemKind::UseDeclaration(
                UseDeclaration::cast_from(self.syntax.clone()).unwrap(),
            ),
            SyntaxKind::Layout => ModuleItemKind::LayoutDefinition(
                LayoutDefinition::cast_from(self.syntax.clone()).unwrap(),
            ),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ModuleItemKind {
    ModuleDeclaration(ModuleDeclaration),
    UseDeclaration(UseDeclaration),
    FunctionDefinition(FunctionDefinition),
    StructDefinition(StructDefinition),
    LayoutDefinition(LayoutDefinition),
}

pub trait ModuleItemOwner: AstNode {
    fn module_items(&self) -> AstChildren<ModuleItem> {
        super::children(self)
    }
}
