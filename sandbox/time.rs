use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn main() {
    let now = SystemTime::now();
    println!("{:?}", now);
    //    // we sleep for 2 seconds
    //    sleep(Duration::new(2, 0));
    //    match now.elapsed() {
    //        Ok(elapsed) => {
    //            // it prints '2'
    //            println!("{}", elapsed.as_secs());
    //        }
    //        Err(e) => {
    //            // an error occurred!
    //            println!("Error: {e:?}");
    //        }
    //    }
}
