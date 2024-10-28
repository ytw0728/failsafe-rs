use std::time::Duration;

/// Configuration for RetryPolicy that specifies retry behavior settings.
#[derive(Debug)]
pub struct RetryPolicyConfig {
    delay_min: Option<Duration>,
    delay_max: Option<Duration>,
    delay_factor: f64,
    max_delay: Option<Duration>,
    jitter: Option<Duration>,
    jitter_factor: f64,
    max_duration: Option<Duration>,
    max_retries: i32,
}

impl Default for RetryPolicyConfig {
    fn default() -> Self {
        Self {
            delay_min: None,
            delay_max: None,
            delay_factor: 1.0,
            max_delay: None,
            jitter: None,
            jitter_factor: 0.0,
            max_duration: None,
            max_retries: 0,
        }
    }
}

impl RetryPolicyConfig {
    /// Creates a new RetryPolicyConfig with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the minimum delay between retries
    pub fn with_delay_min(&mut self, delay: Option<Duration>) -> &mut Self {
        self.delay_min = delay;
        self
    }

    /// Sets the maximum delay between retries
    pub fn with_delay_max(&mut self, delay: Option<Duration>) -> &mut Self {
        self.delay_max = delay;
        self
    }

    /// Sets the delay factor
    pub fn with_delay_factor(&mut self, factor: f64) -> &mut Self {
        self.delay_factor = factor;
        self
    }

    /// Sets the maximum delay
    pub fn with_max_delay(&mut self, delay: Option<Duration>) -> &mut Self {
        self.max_delay = delay;
        self
    }

    /// Sets the jitter duration
    pub fn with_jitter(&mut self, jitter: Option<Duration>) -> &mut Self {
        self.jitter = jitter;
        self
    }

    /// Sets the jitter factor
    pub fn with_jitter_factor(&mut self, factor: f64) -> &mut Self {
        self.jitter_factor = factor;
        self
    }

    /// Sets the maximum duration for retries
    pub fn with_max_duration(&mut self, duration: Option<Duration>) -> &mut Self {
        self.max_duration = duration;
        self
    }

    /// Sets the maximum number of retries
    pub fn with_max_retries(&mut self, retries: i32) -> &mut Self {
        self.max_retries = retries;
        self
    }

    // Getter methods
    pub fn delay_min(&self) -> Option<Duration> {
        self.delay_min
    }

    pub fn delay_max(&self) -> Option<Duration> {
        self.delay_max
    }

    pub fn delay_factor(&self) -> f64 {
        self.delay_factor
    }

    pub fn max_delay(&self) -> Option<Duration> {
        self.max_delay
    }

    pub fn jitter(&self) -> Option<Duration> {
        self.jitter
    }

    pub fn jitter_factor(&self) -> f64 {
        self.jitter_factor
    }

    pub fn max_duration(&self) -> Option<Duration> {
        self.max_duration
    }

    pub fn max_retries(&self) -> i32 {
        self.max_retries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_policy_config() {
        let mut config: RetryPolicyConfig = RetryPolicyConfig::new();
        config
            .with_delay_min(Some(Duration::from_secs(1)))
            .with_delay_max(Some(Duration::from_secs(5)))
            .with_delay_factor(2.0)
            .with_max_delay(Some(Duration::from_secs(10)))
            .with_jitter(Some(Duration::from_millis(100)))
            .with_jitter_factor(0.5)
            .with_max_duration(Some(Duration::from_secs(30)))
            .with_max_retries(3);

        assert_eq!(config.delay_min(), Some(Duration::from_secs(1)));
        assert_eq!(config.delay_max(), Some(Duration::from_secs(5)));
        assert_eq!(config.delay_factor(), 2.0);
        assert_eq!(config.max_delay(), Some(Duration::from_secs(10)));
        assert_eq!(config.jitter(), Some(Duration::from_millis(100)));
        assert_eq!(config.jitter_factor(), 0.5);
        assert_eq!(config.max_duration(), Some(Duration::from_secs(30)));
        assert_eq!(config.max_retries(), 3);
    }
}