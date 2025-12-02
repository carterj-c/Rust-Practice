use std::time::Duration;

use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // Create duration to add to start object
    let billion_seconds = 1_000_000_000;
    let timespan: Duration = Duration::from_secs(billion_seconds);

    // Add duration to start object
    start + timespan
}
