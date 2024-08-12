use ethercrab_wire::{EtherCrabWireRead, WireError};

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

/// An issue that would preclude reporting measurements.
pub enum DeviceError {
    /// The ClipX is switched not on, initialized, or the Ethernet
    /// connection is off.
    NotReady,
    /// Error in the current loaded parameter set. Load a different
    /// parameter set or check all settings and resave the parameter
    /// set. If the parameter set is stored on PC, you can also import
    /// it from there and check the stored version for errors.
    ParameterSet,
    /// The internal function of the device has failed.  Contact HBM.
    Internal(InternalError),
    /// No current can flow at the current output; there is a line break.
    DacAlarm,
    /// The 1-wire TEDS cannot be read. Check the wiring. If possible,
    /// check whether the TEDS module can be read on another device, or
    /// is defective.
    OneWireCommunication,
    /// The ClipX bus is not working correctly. Check the wiring of the
    /// bus system
    ClipxBus,
    /// The excitation voltage for the sensor has been short-circuited.
    /// Check the wiring of the sensor.
    SensorExcitation,
    /// The test signal is activated, no measured values are captured.
    TestSignalActive,
    /// There is an error in the PPMP connection; the system LED is lit
    /// yellow.
    Ppmp,
    /// The data in the TEDS module either contain errors or cannot be
    /// set.
    Teds,
    /// The ClipX hasn't reported as alive for more than 1.0 second.
    ///
    /// Possibly a disconnection or power failure.
    NoHeartbeat,
}

impl std::fmt::Debug for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
	    DeviceError::NotReady => write!(f, "NotReady: The ClipX is switched not on, initialized, or the Ethernet connection is off."),
	    DeviceError::ParameterSet => write!(f, "ParameterSet: Error in the current loaded parameter set. Load a different parameter set or check all settings and resave the parameter set. If the parameter set is stored on PC, you can also import it from there and check the stored version for errors."),
	    DeviceError::Internal(err) => write!(f, "ClipX Internal: {err:?}.  Contact HBM <support@hbkworld.com>."),
	    DeviceError::DacAlarm => write!(f, "DacAlarm: No current can flow at the current output; there is a line break."),
	    DeviceError::OneWireCommunication => write!(f, "OneWireCommunication: The 1-wire TEDS cannot be read. Check the wiring. If possible, check whether the TEDS module can be read on another device, or is defective."),
	    DeviceError::ClipxBus => write!(f, "ClipxBus: The ClipX bus is not working correctly. Check the wiring of the bus system"),
	    DeviceError::SensorExcitation => write!(f, "SensorExcitation: The excitation voltage for the sensor has been short-circuited. Check the wiring of the sensor."),
	    DeviceError::TestSignalActive => write!(f, "TestSignalActive: The test signal is activated, no measured values are captured."),
	    DeviceError::Ppmp => write!(f, "Ppmp: There is an error in the PPMP connection; the system LED is lit yellow."),
	    DeviceError::Teds => write!(f, "Teds: The data in the TEDS module either contain errors or cannot be set."),
	    DeviceError::NoHeartbeat => write!(f, "NoHeartbeat: The ClipX hasn't reported as alive for more than 1.0 second. Possibly a disconnection or power failure."),
	}
    }
}

impl std::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for DeviceError {}

/// ClipX errors related to internal hardware or software faults.
/// Contact HBM and share the error information if encountered.
///
/// Email: <support@hbkworld.com>
/// Phone: [Europe] +49 6151 803-0
///        [Americas] +1 (800) 578-4260
///                   +1 (508) 624-4500
///        [Asia] +86 512-68247776
pub enum InternalError {
    /// Error related to the internal file system.
    FileSystem,
    /// Error related to analog-digital converter communication.
    AdcCommunication,
    /// Error related to analog-digital converter interrupt request.
    AdcIrq,
    /// No change in the analog-digital converter state in 50 ms.
    AdcFrozen,
    /// Error related to analog-digital converter direct memory access.
    AdcDma,
    /// Error related to digital-analog converter communication.
    DacCommunication,
    /// Error in the RAM of the ClipX (not in the RAM of the CPU).
    ExternalRam,
    /// Internal error in the fieldbus controller (only on BM40IE and BM40PB).
    FieldbusController,
    /// There is an error in the calibration of the ClipX.
    FactoryCalibration,
}

