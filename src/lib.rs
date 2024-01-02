use std::{time::Duration, fmt::{Display, Debug}};

pub struct TimeDisplay {
    pub duration: Duration,
}

impl TimeDisplay {
    pub fn new(dur: Duration) -> TimeDisplay {
        TimeDisplay { duration: dur }
    }
}

impl Display for TimeDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.duration.fmt(f)
    }
}

#[macro_export]
macro_rules! speed_test {
    ( $x:block ) => {
        let now = std::time::Instant::now();
        $x;
        println!("{}", TimeDisplay::new(now.elapsed()));
    };
    ( $function:expr ) => {
        let now = std::time::Instant::now();
        colour::red!("Running function: ");
        colour::green_ln!("{}", stringify!($function));

        //Run function
        colour::white!();
        $function;

        colour::red!("Function ");
        colour::green!("{}", stringify!($function));
        colour::red!(" finished in ");
        colour::green_ln!("{}", TimeDisplay::new(now.elapsed()));

        //Reset painter
        colour::white!();
    };
}