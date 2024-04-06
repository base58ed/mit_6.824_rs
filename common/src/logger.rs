use tracing_subscriber::fmt;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let timer = time::format_description::parse(
        "[year]-[month padding:zero]-[day padding:zero] [hour]:[minute]:[second]",
    )
        .expect("failed to parse time format");
    let timer = fmt::time::OffsetTime::new(time::UtcOffset::UTC, timer);
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_thread_ids(true)
        .with_timer(timer)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
