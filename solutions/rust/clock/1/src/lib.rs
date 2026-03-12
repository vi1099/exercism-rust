use std::fmt;

//Not explicitly mentioned in instructions, but compiler gave errors
//Needed to add these since tests check for equality check and need debug attributes
#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {

    //constructor
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (final_hours, final_minutes) = Self::normalize( hours, minutes );
        Clock {
            hours: final_hours,
            minutes: final_minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes: i32 = self.minutes + minutes;

        let (final_hours, final_minutes) = Self::normalize( self.hours, total_minutes );
        
        Self {
            hours: final_hours,
            minutes: final_minutes
        }
    }

    fn normalize(hours: i32, minutes: i32) -> (i32, i32) {
        //To deal with negative instantiations, we need to convert time into total minutes and normalize from there
        let minutes_in_day: i32 = 1440;

        //1. get the total time in minutes
        let total_time_in_minutes: i32 = (hours * 60) + minutes;

        //2. Get remainder of days in the total time (we are reducing potentially large numbers)
        let remainder_time = total_time_in_minutes % minutes_in_day;

        //3. Remainder time should be a small number but in case it's negative we need to shift to the "next day"
        let shifted_minutes = remainder_time + minutes_in_day;

        //4. we need to wrap around to ensure we are within the 24 hour period
        let total_normalized_minutes = shifted_minutes % minutes_in_day;
        
        // since every 60 minutes is an hour, get how many minutes are remaining
        let normalized_minutes: i32 = total_normalized_minutes % 60;

        // get how many hours are in the total minutes
        let normalized_hours: i32 = total_normalized_minutes / 60;

        (normalized_hours, normalized_minutes)
    }
}

// Needed to implement, referred to the rust book ch. 20.2
// Also referred to the fmt::Display documentation for formatting usage
// Similar to to_string()
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:02}:{:02}", self.hours, self.minutes)
        }
}
