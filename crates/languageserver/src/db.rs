use db_utils::Upcast;
use defs::db::{DefsDatabase, DefsGroup};
use filesystem::db::{init_files_group, AsFilesGroupMut, FilesDatabase, FilesGroup};
use lowering::db::{LoweringDatabase, LoweringGroup};
use parser::db::ParserDatabase;
use semantic::db::{SemanticDatabase, SemanticGroup};
use syntax::node::db::{SyntaxDatabase, SyntaxGroup};

#[salsa::database(
    LoweringDatabase,
    SemanticDatabase,
    DefsDatabase,
    ParserDatabase,
    SyntaxDatabase,
    FilesDatabase
)]
pub struct RootDatabase {
    storage: salsa::Storage<RootDatabase>,
}
impl salsa::Database for RootDatabase {}
impl Default for RootDatabase {
    fn default() -> Self {
        let mut res = Self { storage: Default::default() };
        init_files_group(&mut res);
        res
    }
}
impl AsFilesGroupMut for RootDatabase {
    fn as_files_group_mut(&mut self) -> &mut (dyn FilesGroup + 'static) {
        self
    }
}
impl Upcast<dyn FilesGroup> for RootDatabase {
    fn upcast(&self) -> &(dyn FilesGroup + 'static) {
        self
    }
}
impl Upcast<dyn SyntaxGroup> for RootDatabase {
    fn upcast(&self) -> &(dyn SyntaxGroup + 'static) {
        self
    }
}
impl Upcast<dyn DefsGroup> for RootDatabase {
    fn upcast(&self) -> &(dyn DefsGroup + 'static) {
        self
    }
}
impl Upcast<dyn SemanticGroup> for RootDatabase {
    fn upcast(&self) -> &(dyn SemanticGroup + 'static) {
        self
    }
}
impl Upcast<dyn LoweringGroup> for RootDatabase {
    fn upcast(&self) -> &(dyn LoweringGroup + 'static) {
        self
    }
}