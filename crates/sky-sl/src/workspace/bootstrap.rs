use super::workspace::*;
use camino::Utf8Path;
use crate::{db::*, syn::ast::{IdentifierOwner, ModuleDeclarationOwner}};
use std::sync::Arc;

pub fn bootstrap(path: &Utf8Path) -> Result<Workspace, WorkspaceError> {
    let root_path = path.parent().unwrap().to_owned();
    let mut workspace = Workspace::load_from_file(path)?;

    for project in workspace.get_projects() {
        let source_root = root_path.join(&project.path);
        let source = std::fs::read_to_string(&source_root).unwrap();

        let module_path = workspace.db().intern_path_data(ModulePathData::Package(project.name.to_owned()));
        workspace.db_mut().set_input_file(source_root.clone(), Arc::new(source));

        workspace.db_mut().set_module_file_path(module_path, source_root.clone());

        add_modules(&mut workspace, &project.name, &source_root, module_path);
    }

    Ok(workspace)
}

fn add_modules(workspace: &mut Workspace, module_name: &str, path: &Utf8Path, parent_module: ModulePath) {

    // dbg!(parent_module, module_name);

    let ast = workspace.get_ast(path);
    let module_root = path.parent().unwrap().clone();

    for module in ast.tree().modules() {
        if let Some(identifier) = module.identifier() {
            let name = identifier.syntax.to_string();

            // name.skysl
            let file_source_path = module_root.join(format!("{}.skysl", name));
            // name/mod.skysl
            let module_folder_source_path = module_root.join(&name).join("mod.skysl");

            let module_path = workspace.db().intern_path_data(ModulePathData::Module(parent_module, name.to_owned()));

            match (file_source_path.exists(), module_folder_source_path.exists()) {
                (true, true) => {
                    // ERROR both file and folder exist
                    unimplemented!();
                },
                (true, false) => {
                    // file exists
                    let source = std::fs::read_to_string(&file_source_path).expect("expected file to exist");
                    workspace.db_mut().set_input_file(file_source_path.clone(), Arc::new(source));
                    workspace.db_mut().set_module_file_path(module_path, file_source_path.clone());
                    add_modules(workspace, &name, &file_source_path, module_path);
                },
                (false, true) => {
                    // folder exists
                    let source = std::fs::read_to_string(&module_folder_source_path).expect("expected file to exist");
                    workspace.db_mut().set_input_file(module_folder_source_path.clone(), Arc::new(source));
                    workspace.db_mut().set_module_file_path(module_path, module_folder_source_path.clone());
                    add_modules(workspace, &name, &module_folder_source_path, module_path);
                },
                (false, false) => {
                    // ERROR neither file or folder exist
                    unimplemented!();
                }
            }

        }
    }

    // iterate over ast modules
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use camino::Utf8PathBuf;

    use super::*;

    #[test]
    fn it_works() {
        let path = Utf8PathBuf::from_str("C:\\Workspace\\Projects\\sky-sl\\example\\skysl.toml").unwrap();
        let workspace = bootstrap(&path).unwrap();
        dbg!(workspace);
        // panic!();
    }
}
