/*// a)
use chrono::{DateTime, TimeZone, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // "What time is a gigasecond (10^9 seconds) later than 'start';
    Utc.timestamp(start.timestamp() + 1000000000, 0)
}
 */

// b)
use chrono::{DateTime, Duration, Utc};

const GIGASECOND: i64 = 1e9 as i64;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + (Duration::seconds(GIGASECOND))
    /*// I did not see any notable difference when using the error handling version, so I left the code here, but as remark. (Somebody can explain to me the actual use?)
    start
       .checked_add_signed(Duration::seconds(GIGASECOND))
       .expect("overflow!")*/
}
