use std::{time::Duration, fmt::{Display, Debug}};
pub use colour;

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

        //print place
        $crate::colour::red_ln!("Running block at: [{}:{}]", file!(), line!());
        
        //Reset color
        $crate::colour::white!();

        //Display dbg like output
        println!("[{}:{}] = {:#?}", file!(), line!(), $x);


        $crate::colour::red!("Block ");
        $crate::colour::green!("[{}:{}]", file!(), line!());
        $crate::colour::red!(" finished in ");
        $crate::colour::green_ln!("{}", $crate::TimeDisplay::new(now.elapsed()));
    };
    ( $function:expr ) => {
        let now = std::time::Instant::now();
        $crate::colour::red!("Running function: ");
        $crate::colour::green_ln!("{}", stringify!($function));

        //Run function
        $crate::colour::white!();
        
        println!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!($function), $function);

        $crate::colour::red!("Function ");
        $crate::colour::green!("{}", stringify!($function));
        $crate::colour::red!(" finished in ");
        $crate::colour::green_ln!("{}", $crate::TimeDisplay::new(now.elapsed()));

        //Reset painter
        $crate::colour::white!();
    };
}