use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let offset = Duration::seconds(1_000_000_000);
    start.saturating_add(offset)
}
