use std::time::Duration;

pub struct Config {
    pub urls: Vec<String>,
    pub num_workers: usize,
    pub timeout: Duration,
}

impl Config {
    pub fn load() -> Self {
        Self {
            urls: vec![
                "https://www.google.com".to_string(),
                "https://www.youtube.com".to_string(),
                "https://www.facebook.com".to_string(),
                "https://www.amazon.com".to_string(),
                "https://www.wikipedia.org".to_string(),
                "https://www.reddit.com".to_string(),
                "https://www.ebay.com".to_string(),
                "https://www.twitter.com".to_string(),
                "https://www.instagram.com".to_string(),
                "https://www.linkedin.com".to_string(),
                "https://www.apple.com".to_string(),
                "https://www.microsoft.com".to_string(),
                "https://www.netflix.com".to_string(),
                "https://www.spotify.com".to_string(),
                "https://www.nytimes.com".to_string(),
                "https://www.cnn.com".to_string(),
                "https://www.bbc.com".to_string(),
                "https://www.yahoo.com".to_string(),
                "https://www.imdb.com".to_string(),
                "https://www.pinterest.com".to_string(),
                "https://www.tumblr.com".to_string(),
                "https://www.quora.com".to_string(),
                "https://www.medium.com".to_string(),
                "https://www.stackoverflow.com".to_string(),
                "https://www.github.com".to_string(),
                "https://www.gitlab.com".to_string(),
                "https://www.digitalocean.com".to_string(),
                "https://www.heroku.com".to_string(),
                "https://www.cloudflare.com".to_string(),
                "https://www.salesforce.com".to_string(),
                "https://www.bloomberg.com".to_string(), 
                "https://www.udemy.com".to_string(),    
                "https://www.khanacademy.org".to_string(), 
                "https://www.oracle.com".to_string(),
                "https://www.intel.com".to_string(),
                "https://www.nike.com".to_string(),
                "https://www.airbnb.com".to_string(),
                "https://www.booking.com".to_string(),
                "https://www.uber.com".to_string(),
                "https://www.lyft.com".to_string(),
                "https://www.walmart.com".to_string(),
                "https://www.target.com".to_string(),
                "https://www.costco.com".to_string(),
                "https://www.bestbuy.com".to_string(),
                "https://www.zillow.com".to_string(),
                "https://www.etsy.com".to_string(),
                "https://www.weather.com".to_string(),
                "https://www.time.com".to_string(),
                "https://www.forbes.com".to_string(),
                "https://www.wsj.com".to_string(),
            ],
            num_workers: 10, // Adjust based on testing
            timeout: Duration::from_secs(5),
        }
    }
}