impl std::fmt::Debug for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
	    InternalError::FileSystem => write!(f, "FileSystem: Error related to the internal file system."),
	    InternalError::AdcCommunication => write!(f, "AdcCommunication: Error related to analog-digital converter communication."),
	    InternalError::AdcIrq => write!(f, "AdcIrq: Error related to analog-digital converter interrupt request."),
	    InternalError::AdcFrozen => write!(f, "AdcFrozen: No change in the analog-digital converter state in 50 ms."),
	    InternalError::AdcDma => write!(f, "AdcDma: Error related to analog-digital converter direct memory access."),
	    InternalError::DacCommunication => write!(f, "DacCommunication: Error related to digital-analog converter communication."),
	    InternalError::ExternalRam => write!(f, "ExternalRam: Error in the RAM of the ClipX (not in the RAM of the CPU)."),
	    InternalError::FieldbusController => write!(f, "FieldbusController: Internal error in the fieldbus controller (only on BM40IE and BM40PB)."),
	    InternalError::FactoryCalibration => write!(f, "FactoryCalibration: There is an error in the calibration of the ClipX."),
	}
    }
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for InternalError {}

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

#[derive(Debug)]
pub enum Signal {
    /// Input signal in the unit of the measured variable, e.g. in mV/V.
    Electrical,
    /// Gross signal.
    Gross,
    /// Net signal.
    Net,
    /// Peak value minimum.
    Minimum,
    /// Peak value maximum
    Maximum,
    /// Peak-to-peak value.
    PeakToPeak,
    /// Captured value 1.
    Captured1,
    /// Captured value 2.
    Captured2,
    /// Value on the device's ClipX bus at address 1.
    Bus1,
    /// Value on the device's ClipX bus at address 2.
    Bus2,
    /// Value on the device's ClipX bus at address 3.
    Bus3,
    /// Value on the device's ClipX bus at address 4.
    Bus4,
    /// Value on the device's ClipX bus at address 5.
    Bus5,
    /// Value on the device's ClipX bus at address 6.
    Bus6,
    /// Value of the calculation channel 1.
    Calculated1,
    /// Value of the calculation channel 2.
    Calculated2,
    /// Value of the calculation channel 3.
    Calculated3,
    /// Value of the calculation channel 4.
    Calculated4,
    /// Value of the calculation channel 5.
    Calculated5,
    /// Value of the calculation channel 6.
    Calculated6,
    /// Value 1 transmitted via Ethernet.
    ExternalEthernet1,
    /// Value 2 transmitted via Ethernet.
    ExternalEthernet2,
    /// Value 1 transmitted via the fieldbus.
    ExternalFieldbus1,
    /// Value 2 transmitted via the fieldbus.
    ExternalFieldbus2,
    /// Value of the analog output in V or mA.
    AnalogOutput,
}

impl From<ClipxStatus> for Vec<DeviceError> {
    fn from(clipx_status: ClipxStatus) -> Self {
        let mut errors = vec![];
        if clipx_status.error_file_system {
            errors.push(DeviceError::Internal(InternalError::FileSystem));
        }
        if clipx_status.error_adc_communication {
            errors.push(DeviceError::Internal(InternalError::AdcCommunication));
        }
        if clipx_status.error_adc_irq {
            errors.push(DeviceError::Internal(InternalError::AdcIrq));
        }
        if clipx_status.error_adc_frozen {
            errors.push(DeviceError::Internal(InternalError::AdcFrozen));
        }
        if clipx_status.error_adc_dma {
            errors.push(DeviceError::Internal(InternalError::AdcDma));
        }
        if clipx_status.error_dac_communication {
            errors.push(DeviceError::Internal(InternalError::DacCommunication));
        }
        if clipx_status.error_external_ram {
            errors.push(DeviceError::Internal(InternalError::ExternalRam));
        }
        if clipx_status.error_fieldbus_controller {
            errors.push(DeviceError::Internal(InternalError::FieldbusController));
        }
        if clipx_status.error_factory_calibration {
            errors.push(DeviceError::Internal(InternalError::FactoryCalibration));
        }
        if !clipx_status.device_ready {
            errors.push(DeviceError::NotReady);
        }
        if clipx_status.error_parameter_set {
            errors.push(DeviceError::ParameterSet);
        }
        if clipx_status.dac_alarm {
            errors.push(DeviceError::DacAlarm);
        }
        if clipx_status.error_one_wire_communication {
            errors.push(DeviceError::OneWireCommunication);
        }
        if clipx_status.error_clipx_bus {
            errors.push(DeviceError::ClipxBus);
        }
        if clipx_status.error_sensor_excitation {
            errors.push(DeviceError::SensorExcitation);
        }
        if clipx_status.test_signal_active {
            errors.push(DeviceError::TestSignalActive);
        }
        if clipx_status.error_ppmp {
            errors.push(DeviceError::Ppmp);
        }
        if clipx_status.error_teds {
            errors.push(DeviceError::Teds);
        }
        errors
    }
}

