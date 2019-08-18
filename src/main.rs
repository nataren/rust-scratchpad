use std::{thread, time};
use log::{info, warn};

fn main() {
    env_logger::init();
    info!("starting up");

    let one_millis = time::Duration::from_millis(1);

    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        thread::sleep(one_millis);
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    warn!("nothing implemented...");
}
