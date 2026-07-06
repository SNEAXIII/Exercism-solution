#![feature(int_roundings)]
use std::{fmt};

#[derive(Debug,PartialEq,Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let extra_hour = minutes/60;
        let mut new_hours = hours+extra_hour;
        let new_minutes;
        if minutes != -60 {
            if minutes < 0 {
                new_hours-=1
            }
            new_minutes = minutes.rem_euclid(60);
        } else {
            new_minutes = 0;
        }
        new_hours = new_hours.rem_euclid(24);
        Clock{hours:new_hours,minutes:new_minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours,self.minutes+minutes)
    }
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}",self.hours,self.minutes)
    }
}