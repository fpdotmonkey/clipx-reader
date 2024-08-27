// protobuf generated
include!(concat!(env!("OUT_DIR"), "/monitor.rs"));

// this is all glue code to go between the protobuf and Clipx serializations

use crate::clipx;

fn from_device_errors(errors: Vec<clipx::DeviceError>) -> clipx_measurement::ClipxError {
    use clipx_measurement::clipx_error::{single::ErrorKind, DeviceError, InternalError};
    clipx_measurement::ClipxError {
        error: errors
            .iter()
            .map(|error| match error {
                clipx::DeviceError::NotReady => {
                    ErrorKind::DeviceError(DeviceError::NotReady.into())
                }
                clipx::DeviceError::ParameterSet => {
                    ErrorKind::DeviceError(DeviceError::ParameterSet.into())
                }
                clipx::DeviceError::Internal(error) => ErrorKind::InternalError(match error {
                    clipx::InternalError::FileSystem => InternalError::FileSystem.into(),
                    clipx::InternalError::AdcCommunication => {
                        InternalError::AdcCommunication.into()
                    }
                    clipx::InternalError::AdcIrq => InternalError::AdcIrq.into(),
                    clipx::InternalError::AdcFrozen => InternalError::AdcFrozen.into(),
                    clipx::InternalError::AdcDma => InternalError::AdcDma.into(),
                    clipx::InternalError::DacCommunication => {
                        InternalError::DacCommunication.into()
                    }
                    clipx::InternalError::ExternalRam => InternalError::ExternalRam.into(),
                    clipx::InternalError::FieldbusController => {
                        InternalError::FieldbusController.into()
                    }
                    clipx::InternalError::FactoryCalibration => {
                        InternalError::FactoryCalibration.into()
                    }
                }),
                clipx::DeviceError::DacAlarm => {
                    ErrorKind::DeviceError(DeviceError::DacAlarm.into())
                }
                clipx::DeviceError::OneWireCommunication => {
                    ErrorKind::DeviceError(DeviceError::OneWireCommunication.into())
                }
                clipx::DeviceError::ClipxBus => {
                    ErrorKind::DeviceError(DeviceError::ClipxBus.into())
                }
                clipx::DeviceError::SensorExcitation => {
                    ErrorKind::DeviceError(DeviceError::SensorExcitation.into())
                }
                clipx::DeviceError::TestSignalActive => {
                    ErrorKind::DeviceError(DeviceError::TestSignalActive.into())
                }
                clipx::DeviceError::Ppmp => ErrorKind::DeviceError(DeviceError::Ppmp.into()),
                clipx::DeviceError::Teds => ErrorKind::DeviceError(DeviceError::Teds.into()),
                clipx::DeviceError::NoHeartbeat => {
                    ErrorKind::DeviceError(DeviceError::NoHeartbeat.into())
                }
            })
            .map(|error| clipx_measurement::clipx_error::Single {
                error_kind: Some(error),
            })
            .collect(),
    }
}

pub fn from_measurement(
    time_millis: u64,
    measurement: Result<Vec<(clipx::Signal, Option<f32>)>, clipx::GetMeasurementError>,
) -> ClipxMeasurement {
    ClipxMeasurement {
        time_millis,
        result: Some(match measurement {
            Ok(signals) => clipx_measurement::Result::Signals(clipx_measurement::Signals {
                gross: *signals
                    .iter()
                    .find(|(kind, _)| kind == &clipx::Signal::Gross)
                    .map(|(_, value)| value)
                    .unwrap_or(&None),
                net: *signals
                    .iter()
                    .find(|(kind, _)| kind == &clipx::Signal::Net)
                    .map(|(_, value)| value)
                    .unwrap_or(&None),
            }),
            Err(err) => clipx_measurement::Result::Error(clipx_measurement::Error {
                error_kind: Some(match err {
                    clipx::GetMeasurementError::Device(errors) => {
                        clipx_measurement::error::ErrorKind::ClipxError(from_device_errors(errors))
                    }
                    clipx::GetMeasurementError::Wire(error) => {
                        clipx_measurement::error::ErrorKind::WireError(error as u32)
                    }
                }),
            }),
        }),
    }
}
