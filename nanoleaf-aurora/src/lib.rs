use messages::home_automation_msgs;

pub mod utils;

pub async fn run(
    ros_url: &str,
    nanoleaf_config: &configuration::AuroraSettings,
) -> std::io::Result<()> {
    let mut client = roslibrust::ClientHandle::new(ros_url)
        .await
        .map_err(utils::to_io_error)?;

    let nanoleaf_client =
        borealis::client::Aurora::new(&nanoleaf_config.address, &nanoleaf_config.token)
            .expect("Failed to create nanoleaf client");

    let rx = client
        .subscribe::<home_automation_msgs::SetOnState>("/set_nanoleaf_state")
        .await
        .map_err(utils::to_io_error)?;

    loop {
        let incoming_msg: home_automation_msgs::SetOnState = rx.next().await;
        if incoming_msg.set_on {
            nanoleaf_client.turn_on().await.unwrap();
        } else {
            nanoleaf_client.turn_off().await.unwrap();
        }
    }
}
