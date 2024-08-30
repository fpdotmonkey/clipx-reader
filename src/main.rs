use std::{sync::Arc, time::Duration};

use argh::FromArgs;
use env_logger::Env;
use ethercrab::{
    std::{ethercat_now, tx_rx_task},
    MainDevice, MainDeviceConfig, PduStorage, Timeouts,
};
use prost::Message;
use tokio::{net, time};

mod clipx;
pub mod monitor {
    // protobuf generated
    include!(concat!(env!("OUT_DIR"), "/monitor.rs"));

    pub fn to_csv(measurement: &ClipxMeasurement) -> csv::Result<()> {
        let mut writer = csv::Writer::from_writer(std::io::stdout());
        let time = measurement.time_millis;
        let Some(ref result) = measurement.result else {
            writer.write_record(&[time.to_string(), "None".into()])?;
            return Ok(());
        };
        let mut record = vec![time.to_string(), "Some".into()];
        match result {
            clipx_measurement::Result::Ok(signals) => {
                record.push("Signals".into());
                for signal in &signals.signal {
                    record.push(signal.r#type().as_str_name().into());
                    record.push(signal.value().to_string());
                }
            }
            clipx_measurement::Result::Err(error) => {
                record.push("Error".into());
                let Some(ref error_kind) = error.error_kind else {
                    record.push("Unknown".into());
                    writer.write_record(&record)?;
                    return Ok(());
                };
                match error_kind {
                    clipx_measurement::error::ErrorKind::Clipx(errors) => {
                        for fault in &errors.error {
                            if let Some(kind) = fault.kind {
                                match kind {
                                    clipx_measurement::error::clipx::fault::Kind::Device(code) => {
                                        record.push("Device".into());
                                        record.push(code.to_string());
                                    }
                                    clipx_measurement::error::clipx::fault::Kind::Internal(
                                        code,
                                    ) => {
                                        record.push("Internal".into());
                                        record.push(code.to_string());
                                    }
                                }
                            } else {
                                record.push("Unknown".into());
                            }
                        }
                    }
                    clipx_measurement::error::ErrorKind::Wire(code) => {
                        record.push("Wire".into());
                        record.push(code.to_string());
                    }
                }
            }
        }
        writer.write_record(&record)?;
        Ok(())
    }
}

use crate::monitor::clipx_measurement::signals::SignalKind;

/// Maximum number of slaves that can be stored. This must be a power of 2 greater than 1.
const MAX_SLAVES: usize = 16;
/// Maximum PDU data payload size - set this to the max PDI size or higher.
const MAX_PDU_DATA: usize = PduStorage::element_size(1100);
/// Maximum number of EtherCAT frames that can be in flight at any one time.
const MAX_FRAMES: usize = 16;
/// Maximum total PDI length.
const PDI_LEN: usize = 64;

static PDU_STORAGE: PduStorage<MAX_FRAMES, MAX_PDU_DATA> = PduStorage::new();

#[derive(FromArgs)]
/// Monitor data that's coming out of an HBM ClipX amplifier.
struct Cli {
    /// the network interface for the EtherCAT communication
    #[argh(positional)]
    interface: String,
    /// the server's IP address from which monitoring data will be broadcast.
    #[argh(option, default = "\"127.0.0.2\".into()")]
    server_ip: String,
    /// the client's IP address to which monitoring data will be broadcast.
    #[argh(option, default = "\"localhost\".into()")]
    client_ip: String,
    /// the IP port through which monitoring data will be broadcast.
    #[argh(option, default = "49359")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli: Cli = argh::from_env();

    log::info!(
        "Starting {}",
        match std::env::current_exe() {
            Ok(exe) => exe.display().to_string(),
            Err(_) => "clipx-reader".into(),
        }
    );
    log::info!("Run with RUST_LOG=ethercrab=debug or =trace for debug information");

    // let socket = net::UdpSocket::bind(format!("{}:{}", cli.server_ip, cli.port)).await?;
    // socket
    //     .connect(format!("{}:{}", cli.client_ip, cli.port))
    //     .await?;

