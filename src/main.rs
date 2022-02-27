use buttplug::client::{
    ButtplugClient, ButtplugClientDeviceMessageType, ButtplugClientEvent, VibrateCommand,
};
use futures::StreamExt;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::mpsc,
};

async fn buttplug_loop(mut event_receiver: mpsc::Receiver<[u8; 2]>) {
    let client = ButtplugClient::new("test client");
    let mut event_stream = client.event_stream();
    println!("Client connecting...");
    client.connect_in_process(None).await.unwrap();
    println!("Client initiating scan...");
    if let Err(err) = client.start_scanning().await {
        println!("Client errored when starting scan! {}", err);
        return;
    }
    println!("Client scanning...");
    let mut devices = vec![];
    loop {
        tokio::select! {
            buttplug_event = event_stream.next() => {
                match buttplug_event.unwrap() {
                    ButtplugClientEvent::DeviceAdded(device) => {
                        if !device.name.contains("XInput") &&
                          device
                            .allowed_messages
                            .contains_key(&ButtplugClientDeviceMessageType::VibrateCmd)
                        {
                            println!("Adding devices {}", device.name);
                            devices.push(device);
                        }
                    }
                    _ => {}
                }
            }
            rumble_event = event_receiver.recv() => {
                let rumble_event = rumble_event.unwrap();
                for device in &devices {
                    device.vibrate(VibrateCommand::Speed((rumble_event[0] as f64 + rumble_event[1] as f64) / (0xff * 2) as f64)).await.unwrap();
                }
            }
        };
    }
}

#[tokio::main]
async fn main() {
    let (sender, receiver) = mpsc::channel(255);
    tokio::spawn(async move {
        println!("Running");
        buttplug_loop(receiver).await
    });

    let program_args = vec![
        "-o",
        "gamepad6.pcap",
        "--devices",
        "35",
        "-d",
        "\\\\.\\USBPcap1",
    ];
    let mut process =
        tokio::process::Command::new("c:\\Users\\qdot\\code\\elden-cockring\\hapticscap.exe")
            .args(program_args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .unwrap();
    println!("{:?}", process);
    //process.wait().await.unwrap();
    let stdout = process.stdout.take().unwrap();
    let mut stdout_reader = BufReader::new(stdout).lines();
    tokio::spawn(async move {
        println!("Waiting for process exit");
        let status = process
            .wait()
            .await
            .expect("child process encountered an error");
        println!("child status was: {}", status);
    });
    while let Ok(Some(line)) = stdout_reader.next_line().await {
        let speed_array = serde_json::from_str::<[u8; 2]>(&line).unwrap();
        println!("{:?}", speed_array);
        sender.send(speed_array).await.unwrap();
    }
    println!("Exited");
}
