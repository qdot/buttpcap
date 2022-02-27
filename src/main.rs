use buttplug::client::{
    ButtplugClient, ButtplugClientDeviceMessageType, ButtplugClientEvent, VibrateCommand,
};
use futures::StreamExt;
use tokio::{
    io::AsyncReadExt,
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
                        // Skip XBox controllers otherwise we'll fuck up their haptics, since that's
                        // what drives this program in the first place.
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
                println!("{:?}", rumble_event);
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
        "-",
        "--devices",
        "6",
        "-d",
        "\\\\.\\USBPcap1",
    ];
    let mut process =
        tokio::process::Command::new("c:\\Program Files\\USBPcap\\USBPcapCMD.exe")
            .args(program_args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .unwrap();
    let mut stdout = process.stdout.take().unwrap();
    tokio::spawn(async move {
        println!("Waiting for process exit");
        let status = process
            .wait()
            .await
            .expect("child process encountered an error");
        println!("child status was: {}", status);
    });
    let mut buf = vec![0u8; 1024];
    while let Ok(size) = &stdout.read(&mut buf).await {
        // I mean we *could* parse out pcap but meh.
        if *size > 38 && buf[37] == 0x02 && buf[39] == 0x0d {
            sender.send([buf[51], buf[52]]).await.unwrap();
        }
    }
    println!("Exited");
}
