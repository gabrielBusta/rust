dump_vtable_entries = vtable entries for `{$trait_ref}`: {$entries}

unable_to_construct_constant_value = unable to construct a constant value for the unevaluated constant {$unevaluated}

auto_deref_reached_recursion_limit = reached the recursion limit while auto-dereferencing `{$ty}`
    .label = deref recursion limit reached
    .help = consider increasing the recursion limit by adding a `#![recursion_limit = "{$suggested_limit}"]` attribute to your crate (`{$crate_name}`)

empty_on_clause_in_rustc_on_unimplemented = empty `on`-clause in `#[rustc_on_unimplemented]`
    .label = empty on-clause here

invalid_on_clause_in_rustc_on_unimplemented = invalid `on`-clause in `#[rustc_on_unimplemented]`
    .label = invalid on-clause here

no_value_in_rustc_on_unimplemented = this attribute must have a valid value
    .label = expected value here
    .note = "eg `#[rustc_on_unimplemented(message="foo")]`"
