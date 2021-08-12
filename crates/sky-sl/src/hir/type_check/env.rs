use crate::hir::{typed, untyped};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ty(u32);

#[derive(Debug)]
struct Scope {
    entries: HashMap<String, Ty>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &str, ty: Ty) {
        self.entries.insert(name.to_string(), ty);
    }

    pub fn lookup(&self, name: &str) -> Option<Ty> {
        self.entries.get(name).cloned()
    }
}

#[derive(Debug)]
pub struct Env {
    scopes: Vec<Scope>,
    ty: HashMap<String, Ty>,
    ty_i: u32,
}

impl Env {
    pub fn empty() -> Self {
        Self {
            scopes: vec![Scope::new()],
            ty: HashMap::new(),
            ty_i: 0,
        }
    }

    pub fn intern_ty(&mut self, ty_name: &str) -> Ty {
        if let Some(ty) = self.ty.get(ty_name) {
            *ty
        } else {
            let ty = Ty(self.ty_i);
            self.ty_i += 1;
            self.ty.insert(ty_name.to_string(), ty);
            ty
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert(&mut self, name: &str, ty: Ty) {
        self.scopes.last_mut().unwrap().insert(name, ty);
    }

    pub fn lookup(&self, name: &str) -> Option<Ty> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.lookup(name) {
                return Some(ty);
            }
        }

        None
    }
}

pub fn infer_module(module: &untyped::Module, env: &mut Env) -> typed::Module {
    let mut items = Vec::new();
    let mut errors = Vec::new();

    for item in &module.items {
        match item {
            untyped::ItemKind::Function(function) => {
                env.push_scope();
                let function = infer_function(function, env);
                items.push(typed::ItemKind::Function(function));
                env.pop_scope();
            },
            _ => {},
        }
    }

    typed::Module::new(items, errors, module.span)
}

pub fn infer_function(function: &untyped::FunctionKind, env: &mut Env) -> typed::FunctionKind {
    let ty = env.intern_ty(&function.signature.name);
    
    let mut arguments = Vec::new();
    // add arguments to env
    for argument in &function.signature.arguments {
        let ty = env.intern_ty(&argument.ty_name);
        env.insert(&argument.name, ty);
        arguments.push(typed::FunctionArgument::new(
            argument.name.to_string(),
            ty,
            argument.span,
        ));
    }

    let return_type = if let Some(return_type) = &function.signature.return_type {
        env.intern_ty(return_type)
    } else {
        env.intern_ty("()")
    };

    let signature = typed::FunctionSignature::new(
        function.signature.name.to_string(),
        arguments,
        return_type,
        function.signature.span,
    );

    // infer block
    // TODO block return type
    let block = infer_block(&function.block, env);

    typed::FunctionKind::new(signature, block, ty, function.span)
}

pub fn infer_block(block: &untyped::Block, env: &mut Env) -> typed::Block {
    let mut statements = Vec::new();
    for statement in &block.statements {
        let statement = infer_statement(statement, env);
        statements.push(statement);
    }

    typed::Block::new(statements, block.span)
}

pub fn infer_statement(statement: &untyped::StatementKind, env: &mut Env) -> typed::StatementKind {
    match statement {
        untyped::StatementKind::Let(let_statement) => {
            env.push_scope();
            // infer expression
            let expr = infer_expression(&let_statement.expression, env);
            let ty = expr.ty();
            env.pop_scope();

            // add local variable to the env
            env.insert(&let_statement.name, ty);

            typed::StatementKind::Let(typed::LetStatement::new(
                let_statement.name.to_string(),
                expr,
                ty,
                let_statement.span,
            ))
        },
        untyped::StatementKind::Expression(expression_statement) => {
            // infer expression
            env.push_scope();
            let expr = infer_expression(&expression_statement.expression, env);
            let ty = expr.ty();
            env.pop_scope();

            typed::StatementKind::Expression(typed::ExpressionStatement::new(
                expr,
                ty,
                expression_statement.span,
            ))
        },
    }
}

pub fn infer_expression(expression: &untyped::ExpressionKind, env: &mut Env) -> typed::ExpressionKind {
    match expression {
        untyped::ExpressionKind::LiteralExpression(literal_expression) => {
            // primitive type
            // TODO float vs int vs boolean literals
            let ty = env.intern_ty("int");
            typed::ExpressionKind::LiteralExpression(typed::LiteralExpression::new(ty, literal_expression.span))
        },
        untyped::ExpressionKind::BinaryExpression(binary_expression) => {
            // infer left
            let lhs = infer_expression(&binary_expression.lhs, env);
            // infer right
            let rhs = infer_expression(&binary_expression.rhs, env);

            // TODO proper inference of operations
            if lhs.ty() == rhs.ty() {
                let ty = lhs.ty();
                typed::ExpressionKind::BinaryExpression(typed::BinaryExpression::new(
                    Box::new(lhs),
                    Box::new(rhs),
                    ty,
                    binary_expression.span,
                ))
            } else {
                unimplemented!()
            }
        },
        untyped::ExpressionKind::GroupExpression(group_expression) => {
            infer_expression(&group_expression.inner, env)
        },
        untyped::ExpressionKind::CallExpression(call_expression) => {
            // infer arguments
            let mut arguments = Vec::new();
            for argument in &call_expression.arguments {
                let expr = infer_expression(argument, env);
                arguments.push(expr);
            }

            // fn return type
            todo!()
        },
        untyped::ExpressionKind::FieldAccessExpression(_) => {
            // member type
            todo!()
        },
        untyped::ExpressionKind::IndexExpression(_) => {
            // indexee type
            todo!()
        },
        untyped::ExpressionKind::PathExpression(path_expression) => {
            // variables
            if let Some(ty) = env.lookup(&path_expression.path) {
                typed::ExpressionKind::PathExpression(typed::PathExpression::new(
                    path_expression.path.to_string(),
                    ty,
                    path_expression.span
                ))
            } else {
                // not found
                todo!()
            }
        },
        untyped::ExpressionKind::StructExpression(_) => {
            // struct type
            todo!()
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::db::*;
    use camino::Utf8PathBuf;
    use std::str::FromStr;
    use std::sync::Arc;

    #[test]
    fn it_works() {
        let mut db = CompilerDatabase::default();

        let path = Utf8PathBuf::from_str("/foo/bar").unwrap();
        let input = "fn foo() { let a = 1.0 + 2.0; let b = 3.0; let c = a + b; }".to_string();

        db.set_input_file(path.clone(), Arc::from(input));
        
        // let typed = db.types(path);
        // dbg!(typed);

        dbg!(db.type_at(path.clone(), 0, 0));

        // panic!();
    }
}
