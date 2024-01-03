use std::{time::Duration, thread::sleep, fs::OpenOptions, path::PathBuf, io::Write};
use speed_test::*;

fn expensive_calculation(dur: Duration) -> f64 {

    let calculation = 3461235000023.234423f64 / 234235.23423443554323;

    sleep(dur);

    calculation
    
}

fn main() {

    let dur = Duration::from_secs(2);

    speed_test!(expensive_calculation(dur));

    speed_test!(
        {
            speed_test_log!(expensive_calculation(dur));
        }
    );

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
    speed_test_log!(
        expensive_calculation(dur)
    );

}