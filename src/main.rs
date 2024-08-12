use std::{sync::Arc, time::Duration};

use env_logger::Env;
use ethercrab::{
    error::Error,
    std::{ethercat_now, tx_rx_task},
    MainDevice, MainDeviceConfig, PduStorage, Timeouts,
};
use ethercrab::EtherCrabWireRead;
use tokio::time::MissedTickBehavior;

mod types;

/// Maximum number of slaves that can be stored. This must be a power of 2 greater than 1.
const MAX_SLAVES: usize = 16;
/// Maximum PDU data payload size - set this to the max PDI size or higher.
const MAX_PDU_DATA: usize = PduStorage::element_size(1100);
/// Maximum number of EtherCAT frames that can be in flight at any one time.
const MAX_FRAMES: usize = 16;
/// Maximum total PDI length.
const PDI_LEN: usize = 64;

static PDU_STORAGE: PduStorage<MAX_FRAMES, MAX_PDU_DATA> = PduStorage::new();

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let interface: String = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => {
            eprintln!("provide a network device as the first argument");
            return Err(Error::Internal);
        }
    };

    log::info!("Starting EK1100/EK1501 demo...");
    log::info!(
        "Ensure an EK1100 or EK1501 is the first slave, with any number of modules connected after"
    );
    log::info!("Run with RUST_LOG=ethercrab=debug or =trace for debug information");

    let (tx, rx, pdu_loop) = PDU_STORAGE.try_split().expect("can only split once");

    let client = Arc::new(MainDevice::new(
        pdu_loop,
        Timeouts {
            wait_loop_delay: Duration::from_millis(2),
            mailbox_response: Duration::from_millis(1000),
	    mailbox_echo: Duration::from_millis(200),
            ..Default::default()
        },
        MainDeviceConfig::default(),
    ));

    tokio::spawn(tx_rx_task(&interface, tx, rx).expect("spawn TX/RX task"));

    let mut group = client
        .init_single_group::<MAX_SLAVES, PDI_LEN>(ethercat_now)
        .await
        .expect("Init");

    log::info!("Discovered {} slaves", group.len());

    // group.iter(&client).find(|subdevice| subdevice.name() == "ClipX").map(|subdevice| subdevice.

    for subdevice in group.iter(&client) {
        if subdevice.name() == "ClipX" {
            subdevice.sdo_write(0x1c12, 0, 0u8).await?;
            subdevice.sdo_write(0x1c12, 1, 0x1601u16).await?;
	    subdevice.sdo_write(0x1c12, 2, 0x1603u16).await?;
	    subdevice.sdo_write(0x1c12, 3, 0x1625u16).await.unwrap();
            subdevice.sdo_write(0x1c12, 0, 3u8).await?;

            subdevice.sdo_write(0x1c13, 0, 0u8).await?;
            subdevice.sdo_write(0x1c13, 1, 0x1a00u16).await?;
	    subdevice.sdo_write(0x1c13, 2, 0x1a03u16).await?;
	    subdevice.sdo_write(0x1c13, 3, 0x1a04u16).await?;
	    subdevice.sdo_write(0x1c13, 4, 0x1a20u16).await?;
            subdevice.sdo_write(0x1c13, 0, 4u8).await?;
        }
    }

    let mut group = group.into_op(&client).await.expect("PRE-OP -> OP");

    let mut tick_interval = tokio::time::interval(Duration::from_millis(10));
    tick_interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let shutdown = Arc::new(std::sync::atomic::AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&shutdown))
        .expect("Register hook");

    loop {
        // graceful shutdown on ^C
        if shutdown.load(std::sync::atomic::Ordering::Relaxed) {
            log::info!("Shutting down...");
            break;
        }
        group.tx_rx(&client).await.expect("TX/RX");

        if let Some(clipx) = group.iter(&client).find(|slave| slave.name() == "ClipX") {
            let (i, _o) = clipx.io_raw();
	    println!("{:?}", <(types::ClipxStatus, f32, f32, types::MeasurementStatus)>::unpack_from_slice(i));
        }

        tick_interval.tick().await;
    }

    let group = group.into_safe_op(&client).await.expect("OP -> SAFE-OP");
    let group = group.into_pre_op(&client).await.expect("SAFE-OP -> PRE-OP");
    let _group = group.into_init(&client).await.expect("PRE-OP -> INIT");

    Ok(())
}
