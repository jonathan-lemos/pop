use crate::operations::showdown::print_showdown_help;

pub fn format_separated_values<
    T,
    I: Iterator<Item = T>,
    F: FnMut(T, &mut std::fmt::Formatter<'_>) -> std::fmt::Result,
>(
    values: I,
    separator: &str,
    fmt: &mut std::fmt::Formatter<'_>,
    mut f: F,
) -> std::fmt::Result {
    let mut first = true;
    for value in values {
        if !first {
            fmt.write_str(separator)?;
        } else {
            first = false;
        }
        f(value, fmt)?;
    }
    Ok(())
}

pub fn print_basic_help(executable_name: &str) {
    println!("{}: Poker Odds Program", executable_name);
    println!("Usage: {} <operation> [...]", executable_name);
    println!(
        "\t{} showdown <card><card> [vs <card><card>]* [on <card>+]",
        executable_name
    );
    println!();
    println!(
        "Use `{} <operation> --help` for detailed help with an operation",
        executable_name
    )
}

pub fn print_unrecognized_operation(executable_name: &str, operation: &str) {
    println!("Unrecognized operation '{}'", operation);
    print_help(executable_name, None);
}

pub fn print_help(executable_name: &str, operation: Option<&str>) {
    match operation {
        None => print_basic_help(executable_name),
        Some("--help") => print_basic_help(executable_name),
        Some("showdown") => print_showdown_help(executable_name),
        Some(op) => print_unrecognized_operation(executable_name, &op),
    }
}
