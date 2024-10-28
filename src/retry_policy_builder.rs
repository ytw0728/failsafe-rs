use std::time::Duration;
use crate::retry_policy::RetryPolicy;

pub type PolicyResult<T> = Result<T, &'static str>;

#[derive(Debug)]
pub struct RetryPolicyBuilder {
    policy: RetryPolicy,
}

impl RetryPolicyBuilder {
    pub fn new() -> Self {
        RetryPolicyBuilder {
            policy: RetryPolicy::default(),
        }
    }

    pub fn with_simple_backoff(
        self,
        delay: Duration,
        max_delay: Duration,
    ) -> PolicyResult<Self> {
        self.with_backoff(delay, max_delay, 2.0)
    }

    pub fn with_backoff(
        mut self,
        delay: Duration,
        max_delay: Duration,
        delay_factor: f64,
    ) -> PolicyResult<Self> {
        // Validate delay is positive and non-zero
        if delay.is_zero() {
            return Err("The delay must be greater than zero");
        }

        // Check if delay is less than max_duration if set
        if let Some(max_duration) = self.policy.get_config().max_duration() {
            if delay >= max_duration {
                return Err("Delay must be less than the max duration");
            }
        }

        // Check if delay is greater than or equal to jitter if set
        if let Some(jitter) = self.policy.get_config().jitter() {
            if delay < jitter {
                return Err("Delay must be greater than or equal to the jitter duration");
            }
        }

        // Check if delay is less than max_delay
        if delay >= max_delay {
            return Err("Delay must be less than the max delay");
        }

        // Check delay_factor
        if delay_factor <= 1.0 {
            return Err("Delay factor must be greater than 1");
        }

        // Set the values
        self.policy.get_config()
            .with_max_delay(Some(max_delay))
            .with_delay_factor(delay_factor)
            .with_delay_min(None)
            .with_delay_max(None);

        Ok(self)
    }

    pub fn with_delay(mut self, delay: Duration) -> PolicyResult<Self> {
        // Ensure delay is not None
        if delay.is_zero() {
            return Err("Delay must not be None");
        }

        // Convert delay to a safe duration (assuming this means ensuring it's non-negative)
        let delay = Duration::from_nanos(delay.as_nanos() as u64);

        // Ensure delay is less than max_duration if set
        if let Some(max_duration) = self.policy.get_config().max_duration() {
            if delay >= max_duration {
                return Err("Delay must be less than the max duration");
            }
        }

        // Ensure delay is greater than or equal to jitter if set
        if let Some(jitter) = self.policy.get_config().jitter() {
            if delay < jitter {
                return Err("Delay must be greater than or equal to the jitter duration");
            }
        }

        // Clear backoff and random delays
        self.policy.get_config()
            .with_max_delay(None)
            .with_delay_min(None)
            .with_delay_max(None);

        Ok(self)
    }

    pub fn with_delay_min_max(mut self, delay_min: Duration, delay_max: Duration) -> PolicyResult<Self> {
        // Ensure delay_min and delay_max are not None
        if delay_min.is_zero() || delay_max.is_zero() {
            return Err("delayMin and delayMax must not be None");
        }

        // Convert delay_min and delay_max to safe durations
        let delay_min = Duration::from_nanos(delay_min.as_nanos() as u64);
        let delay_max = Duration::from_nanos(delay_max.as_nanos() as u64);

        // Ensure delay_min is greater than 0
        if delay_min.is_zero() || delay_min < Duration::ZERO {
            return Err("delayMin must be greater than 0");
        }

        // Ensure delay_max is greater than 0
        if delay_max.is_zero() || delay_max < Duration::ZERO {
            return Err("delayMax must be greater than 0");
        }

        // Ensure delay_min is less than delay_max
        if delay_min >= delay_max {
            return Err("delayMin must be less than delayMax");
        }

        // Ensure delay_max is less than max_duration if set
        if let Some(max_duration) = self.policy.get_config().max_duration() {
            if delay_max >= max_duration {
                return Err("delayMax must be less than the max duration");
            }
        }

        // Ensure delay_min is greater than or equal to jitter if set
        if let Some(jitter) = self.policy.get_config().jitter() {
            if delay_min < jitter {
                return Err("delayMin must be greater than or equal to the jitter duration");
            }
        }

        // Set delay_min and delay_max in the configuration
        self.policy.get_config()
            .with_delay_min(Some(delay_min))
            .with_delay_max(Some(delay_max));

        // Clear fixed and random delays
        self.policy.get_config()
            //.with_delay(Some(Duration::ZERO))
            .with_max_delay(None);

        Ok(self)
    }

    pub fn with_jitter(mut self, jitter_factor: f64) -> PolicyResult<Self> {
        // Ensure jitter_factor is between 0.0 and 1.0 inclusive
        if jitter_factor < 0.0 || jitter_factor > 1.0 {
            return Err("jitterFactor must be >= 0 and <= 1");
        }

        // Set jitter_factor in the configuration
        self.policy.get_config().with_jitter_factor(jitter_factor);

        // Clear the jitter duration
        self.policy.get_config().with_jitter(None);

        Ok(self)
    }

    pub fn with_max_attempts(mut self, max_attempts: i32) -> PolicyResult<Self> {
        // Ensure max_attempts is not 0
        if max_attempts == 0 {
            return Err("maxAttempts cannot be 0");
        }

        // Ensure max_attempts is greater than or equal to -1
        if max_attempts < -1 {
            return Err("maxAttempts must be >= -1");
        }

        // Set max_retries in the configuration
        self.policy.get_config().with_max_retries(if max_attempts == -1 { -1 } else { max_attempts - 1 });

        Ok(self)
    }

    pub fn with_max_duration(mut self, max_duration: Duration) -> PolicyResult<Self> {
        // Ensure max_duration is not None
        if max_duration.is_zero() {
            return Err("maxDuration must not be None");
        }

        // Convert max_duration to a safe duration
        let max_duration = Duration::from_nanos(max_duration.as_nanos() as u64);

        // Ensure max_duration is greater than the current delay
        if let Some(delay) = self.policy.get_config().delay_min() {
            if max_duration <= delay {
                return Err("maxDuration must be greater than the delay");
            }
        }

        // Ensure max_duration is greater than delay_max if set
        if let Some(delay_max) = self.policy.get_config().delay_max() {
            if max_duration <= delay_max {
                return Err("maxDuration must be greater than the max random delay");
            }
        }

        // Set max_duration in the configuration
        self.policy.get_config().with_max_duration(Some(max_duration));

        Ok(self)
    }

    pub fn with_max_retries(mut self, max_retries: i32) -> PolicyResult<Self> {
        // Ensure max_retries is greater than or equal to -1
        if max_retries < -1 {
            return Err("maxRetries must be >= -1");
        }

        // Set max_retries in the configuration
        self.policy.get_config().with_max_retries(max_retries);

        Ok(self)
    }

    pub fn build(self) -> RetryPolicy {
        self.policy
    }
}