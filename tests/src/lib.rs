pub fn failed_template(s: &str) -> String {
    format!("\x1b[31mFailed in '{s}' case\x1b[0m")
}