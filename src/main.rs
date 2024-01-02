use std::{time::Duration, thread::sleep};
use speed_test::*;

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