#[derive(Debug)]
pub enum GetMeasurementError {
    Device(Vec<DeviceError>),
    Wire(WireError),
}

impl std::fmt::Display for GetMeasurementError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for GetMeasurementError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GetMeasurementError::Device(_) => None,
            GetMeasurementError::Wire(err) => Some(err),
        }
    }
}

/// Get measurement data out of the input buffer.
///
/// This assumes that the first 32 bits out of the buffer are the System
/// Status (`0x4200:1`), the 32 following are the Measurement Status
/// (`0x44f4:1`), and that the rest are [`f32`] signals in the order
/// given in the `signals` arg.
///
/// This will return `Err` if the System Status indicates an error.  The
/// individual values in `Ok` will be `None` if that signal's
/// measurement status is invalid.
pub fn get_measurement<const N: usize>(
    i: &[u8],
    signal_types: [Signal; N],
) -> Result<Vec<(Signal, Option<f32>)>, GetMeasurementError> {
    let (clipx_status, measurement_status, signals) =
        <(ClipxStatus, MeasurementStatus, [f32; N])>::unpack_from_slice(i)
            .map_err(GetMeasurementError::Wire)?;
    let maybe_errors: Vec<DeviceError> = clipx_status.into();
    if !maybe_errors.is_empty() {
        return Err(GetMeasurementError::Device(maybe_errors));
    }
    Ok(signal_types
        .into_iter()
        .zip(signals.iter())
        .map(|(signal_type, signal)| {
            let signal_is_valid = match signal_type {
                Signal::Electrical => measurement_status.electrical,
                Signal::Gross => measurement_status.gross,
                Signal::Net => measurement_status.net,
                Signal::Minimum => measurement_status.minimum,
                Signal::Maximum => measurement_status.maximum,
                Signal::PeakToPeak => measurement_status.peak_to_peak,
                Signal::Captured1 => measurement_status.captured1,
                Signal::Captured2 => measurement_status.captured2,
                Signal::Bus1 => measurement_status.bus1,
                Signal::Bus2 => measurement_status.bus2,
                Signal::Bus3 => measurement_status.bus3,
                Signal::Bus4 => measurement_status.bus4,
                Signal::Bus5 => measurement_status.bus5,
                Signal::Bus6 => measurement_status.bus6,
                Signal::Calculated1 => measurement_status.calculated1,
                Signal::Calculated2 => measurement_status.calculated2,
                Signal::Calculated3 => measurement_status.calculated3,
                Signal::Calculated4 => measurement_status.calculated4,
                Signal::Calculated5 => measurement_status.calculated5,
                Signal::Calculated6 => measurement_status.calculated6,
                Signal::ExternalEthernet1 => measurement_status.external_ethernet1,
                Signal::ExternalEthernet2 => measurement_status.external_ethernet2,
                Signal::ExternalFieldbus1 => measurement_status.external_fieldbus1,
                Signal::ExternalFieldbus2 => measurement_status.external_fieldbus2,
                Signal::AnalogOutput => measurement_status.analog_output,
            };
            if signal_is_valid {
                (signal_type, Some(signal).cloned())
            } else {
                (signal_type, None)
            }
        })
        .collect())
}
