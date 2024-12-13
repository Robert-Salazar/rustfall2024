use FP2::config::Config;
use FP2::thread_pool::ThreadPool;
use std::sync::mpsc;

fn main() {
    let config = Config::load();

    let thread_pool = ThreadPool::new(config.num_workers);

    let (tx, rx) = mpsc::channel();

    for url in config.urls.clone() {
        let tx = tx.clone();
        let timeout = config.timeout;
        thread_pool.execute(move || {
            let status = FP2::checker::check_website(&url, timeout);
            tx.send(status).expect("Failed to send status");
        });
    }

    drop(tx); 

    for result in rx {
        println!("{:?}", result);
    }
}
