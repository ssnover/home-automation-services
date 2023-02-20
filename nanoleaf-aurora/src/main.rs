use configuration::get_configuration;
use nanoleaf_aurora::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Unable to read config");
    let ros_url = format!("ws://localhost:{}", config.ros.port);
    run(&ros_url, &config.aurora).await
}