    let (tx, rx, pdu_loop) = PDU_STORAGE.try_split().expect("can only split once");

    let client = Arc::new(MainDevice::new(
        pdu_loop,
        Timeouts {
            wait_loop_delay: Duration::from_millis(2),
            mailbox_response: Duration::from_millis(1000),
            mailbox_echo: Duration::from_millis(200), // needed to be increased from 100
            ..Default::default()
        },
        MainDeviceConfig::default(),
    ));

    tokio::spawn(tx_rx_task(&cli.interface, tx, rx).expect("spawn TX/RX task"));

    let mut group = client
        .init_single_group::<MAX_SLAVES, PDI_LEN>(ethercat_now)
        .await
        .expect("Init");

    log::info!("Discovered {} slaves", group.len());

    for subdevice in group.iter(&client) {
        if subdevice.name() == "ClipX" {
            // RxPDOs
            subdevice.sdo_write(0x1c12, 0, 0u8).await?;
            // Fieldbus value 1
            subdevice.sdo_write(0x1c12, 1, 0x1601u16).await?;
            // Fieldbus flags
            subdevice.sdo_write(0x1c12, 2, 0x1603u16).await?;
            // Control flags
            subdevice.sdo_write(0x1c12, 3, 0x1625u16).await?;
            subdevice.sdo_write(0x1c12, 0, 3u8).await?;

            // TxPDOs
            subdevice.sdo_write(0x1c13, 0, 0u8).await?;
            // System status
            subdevice.sdo_write(0x1c13, 1, 0x1a00u16).await?;
            // Measurement value status
            subdevice.sdo_write(0x1c13, 2, 0x1a20u16).await?;
            // Electrical value
            subdevice.sdo_write(0x1c13, 3, 0x1a02u16).await?;
            // Gross value
            subdevice.sdo_write(0x1c13, 4, 0x1a03u16).await?;
            // Net value
            subdevice.sdo_write(0x1c13, 5, 0x1a04u16).await?;
            subdevice.sdo_write(0x1c13, 0, 5u8).await?;

            // set sensor type to Full bridge 2.5 mv/V (CF)
            subdevice.sdo_write(0x4400, 1, 8u8).await?;
        }
    }

    let mut group = group.into_op(&client).await.expect("PRE-OP -> OP");

    let mut tick_interval = time::interval(Duration::from_millis(10));
    tick_interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    let shutdown = Arc::new(std::sync::atomic::AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&shutdown))
        .expect("Register hook");

    let start = time::Instant::now();
    // let mut buffer = vec![];
    loop {
        // graceful shutdown on ^C
        if shutdown.load(std::sync::atomic::Ordering::Relaxed) {
            log::info!("Shutting down...");
            break;
        }
        // shutdown on EtherCAT disconnect
        if let Err(err) = group.tx_rx(&client).await {
            log::error!("Tx/Rx error: {}", err);
            break;
        };

        if let Some(clipx) = group.iter(&client).find(|slave| slave.name() == "ClipX") {
            let (i, _o) = clipx.io_raw();
            if let Err(err) = monitor::to_csv(&clipx::get_measurement(
                start.elapsed(),
                i,
                [SignalKind::Electrical, SignalKind::Gross, SignalKind::Net],
            )) {
                println!("CSV_ERROR,{:?}", err);
            };
            // ).encode(&mut buffer)?;
            // match socket.send(&buffer).await {
            //     // ignore if there isn't a client listening yet
            //     Err(io_error) if io_error.kind() == std::io::ErrorKind::ConnectionRefused => Ok(0),
            //     other => other,
            // }?;
        }

        tick_interval.tick().await;
    }

    let group = group.into_safe_op(&client).await.expect("OP -> SAFE-OP");
    let group = group.into_pre_op(&client).await.expect("SAFE-OP -> PRE-OP");
    let _group = group.into_init(&client).await.expect("PRE-OP -> INIT");

    Ok(())
}
