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

pub fn as_percentage(numerator: usize, denominator: usize) -> String {
    if denominator != 0 {
        format!("{:.2}%", numerator as f64 / denominator as f64)
    } else {
        "NaN%".to_string()
    }
}
