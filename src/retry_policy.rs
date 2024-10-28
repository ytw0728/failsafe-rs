use crate::retry_policy_config::RetryPolicyConfig;
use crate::retry_policy_builder::RetryPolicyBuilder;

/// Represents a retry policy with configurable parameters for managing retry attempts.
///
/// This struct encapsulates various settings that control the behavior of retry operations,
/// such as the maximum number of attempts, delays between retries, and backoff strategies.
#[derive(Debug)]
#[derive(Default)]
pub struct RetryPolicy {
    config: RetryPolicyConfig,
}

impl RetryPolicy {
    pub fn builder() -> RetryPolicyBuilder {
        RetryPolicyBuilder::new()
    }

    pub fn of_defaults() -> Self {
        Self::builder().build()
    }

    pub fn get_config(&mut self) -> &mut RetryPolicyConfig {
        &mut self.config
    }
}

#[cfg(test)]
mod tests {
    mod build_test {
        use core::time::Duration;
        use crate::retry_policy::RetryPolicy;

        #[test]
        fn build_test() {
            let mut policy = RetryPolicy::builder()
                .with_simple_backoff(
                    Duration::from_secs(1),
                    Duration::from_secs(10),
                ).expect("Failed to set simple backoff")
                .with_jitter(0.5).expect("Failed to set jitter")
                .with_max_attempts(5).expect("Failed to set max attempts")
                .with_max_duration(Duration::from_secs(30)).expect("Failed to set max duration")
                .with_max_retries(3).expect("Failed to set max retries")
                .build();

            let config = policy.get_config();
            assert_eq!(config.max_delay(), Some(Duration::from_secs(10)));
            assert_eq!(config.delay_factor(), 2.0);  // simple_backoff의 기본값
            assert_eq!(config.jitter_factor(), 0.5);
            assert_eq!(config.max_retries(), 3);
            assert_eq!(config.max_duration(), Some(Duration::from_secs(30)));
        }
    }
}