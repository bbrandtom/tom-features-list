use std::fs;

use cairo_lang_sierra::ProgramParser;
use cairo_lang_sierra_ap_change::calc_ap_changes;
use cairo_lang_sierra_gas::calc_gas_info;
use cairo_lang_sierra_to_casm::metadata::Metadata;
use cairo_lang_utils::logging::init_logging;
use clap::Parser;

/// Command line args parser.
/// Exits with 0/1 if the input is formatted correctly/incorrectly.
#[derive(Parser, Debug)]
#[clap(version, verbatim_doc_comment)]
struct Args {
    /// The file to compile
    file: String,
    output: String,
}

fn main() {
    init_logging(log::LevelFilter::Off);
    log::info!("Starting Sierra compilation.");

    let args = Args::parse();

    let sierra_code = fs::read_to_string(args.file).expect("Could not read file!");
    let program = ProgramParser::new().parse(&sierra_code).unwrap();

    let gas_info = calc_gas_info(&program).expect("Failed calculating gas variables.");

    let gas_usage_check = true;
    let cairo_program = cairo_lang_sierra_to_casm::compiler::compile(
        &program,
        &Metadata {
            ap_change_info: calc_ap_changes(&program).expect("Failed calculating ap changes."),
            gas_info,
        },
        gas_usage_check,
    )
    .expect("Compilation failed.");

    fs::write(args.output, format!("{}", cairo_program)).expect("Failed to write output.");
}
