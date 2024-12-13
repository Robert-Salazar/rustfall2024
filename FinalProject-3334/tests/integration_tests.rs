use FP2::checker;
use FP2::config::Config;
use std::time::Instant;

fn main() {
    let config = Config::load();
    let urls = config.urls;

    let start = Instant::now();
    for url in &urls {
        let _ = checker::check_website(url, config.timeout);
    }
    let duration = start.elapsed();

    println!("Performance Test: Checked {} URLs in {:?}", urls.len(), duration);
}

