use std::io;
use std::io::Write;
use std::ops::Rem;
use std::time::{Duration, Instant};

pub fn print_progress(
    start: Instant,
    fraction_complete: f32,
) {
    print!(
        "\r{:>3}% {:>20} elapsed. eta: {:<20}",
        (fraction_complete * 100.).floor(),
        format_rough_duration(start.elapsed()),
        format_remaining_secs(start, fraction_complete)
    );
    io::stdout().flush().unwrap();
}

fn format_remaining_secs(
    start: Instant,
    fraction_complete: f32,
) -> String {
    let remaining_secs = (start.elapsed().as_secs()
        as f32
        / (fraction_complete))
        * (1. - fraction_complete);
    format_seconds(remaining_secs as u64)
}

/// Human-readable representation of duration
/// Ignoring anything below a second.
pub fn format_rough_duration(
    duration: Duration,
) -> String {
    format_seconds(duration.as_secs())
}

/// Human-readable string for a number of seconds.
///
/// The default time::Duration format doesn't show minutes,
/// hours etc.
fn format_seconds(seconds: u64) -> String {
    const ONE_MINUTE: u64 = 60;
    const ONE_HOUR: u64 = ONE_MINUTE * 60;
    const ONE_DAY: u64 = ONE_HOUR * 24;

    let mut secs = seconds;
    let mut out = String::new();
    if secs > ONE_DAY {
        out += &format!("{}d", secs / ONE_DAY);
        secs = secs.rem(ONE_DAY);
    }
    if secs > ONE_HOUR {
        out += &format!("{}h", secs / ONE_HOUR);
        secs = secs.rem(ONE_HOUR);
    }
    if secs > ONE_MINUTE {
        out += &format!("{}m", secs / ONE_MINUTE);
        secs = secs.rem(ONE_MINUTE);
    }
    if secs > 0 {
        out += &format!("{}s", secs);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_seconds() {
        assert_eq!(format_seconds(3), "3s");
        assert_eq!(format_seconds(65), "1m5s");
        assert_eq!(format_seconds(178), "2m58s");
        assert_eq!(format_seconds(18234), "5h3m54s");
        assert_eq!(
            format_seconds(500000),
            "5d18h53m20s"
        );
    }
}
