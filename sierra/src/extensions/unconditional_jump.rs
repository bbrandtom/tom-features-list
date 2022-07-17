use crate::{extensions::*, utils::gas_type};

struct UnconditionalJumpExtension {}

impl ExtensionImplementation for UnconditionalJumpExtension {
    fn get_signature(
        self: &Self,
        tmpl_args: &Vec<TemplateArg>,
    ) -> Result<ExtensionSignature, Error> {
        if !tmpl_args.is_empty() {
            return Err(Error::WrongNumberOfTypeArgs);
        }
        Ok(ExtensionSignature {
            args: vec![gas_type(1)],
            results: vec![vec![]],
            fallthrough: None,
        })
    }

    fn mem_change(
        self: &Self,
        _tmpl_args: &Vec<TemplateArg>,
        _registry: &TypeRegistry,
        context: Context,
        _arg_refs: Vec<RefValue>,
    ) -> Result<Vec<(Context, Vec<RefValue>)>, Error> {
        Ok(vec![(context, vec![])])
    }
}

pub(super) fn extensions() -> [(String, ExtensionBox); 1] {
    [("jump".to_string(), Box::new(UnconditionalJumpExtension {}))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::val_arg;

    #[test]
    fn legal_usage() {
        assert_eq!(
            UnconditionalJumpExtension {}.get_signature(&vec![]),
            Ok(ExtensionSignature {
                args: vec![gas_type(1)],
                results: vec![vec![]],
                fallthrough: None,
            })
        );
    }

    #[test]
    fn wrong_num_of_args() {
        assert_eq!(
            UnconditionalJumpExtension {}.get_signature(&vec![val_arg(1)]),
            Err(Error::WrongNumberOfTypeArgs)
        );
    }
}
