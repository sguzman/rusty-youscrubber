use yourust::validate_json_files;

use log::info;

// Set logging to debug
fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.filter_level(log::LevelFilter::Debug);
    builder.init();
}

fn main() {
    init_logger();
    info!("Hello, world!");
    validate_json_files();
}
