#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![warn(clippy::allow_attributes)] // add once lint_reasons is stable
#![warn(clippy::allow_attributes_without_reason)] // add once lint_reasons is stable
#![warn(clippy::arithmetic_side_effects)] // potentially add in the future
#![warn(clippy::as_underscore)]
#![warn(clippy::assertions_on_result_states)]
#![warn(clippy::branches_sharing_code)]
#![warn(clippy::cargo_common_metadata)]
#![warn(clippy::clear_with_drain)]
#![warn(clippy::clone_on_ref_ptr)]
// #![warn(clippy::cognitive_complexity)] // later
#![warn(clippy::collection_is_never_read)]
#![warn(clippy::create_dir)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::debug_assert_with_mut_call)]
#![warn(clippy::decimal_literal_representation)]
#![warn(clippy::default_union_representation)]
#![warn(clippy::deref_by_slicing)]
#![warn(clippy::derive_partial_eq_without_eq)]
#![warn(clippy::empty_drop)]
#![warn(clippy::empty_line_after_doc_comments)]
#![warn(clippy::empty_line_after_outer_attr)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::equatable_if_let)]
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::filetype_is_file)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::fn_to_numeric_cast_any)]
#![warn(clippy::format_push_string)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::impl_trait_in_params)]
#![warn(clippy::implied_bounds_in_impls)] // supposed to exist but doesn't
#![warn(clippy::imprecise_flops)]
#![warn(clippy::iter_on_empty_collections)]
#![warn(clippy::iter_on_single_items)]
#![warn(clippy::iter_with_drain)]
#![warn(clippy::large_stack_frames)]
#![warn(clippy::let_underscore_untyped)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::manual_clamp)]
#![warn(clippy::mem_forget)]
#![warn(clippy::min_ident_chars)]
#![warn(clippy::missing_asserts_for_indexing)] // supposed to exist but doesn't
#![warn(clippy::mixed_read_write_in_expression)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::needless_collect)]
#![warn(clippy::needless_pass_by_ref_mut)]
#![warn(clippy::negative_feature_names)]
#![warn(clippy::nonstandard_macro_braces)]
#![warn(clippy::or_fun_call)]
#![warn(clippy::path_buf_push_overwrite)]
#![warn(clippy::pub_without_shorthand)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::readonly_write_lock)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::redundant_clone)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::same_name_method)]
#![warn(clippy::self_named_module_files)]
#![warn(clippy::semicolon_inside_block)]
#![warn(clippy::significant_drop_in_scrutinee)]
#![warn(clippy::significant_drop_tightening)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_lit_chars_any)]
#![warn(clippy::string_to_string)]
#![warn(clippy::suboptimal_flops)]
#![warn(clippy::suspicious_operation_groupings)]
#![warn(clippy::suspicious_xor_used_as_pow)]
#![warn(clippy::tests_outside_test_module)]
#![warn(clippy::trait_duplication_in_bounds)]
#![warn(clippy::trivial_regex)]
#![warn(clippy::try_err)]
#![warn(clippy::type_repetition_in_bounds)]
#![warn(clippy::unnecessary_struct_initialization)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::unused_peekable)]
#![warn(clippy::unused_rounding)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::useless_let_if_seq)]
#![warn(clippy::verbose_file_reads)]
#![warn(clippy::wildcard_dependencies)]
#![warn(absolute_paths_not_starting_with_crate)]
#![warn(keyword_idents)]
#![warn(let_underscore_drop)]
#![warn(macro_use_extern_crate)]
#![warn(meta_variable_misuse)]
#![warn(missing_abi)]
// #![warn(must_not_suspend)] // add once stable
#![warn(pointer_structural_match)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_tuple_struct_fields)]
#![allow(clippy::doc_markdown)] // false positives on any kind of camel case-looking words
#![allow(clippy::enum_glob_use)]
#![allow(clippy::too_many_lines)] // warn later with cognitive_complexity
#![allow(uncommon_codepoints)]

use clap::{Parser, Subcommand};

pub mod client;
pub mod common;
pub mod datagram;
pub mod message;
pub mod server;
pub mod types;

// TODO: automatically generate these
pub const COMPILE_DATE: &str = "Nov 16 2023";
pub const COMPILE_TIME: &str = "21:24:18";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Client(client::Parameter),
    Server(server::Parameter),
}

#[allow(clippy::missing_errors_doc)]
pub fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Client(parameter) => {
            client::main::interactive(parameter)?;
        }
        Commands::Server(parameter) => {
            server::main::serve(parameter)?;
        }
    }

    Ok(())
}
