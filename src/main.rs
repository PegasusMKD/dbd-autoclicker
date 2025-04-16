use tokio::task::JoinSet;
use tracing::info;

pub mod keyboard;
pub mod mouse;
pub mod processor;

fn setup_logging(config: &Config) -> tracing_appender::non_blocking::WorkerGuard {
    let parent_dir = "./logs/run/";
    let (non_blocking, guard) = if let Some(file) = &config.log_file {
        let file_appender = tracing_appender::rolling::daily(parent_dir, file);
        tracing_appender::non_blocking(file_appender)
    } else {
        tracing_appender::non_blocking(std::io::stdout())
    };

    let my_crate = env!("CARGO_CRATE_NAME");

    // Replace `your_crate_name` with the actual name of your crate.
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive("info".parse().unwrap()) // default for all other crates
        .parse_lossy(&format!("{my_crate}=debug"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NEW)
        .with_writer(non_blocking)
        .init();

    guard
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = configuration::read_config();

    let guard = setup_logging(&config);
    info!("Started with the following configuration: {}", config);

    let mut handles = JoinSet::new();

    let (keyboard_tx, keyboard_rx) = async_channel::unbounded();
    let (mouse_tx, mouse_rx) = async_channel::unbounded();

    keyboard::track(keyboard_tx).await;
    mouse::track(keyboard_rx, mouse_tx).await;
    processor::process(mouse_rx).await;

    // TODO: Keyboard thread - channel for keyboard input
    // TODO: Mouse thread - channel for mouse input (location, screenshot data)
    // TODO: Processor thread - receives inputs, and based on them, decides whether to click on the
    // mouse

    while let Some(res) = handles.join_next().await {
        res.unwrap();
    }

    drop(guard);

    Ok(())
}
