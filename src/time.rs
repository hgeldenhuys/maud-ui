//! Duration formatting for session ages and turn timers.
//!
//! Two registers, one input (elapsed whole seconds):
//!   * [`relative_time`] — the compact readout that sits inline in a status
//!     line: `41s`, `2m 08s`, `4h`, `1d`. Granularity steps down as the
//!     duration grows — second-scale shows seconds, hour-scale shows only
//!     hours, day-scale only days — because at four hours nobody reads the
//!     minutes off a status line.
//!   * [`duration_long`] — the two-unit form for a label with room:
//!     `4h 12m`. [`asleep_label`] prefixes it for the composer's sleep bar.

const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;
const DAY: u64 = 24 * HOUR;

/// Compact age/duration readout for a status line.
///
/// ```
/// use maud_ui::time::relative_time;
/// assert_eq!(relative_time(41), "41s");
/// assert_eq!(relative_time(128), "2m 08s");
/// assert_eq!(relative_time(4 * 3600), "4h");
/// assert_eq!(relative_time(86_400), "1d");
/// ```
pub fn relative_time(secs: u64) -> String {
    if secs < MINUTE {
        format!("{secs}s")
    } else if secs < HOUR {
        format!("{}m {:02}s", secs / MINUTE, secs % MINUTE)
    } else if secs < DAY {
        format!("{}h", secs / HOUR)
    } else {
        format!("{}d", secs / DAY)
    }
}

/// Two-unit long form: `4h 12m` at hour scale, `2m 08s` below the hour,
/// `1d 04h` above the day. For a label that has the width the status line
/// does not.
///
/// ```
/// use maud_ui::time::duration_long;
/// assert_eq!(duration_long(41), "41s");
/// assert_eq!(duration_long(128), "2m 08s");
/// assert_eq!(duration_long(4 * 3600 + 12 * 60), "4h 12m");
/// assert_eq!(duration_long(86_400 + 4 * 3600), "1d 04h");
/// ```
pub fn duration_long(secs: u64) -> String {
    if secs < MINUTE {
        format!("{secs}s")
    } else if secs < HOUR {
        format!("{}m {:02}s", secs / MINUTE, secs % MINUTE)
    } else if secs < DAY {
        format!("{}h {:02}m", secs / HOUR, (secs % HOUR) / MINUTE)
    } else {
        format!("{}d {:02}h", secs / DAY, (secs % DAY) / HOUR)
    }
}

/// The composer sleep-bar label: `asleep 4h 12m`.
///
/// ```
/// use maud_ui::time::asleep_label;
/// assert_eq!(asleep_label(4 * 3600 + 12 * 60), "asleep 4h 12m");
/// ```
pub fn asleep_label(secs: u64) -> String {
    format!("asleep {}", duration_long(secs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_time_boundaries() {
        // second → minute
        assert_eq!(relative_time(59), "59s");
        assert_eq!(relative_time(60), "1m 00s");
        // within the minute band, seconds are zero-padded
        assert_eq!(relative_time(128), "2m 08s");
        // minute → hour: at 59m the seconds still show; at 60m they vanish
        assert_eq!(relative_time(59 * 60 + 59), "59m 59s");
        assert_eq!(relative_time(60 * 60), "1h");
        // hour → day
        assert_eq!(relative_time(23 * 3600), "23h");
        assert_eq!(relative_time(24 * 3600), "1d");
        // zero
        assert_eq!(relative_time(0), "0s");
    }

    #[test]
    fn duration_long_boundaries() {
        assert_eq!(duration_long(59), "59s");
        assert_eq!(duration_long(60), "1m 00s");
        assert_eq!(duration_long(59 * 60 + 59), "59m 59s");
        assert_eq!(duration_long(60 * 60), "1h 00m");
        assert_eq!(duration_long(4 * 3600 + 12 * 60), "4h 12m");
        assert_eq!(duration_long(23 * 3600 + 59 * 60), "23h 59m");
        assert_eq!(duration_long(24 * 3600), "1d 00h");
    }

    #[test]
    fn asleep_label_prefixes_the_long_form() {
        assert_eq!(asleep_label(4 * 3600 + 12 * 60), "asleep 4h 12m");
    }
}
