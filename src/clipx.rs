use ethercrab_wire::{EtherCrabWireRead, WireError};
use tokio::time;

use crate::monitor::clipx_measurement;

#[derive(Debug, EtherCrabWireRead)]
#[wire(bits = 32)]
pub struct ClipxStatus {
    #[wire(bits = 1)]
    device_ready: bool,
    #[wire(bits = 1)]
    sync_master: bool,
    #[wire(bits = 1)]
    sync_subdevice: bool,
    #[wire(bits = 1)]
    sync_slave_no_sync_in: bool,
    #[wire(bits = 1)]
    change_parameter_set: bool,
    #[wire(bits = 1)]
    error_parameter_set: bool,
    #[wire(bits = 1)]
    error_file_system: bool,
    #[wire(bits = 1)]
    error_adc_communication: bool,
    #[wire(bits = 1)]
    error_adc_irq: bool,
    #[wire(bits = 1)]
    error_adc_frozen: bool,
    #[wire(bits = 1)]
    error_adc_dma: bool,
    #[wire(bits = 1)]
    error_dac_communication: bool,
    #[wire(bits = 1)]
    dac_alarm: bool,
    #[wire(bits = 1)]
    error_one_wire_communication: bool,
    #[wire(bits = 1)]
    error_clipx_bus: bool,
    #[wire(bits = 1)]
    error_external_ram: bool,
    #[wire(bits = 1)]
    error_sensor_excitation: bool,
    #[wire(bits = 1)]
    fieldbus_io: bool,
    #[wire(bits = 1)]
    error_fieldbus_controller: bool,
    #[wire(bits = 1)]
    error_factory_calibration: bool,
    #[wire(bits = 1)]
    test_signal_active: bool,
    #[wire(bits = 1)]
    ethernet_connection_established: bool,
    // reserved 1 bit
    #[wire(pre_skip = 1, bits = 1)]
    ppmp_connected: bool,
    #[wire(bits = 1)]
    error_ppmp: bool,
    // reserved 4 bits
    #[wire(pre_skip = 4, bits = 1)]
    reading_teds: bool,
    #[wire(bits = 1)]
    error_teds: bool,
    #[wire(bits = 1)]
    heartbeat: bool,
}

/// The status of each measurement signal.  `true` if invalid.
#[derive(Debug, EtherCrabWireRead)]
#[wire(bits = 32)]
pub struct MeasurementStatus {
    #[wire(pre_skip = 2, bits = 1)]
    electrical: bool,
    #[wire(bits = 1)]
    gross: bool,
    #[wire(bits = 1)]
    net: bool,
    #[wire(bits = 1)]
    minimum: bool,
    #[wire(bits = 1)]
    maximum: bool,
    #[wire(bits = 1)]
    peak_to_peak: bool,
    #[wire(bits = 1)]
    captured1: bool,
    #[wire(bits = 1)]
    captured2: bool,
    #[wire(bits = 1)]
    bus1: bool,
    #[wire(bits = 1)]
    bus2: bool,
    #[wire(bits = 1)]
    bus3: bool,
    #[wire(bits = 1)]
    bus4: bool,
    #[wire(bits = 1)]
    bus5: bool,
    #[wire(bits = 1)]
    bus6: bool,
    #[wire(pre_skip = 5, bits = 1)]
    calculated1: bool,
    #[wire(bits = 1)]
    calculated2: bool,
    #[wire(bits = 1)]
    calculated3: bool,
    #[wire(bits = 1)]
    calculated4: bool,
    #[wire(bits = 1)]
    calculated5: bool,
    #[wire(bits = 1)]
    calculated6: bool,
    #[wire(bits = 1)]
    external_ethernet1: bool,
    #[wire(bits = 1)]
    external_ethernet2: bool,
    #[wire(bits = 1)]
    external_fieldbus1: bool,
    #[wire(bits = 1)]
    external_fieldbus2: bool,
    #[wire(bits = 1)]
    analog_output: bool,
}

