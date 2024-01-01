use std::{time::Duration, fmt::{Display, Debug}, thread::sleep};
use colour;
use speed_test::*;

struct TimeDisplay {
    pub duration: Duration,
}

impl TimeDisplay {
    fn new(dur: Duration) -> TimeDisplay {
        TimeDisplay { duration: dur }
    }
}

impl Display for TimeDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.duration.fmt(f)
    }
}

fn expensive_calculation(dur: Duration) -> () {

    let calculation = 3461235000023.234423f64 / 234235.23423443554323;

    sleep(dur);

    println!("{calculation}");
    
}

fn main() {

    let dur = Duration::from_micros(4000);

    speed_test!(expensive_calculation(dur));

}

#[test]
fn test() {
    //Default duration
    let dur = Duration::from_secs(1);

    //Speed testing code blocks
    speed_test!(
        {
            sleep(Duration::from_secs(1))
        }
    );

    //Speed testing functions
    speed_test!(
        expensive_calculation(dur)
    );

}