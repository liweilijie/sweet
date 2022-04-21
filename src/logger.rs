use time::macros::format_description;
use time::UtcOffset;
use tracing::info;
use tracing_subscriber::fmt::time::OffsetTime;

pub struct Logger;

impl Logger {
    pub fn init() {
        // 第三种, 自定义时间显示格式
        let local_time = OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
            ),
        );

        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "sweet=debug".to_string()),
            ))
            .with_timer(local_time)
            .init();

        info!("log init.");
    }
}
