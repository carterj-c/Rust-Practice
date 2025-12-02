use std::fmt; 
#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Transform into minutes only
        // Need to handle 1440 and rollover hours and minutes here as well
        let added_minutes: i32 = hours * 60 + minutes;
        Clock {
            minutes: Self::wraparound(added_minutes)
        }
        
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {

        let added_minutes: i32 = self.minutes + minutes;

        Clock {
            minutes: Self::wraparound(added_minutes)
        }
    }

    fn wraparound(minutes: i32) -> i32 {
        ((minutes % 1440) + 1440) % 1440
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We need to calculate hours and minutes from self.minutes here
        // then write them to 'f'
        let total_minutes: i32 = self.minutes;

        // Start with floor division for hours
        let hours: i32 = total_minutes / 60;
        let minutes: i32 = total_minutes - (hours * 60);

        write!(f, "{:02}:{:02}", hours, minutes)

    }
}