use std::time::{Duration, Instant};
use ureq;

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
}

pub fn check_website(url: &str, timeout: Duration) -> WebsiteStatus {
    let start = Instant::now();
    let response = ureq::get(url)
        .timeout(timeout)
        .call();

    let duration = start.elapsed();

    let status = match response {
        Ok(res) => Ok(res.status()), 
        Err(ureq::Error::Status(code, _)) => Ok(code), 
        Err(err) => Err(err.to_string()), 
    };

    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time: duration,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_invalid_url() {
        let url = "invalid_url"; 
        let timeout = Duration::from_secs(5);
        let result = check_website(url, timeout);

        assert!(result.status.is_err()); 
    }

    #[test]
    fn test_unreachable_host() {
        let url = "http://10.255.255.1"; 
        let timeout = Duration::from_secs(2);
        let result = check_website(url, timeout);

        assert!(result.status.is_err()); 
    }

    #[test]
    fn test_timeout() {
        let url = "https://httpstat.us/200?sleep=5000"; 
        let timeout = Duration::from_secs(1); 
        let result = check_website(url, timeout);

        assert!(result.status.is_err()); 
    }

    #[test]
    fn test_http_error() {
        let url = "https://httpstat.us/404";
        let timeout = Duration::from_secs(5);
        let result = check_website(url, timeout);
    
        assert!(result.status.is_ok()); 
        assert_eq!(result.status.unwrap(), 404); 
    }
}    
