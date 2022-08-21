use rustc_macros::SessionDiagnostic;
use rustc_span::Span;

#[derive(SessionDiagnostic)]
#[error(trait_selection::dump_vtable_entries)]
pub struct DumpVTableEntries {
    #[primary_span]
    pub span: Span,
    pub trait_ref: String,
    pub entries: String,
}

#[derive(SessionDiagnostic)]
#[error(trait_selection::unable_to_construct_constant_value)]
pub struct UnableToConstructConstantValue {
    #[primary_span]
    pub span: Span,
    pub unevaluated: String,
}

#[derive(SessionDiagnostic)]
#[help(trait_selection::auto_deref_reached_recursion_limit_help)]
#[error(trait_selection::auto_deref_reached_recursion_limit, code = "E0055")]
pub struct AutoDerefReachedRecursionLimit {
    #[primary_span]
    #[label(trait_selection::auto_deref_reached_recursion_limit_label)]
    pub span: Span,
    pub ty: String,
    pub suggested_limit: String,
    pub crate_name: String,
}

#[derive(SessionDiagnostic)]
#[error(trait_selection::empty_on_clause_in_rustc_on_unimplemented, code = "E0232")]
pub struct EmptyOnClauseInOnUnimplemented {
    #[primary_span]
    #[label(trait_selection::empty_on_clause_in_rustc_on_unimplemented_label)]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[error(trait_selection::invalid_on_clause_in_rustc_on_unimplemented, code = "E0232")]
pub struct InvalidOnClauseInOnUnimplemented {
    #[primary_span]
    #[label(trait_selection::invalid_on_clause_in_rustc_on_unimplemented_label)]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[error(trait_selection::no_value_in_rustc_on_unimplemented, code = "E0232")]
pub struct NoValueInOnUnimplemented {
    #[primary_span]
    #[label(trait_selection::no_value_in_rustc_on_unimplemented_label)]
    #[note(trait_selection::no_value_in_rustc_on_unimplemented_note)]
    pub span: Span,
}
