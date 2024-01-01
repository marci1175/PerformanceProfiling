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