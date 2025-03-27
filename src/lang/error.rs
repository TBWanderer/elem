use crate::utils::{Color, COLOR_RESET};
use atty::Stream;

pub fn fmt(
    module_name: &str,
    value_type: &str,
    value_name: &str,
    error_name: &str,
    description: &str,
) -> String {
    if atty::is(Stream::Stdout) {
        format!(
            "{COLOR_RESET}[{}] <{}> {}: {} - {}{COLOR_RESET}",
            module_name,
            value_type,
            value_name,
            Color::Red.bold().paint(error_name),
            Color::Red.paint(description)
        )
    } else {
        format!(
            "[{}] <{}> {}: {} - {}",
            module_name, value_type, value_name, error_name, description
        )
    }
}
