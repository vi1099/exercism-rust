use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {

    // gigasecond = one thousand million seconds || 1,000, 000, 000 (billion)
    let gigasecond_duration: Duration = Duration::seconds(1000000000);

    //given a datetime we need to add 1 gigasecond to it. then return that datetime value

    //1. add duration
    let after: Option<DateTime> = start.checked_add( gigasecond_duration );

    return after.unwrap()
    
}
