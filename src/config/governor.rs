use std::net::IpAddr;
use std::num::NonZeroU32;

use axum_governor::{
    BodyPreset, GovernorConfigBuilder, GovernorLayer, Quota, extractor::PeerIp, nz,
};

use crate::config::env::AppConfig;

/// Build a rate-limiting layer keyed on the peer IP, enforcing every quota in
/// `quotas` (for example a per-minute and a per-hour window). The first window
/// to be exceeded rejects the request.
///
/// The configuration is fully specified here, so `finish()` cannot fail; the
/// `expect` documents that invariant rather than handling an unreachable error.
fn build_governor(
    config: &AppConfig,
    name: &'static str,
    quotas: impl IntoIterator<Item = Quota>,
) -> GovernorLayer<IpAddr> {
    let config = GovernorConfigBuilder::default()
        .with_extractor(PeerIp::default())
        .expect_connect_info()
        .max_keys(config.rate_limit_max_keys.max(1))
        .redact_keys(true)
        .body_preset(BodyPreset::Json)
        .quotas(name, PeerIp::default(), quotas)
        .finish()
        .expect("governor configuration is fully specified");

    GovernorLayer::new(config)
}

impl AppConfig {
    /// Strict quota for one-time-password verification and request endpoints.
    pub fn otp_governor(&self) -> GovernorLayer<IpAddr> {
        build_governor(
            self,
            "otp",
            [
                Quota::requests_per_minute(non_zero_or(self.rate_limit_otp_per_minute, 2)),
                Quota::requests_per_hour(non_zero_or(self.rate_limit_otp_per_hour, 10)),
            ],
        )
    }

    /// Quota for login, signup, forgotten-password, refresh and confirmation.
    pub fn auth_governor(&self) -> GovernorLayer<IpAddr> {
        build_governor(
            self,
            "auth",
            [
                Quota::requests_per_minute(non_zero_or(self.rate_limit_auth_per_minute, 5)),
                Quota::requests_per_hour(non_zero_or(self.rate_limit_auth_per_hour, 30)),
            ],
        )
    }

    /// Quota for the anonymous contact form and newsletter subscription.
    pub fn contact_governor(&self) -> GovernorLayer<IpAddr> {
        build_governor(self, "contact", [Quota::requests_per_hour(nz!(3u32))])
    }
}

pub fn non_zero_or(value: u32, fallback: u32) -> NonZeroU32 {
    NonZeroU32::new(value)
        .or_else(|| NonZeroU32::new(fallback))
        .unwrap_or(nz!(1u32))
}