impl From<ClipxStatus> for clipx_measurement::error::Clipx {
    fn from(clipx_status: ClipxStatus) -> Self {
        use clipx_measurement::error::clipx::{fault, Fault};

        let mut errors = vec![];
        if clipx_status.error_file_system {
            errors.push(Fault {
                kind: Some(fault::Kind::Internal(fault::Internal::FileSystem.into())),
            });
        }
        if clipx_status.error_adc_communication {
            errors.push(Fault {
                kind: Some(fault::Kind::Internal(
                    fault::Internal::AdcCommunication.into(),
                )),
            });
        }
        if clipx_status.error_adc_irq {
            errors.push(Fault {
                kind: Some(fault::Kind::Internal(fault::Internal::AdcIrq.into())),
            });
        }
        if clipx_status.error_adc_frozen {
            errors.push(Fault {
                kind: Some(fault::Kind::Internal(fault::Internal::AdcFrozen.into())),
            });
        }
        if clipx_status.error_adc_dma {
            errors.push(Fault {
                kind: Some(fault::Kind::Internal(fault::Internal::AdcDma.into())),
            });
        }
        if clipx_status.error_dac_communication {
            errors.push(Fault {
                kind: Some(fault::Kind::Internal(
                    fault::Internal::DacCommunication.into(),
                )),
            });
        }
        if clipx_status.error_external_ram {
            errors.push(Fault {
                kind: Some(fault::Kind::Internal(fault::Internal::ExternalRam.into())),
            });
        }
        if clipx_status.error_fieldbus_controller {
            errors.push(Fault {
                kind: Some(fault::Kind::Internal(
                    fault::Internal::FieldbusController.into(),
                )),
            });
        }
        if clipx_status.error_factory_calibration {
            errors.push(Fault {
                kind: Some(fault::Kind::Internal(
                    fault::Internal::FactoryCalibration.into(),
                )),
            });
        }
        if !clipx_status.device_ready {
            errors.push(Fault {
                kind: Some(fault::Kind::Device(fault::Device::NotReady.into())),
            });
        }
        if clipx_status.error_parameter_set {
            errors.push(Fault {
                kind: Some(fault::Kind::Device(fault::Device::ParameterSet.into())),
            });
        }
        if clipx_status.dac_alarm {
            errors.push(Fault {
                kind: Some(fault::Kind::Device(fault::Device::DacAlarm.into())),
            });
        }
        if clipx_status.error_one_wire_communication {
            errors.push(Fault {
                kind: Some(fault::Kind::Device(
                    fault::Device::OneWireCommunication.into(),
                )),
            });
        }
        if clipx_status.error_clipx_bus {
            errors.push(Fault {
                kind: Some(fault::Kind::Device(fault::Device::ClipxBus.into())),
            });
        }
        if clipx_status.error_sensor_excitation {
            errors.push(Fault {
                kind: Some(fault::Kind::Device(fault::Device::SensorExcitation.into())),
            });
        }
        if clipx_status.test_signal_active {
            errors.push(Fault {
                kind: Some(fault::Kind::Device(fault::Device::TestSignalActive.into())),
            });
        }
        if clipx_status.error_ppmp {
            errors.push(Fault {
                kind: Some(fault::Kind::Device(fault::Device::Ppmp.into())),
            });
        }
        if clipx_status.error_teds {
            errors.push(Fault {
                kind: Some(fault::Kind::Device(fault::Device::Teds.into())),
            });
        }
        Self { error: errors }
    }
}

