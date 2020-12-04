#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    arithmetic_overflow,
    array_into_iter,
    asm_sub_register,
    bindings_with_variant_name,
    broken_intra_doc_links,
    cenum_impl_drop_cast,
    clashing_extern_declarations,
    coherence_leak_check,
    conflicting_repr_hints,
    confusable_idents,
    const_err,
    const_evaluatable_unchecked,
    const_item_mutation,
    dead_code,
    deprecated,
    deprecated_in_future,
    drop_bounds,
    ellipsis_inclusive_range_patterns,
    explicit_outlives_requirements,
    exported_private_dependencies,
    function_item_references,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    incomplete_include,
    indirect_structural_match,
    inline_no_sanitize,
    invalid_codeblock_attributes,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_docs,
    mixed_script_confusables,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    nontrivial_structural_match,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_patterns,
    path_statements,
    patterns_in_fns_without_body,
    pointer_structural_match,
    private_doc_tests,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolons,
    safe_packed_borrows,
    soft_unstable,
    stable_features,
    temporary_cstring_as_ptr,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unaligned_references,
    uncommon_codepoints,
    unconditional_panic,
    unconditional_recursion,
    uninhabited_static,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_crate_dependencies,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    invalid_html_tags,
    missing_doc_code_examples,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]

//! # duckscript_cli
//!
//! The duckscript command line executable.
//!
//! This executable enables to run the duckscript runner with the default sdk.
//!
//! # Installation
//! See [main duckscript documentation](https://github.com/sagiegurari/duckscript)
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/duckscript/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/duckscript/blob/master/LICENSE) open source license.
//!

mod linter;

use duckscript::runner;
use duckscript::types::error::ScriptError;
use duckscript::types::runtime::Context;
use duckscriptsdk;
use std::env;
use std::process::exit;

static VERSION: &str = env!("CARGO_PKG_VERSION");
static AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
static DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    match run_cli() {
        Err(error) => {
            println!("Error: {}", error);
            exit(1);
        }
        _ => (),
    };
}

fn run_cli() -> Result<(), ScriptError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        run_repl()
    } else if args[1] == "--version" {
        println!(
            "Duckscript Runtime: {}\nDuckscript SDK: {}\nDuckscript CLI: {}",
            duckscript::version(),
            duckscriptsdk::version(),
            VERSION
        );

        Ok(())
    } else if args[1] == "--help" || args[1] == "-h" {
        let usage = include_str!("help.txt");
        println!(
            "duckscript {}\n{}\n{}\n\n{}",
            VERSION, AUTHOR, DESCRIPTION, usage
        );

        Ok(())
    } else {
        let (value, is_file, run) = if args.len() == 2 {
            (args[1].clone(), true, true)
        } else {
            if args[1] == "-e" || args[1] == "--eval" {
                (args[2].clone(), false, true)
            } else if args[1] == "-l" || args[1] == "--lint" {
                (args[2].clone(), true, false)
            } else {
                (args[1].clone(), true, true)
            }
        };

        if run {
            run_script(&value, is_file)
        } else {
            linter::lint_file(&value)
        }
    }
}

fn create_context() -> Result<Context, ScriptError> {
    let mut context = Context::new();
    duckscriptsdk::load(&mut context.commands)?;

    Ok(context)
}

fn run_script(value: &str, is_file: bool) -> Result<(), ScriptError> {
    let context = create_context()?;

    if is_file {
        runner::run_script_file(value, context)?;
    } else {
        runner::run_script(value, context)?;
    }

    Ok(())
}

fn run_repl() -> Result<(), ScriptError> {
    let context = create_context()?;

    runner::repl(context)?;

    Ok(())
}
