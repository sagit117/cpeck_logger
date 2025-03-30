use cpeck_logger::{LogLevel, Logger};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::create("test.txt")?;

    let mut logger = Logger {
        out: Box::new(f),
        level: LogLevel::ALL,
        formatter: |message: &str, level: LogLevel| -> String{
            format!("[{: <7}] {}\n", level.to_string(), message)
        }
    };

    logger.error("Ошибка")?;
    logger.info("Информация")?;

    Ok(())
}