/// Get measurement data out of the input buffer.
///
/// This assumes that the first 32 bits out of the buffer are the System
/// Status (SDO `0x4200:1`), the 32 following are the Measurement Status
/// (SDO `0x44f4:1`), and that the rest are [`f32`] signals in the order
/// given in the `signal_types` arg.
///
/// This will return `Err` if the System Status indicates an error.  The
/// individual values in `Ok` will be `None` if that signal's
/// measurement status is invalid.
///
/// After [`u64::MAX`] milliseconds, the time will wrap back to 0.
pub fn get_measurement<const N: usize>(
    time: time::Duration,
    i: &[u8],
    signal_types: [clipx_measurement::signals::SignalKind; N],
) -> crate::monitor::ClipxMeasurement {
    use clipx_measurement::signals::SignalKind;

    let (clipx_status, measurement_status, signals) =
        match <(ClipxStatus, MeasurementStatus, [f32; N])>::unpack_from_slice(i) {
            Ok((clipx_status, measurement_status, signals)) => {
                (clipx_status, measurement_status, signals)
            }
            Err(err) => {
                return crate::monitor::ClipxMeasurement {
                    time_millis: (time.as_millis() % (u64::MAX as u128 + 1))
                        .try_into()
                        .unwrap(),
                    result: Some(clipx_measurement::Result::Err(clipx_measurement::Error {
                        error_kind: Some(clipx_measurement::error::ErrorKind::Wire(err as u32)),
                    })),
                };
            }
        };
    let maybe_errors: clipx_measurement::error::Clipx = clipx_status.into();
    if !maybe_errors.error.is_empty() {
        return crate::monitor::ClipxMeasurement {
            time_millis: (time.as_millis() % (u64::MAX as u128 + 1))
                .try_into()
                .unwrap(),
            result: Some(clipx_measurement::Result::Err(clipx_measurement::Error {
                error_kind: Some(clipx_measurement::error::ErrorKind::Clipx(maybe_errors)),
            })),
        };
    }

    crate::monitor::ClipxMeasurement {
        time_millis: time.as_millis().try_into().unwrap_or(u64::MAX),
        result: Some(clipx_measurement::Result::Ok(clipx_measurement::Signals {
            signal: signal_types
                .into_iter()
                .zip(signals.iter())
                .map(|(signal_type, signal)| {
                    let signal_is_valid = !match signal_type {
                        SignalKind::Electrical => measurement_status.electrical,
                        SignalKind::Gross => measurement_status.gross,
                        SignalKind::Net => measurement_status.net,
                        SignalKind::Minimum => measurement_status.minimum,
                        SignalKind::Maximum => measurement_status.maximum,
                        SignalKind::PeakToPeak => measurement_status.peak_to_peak,
                        SignalKind::Captured1 => measurement_status.captured1,
                        SignalKind::Captured2 => measurement_status.captured2,
                        SignalKind::Bus1 => measurement_status.bus1,
                        SignalKind::Bus2 => measurement_status.bus2,
                        SignalKind::Bus3 => measurement_status.bus3,
                        SignalKind::Bus4 => measurement_status.bus4,
                        SignalKind::Bus5 => measurement_status.bus5,
                        SignalKind::Bus6 => measurement_status.bus6,
                        SignalKind::Calculated1 => measurement_status.calculated1,
                        SignalKind::Calculated2 => measurement_status.calculated2,
                        SignalKind::Calculated3 => measurement_status.calculated3,
                        SignalKind::Calculated4 => measurement_status.calculated4,
                        SignalKind::Calculated5 => measurement_status.calculated5,
                        SignalKind::Calculated6 => measurement_status.calculated6,
                        SignalKind::ExternalEthernet1 => measurement_status.external_ethernet1,
                        SignalKind::ExternalEthernet2 => measurement_status.external_ethernet2,
                        SignalKind::ExternalFieldbus1 => measurement_status.external_fieldbus1,
                        SignalKind::ExternalFieldbus2 => measurement_status.external_fieldbus2,
                        SignalKind::AnalogOutput => measurement_status.analog_output,
                    };
                    if signal_is_valid {
                        (signal_type, Some(signal).cloned())
                    } else {
                        (signal_type, None)
                    }
                })
                .map(|(kind, value)| clipx_measurement::signals::Signal {
                    r#type: kind.into(),
                    value,
                })
                .collect(),
        })),
    }
}
