use std::{time::{Duration, Instant}, fmt::{Display, Debug}};
pub use colour;
pub use chrono;

pub struct TimeDisplay {
    pub duration: chrono::Duration,
}

impl TimeDisplay {
    pub fn new(dur: chrono::Duration) -> TimeDisplay {
        TimeDisplay { duration: dur }
    }
}

impl Display for TimeDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&format!("{} ms ({} microsecs)", &self.duration.num_milliseconds(), &self.duration.num_microseconds().unwrap()), f)
    }
}




#[macro_export]
///Print out all the runtime details into console
macro_rules! speed_test {
    ( $x:block ) => {
        let now = $crate::chrono::Local::now();

        //print place
        $crate::colour::blue_ln!("{}\nRunning block at: [{}:{}]", now, file!(), line!());
        
        //Reset color
        $crate::colour::white!();

        //Display dbg like output
        println!("[{}:{}] = {:#?}", file!(), line!(), $x);


        $crate::colour::blue!("Block ");
        $crate::colour::green!("[{}:{}]", file!(), line!());
        $crate::colour::blue!(" finished in ");
        $crate::colour::green_ln!("{}", $crate::TimeDisplay::new($crate::chrono::Local::now().signed_duration_since(now)));
        
        //Reset painter
        $crate::colour::white!();
    };
    ( $function:expr ) => {
        let now = $crate::chrono::Local::now();
        $crate::colour::blue!("{}\nRunning function at [{}:{}] ",now ,file!(), line!());
        $crate::colour::green_ln!("named: {}", stringify!($function));

        //Run function
        $crate::colour::white!();
        
        println!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!($function), $function);

        $crate::colour::blue!("Function ");
        $crate::colour::green!("{}", stringify!($function));
        $crate::colour::blue!(" finished in ");
        $crate::colour::green_ln!("{}", $crate::TimeDisplay::new($crate::chrono::Local::now().signed_duration_since(now)));

        //Reset painter
        $crate::colour::white!();
    };
    () => {};
}

#[macro_export]
///With this macro you can log all the runtime detalis into a file located in CARGO_MAINFEST_DIR
macro_rules! speed_test_log {
    ( $x:block ) => {

        let now = $crate::chrono::Local::now();

        let mut contents = format!("\n{}\nRunning block at: [{}:{}]\n", now, file!(), line!());

        contents.push_str(&format!("[{}:{}] = {:#?}\n", file!(), line!(), $x));

        contents.push_str(&format!("Block [{}:{}] finished in {}\n", file!(), line!(), $crate::TimeDisplay::new($crate::chrono::Local::now().signed_duration_since(now))));

        std::fs::write(format!("speed_test.log"), contents).unwrap();

    };
    ( $function:expr ) => {

        let mut file = OpenOptions::new().create(true).append(true).open(PathBuf::from(format!("speed_test.log"))).unwrap();

        let now = $crate::chrono::Local::now();

        let mut contents = format!("\n{}\nRunning function at: [{}:{}], named: {}\n", now, file!(), line!(), stringify!($function));

        contents.push_str(&format!("[{}:{}]: {} = {:#?}\n",file!(), line!(), stringify!($function) , $function));

        contents.push_str(&format!("Function at: [{}:{}], named {} finished in {}\n", file!(), line!(), stringify!($function), $crate::TimeDisplay::new($crate::chrono::Local::now().signed_duration_since(now))));

        file.write(contents.as_bytes()).unwrap();

    };
    () => {};
}