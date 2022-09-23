use defs::ids::{GenericParamId, GenericParamLongId, ModuleId};
use syntax::node::{ast, TypedSyntaxNode};

use crate::db::SemanticGroup;
use crate::diagnostic::SemanticDiagnostics;

/// Returns the parameters of the given function signature's AST.
pub fn semantic_generic_params(
    db: &dyn SemanticGroup,
    _diagnostics: &mut SemanticDiagnostics,
    module_id: ModuleId,
    generic_args: &ast::OptionGenericParams,
) -> Vec<GenericParamId> {
    let syntax_db = db.upcast();

    match generic_args {
        syntax::node::ast::OptionGenericParams::Empty(_) => vec![],
        syntax::node::ast::OptionGenericParams::Some(syntax) => syntax
            .generic_params(syntax_db)
            .elements(syntax_db)
            .iter()
            .map(|param_syntax| {
                db.intern_generic_param(GenericParamLongId(module_id, param_syntax.stable_ptr()))
            })
            .collect(),
    }
}