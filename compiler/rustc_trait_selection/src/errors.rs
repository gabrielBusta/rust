use rustc_span::Span;
use rustc_macros::SessionDiagnostic;

#[derive(SessionDiagnostic)]
#[error(trait_selection::dump_vtable_entries)]
pub struct DumpVTableEntries {
    #[primary_span]
    pub span: Span,
    pub trait_ref: String,
    pub entries: String,
}

#[derive(SessionDiagnostic)]
#[error(trait_selection::unable_to_construct_constant_value_unevaluated_constant)]
pub struct UnableToConstructConstantValueUnevaluatedConstant {
    #[primary_span]
    pub span: Span,
    pub unevaluated: String,
}

#[derive(SessionDiagnostic)]
#[help(trait_selection::deref_recursion_limit_reached_help)]
#[error(trait_selection::reached_recursion_limit_auto_deref, code = "E0055")]
pub struct ReachedRecursionLimitDeref {
    #[primary_span]
    #[label(trait_selection::deref_recursion_limit_reached)]
    pub span: Span,
    pub ty: String,
    pub suggested_limit: String,
    pub crate_name: String,
}
