use ethercrab_wire::EtherCrabWireRead;

#[derive(Debug, EtherCrabWireRead)]
#[wire(bits=32)]
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
    ppmp_error: bool,
    // reserved 4 bits
    #[wire(pre_skip = 4, bits = 1)]
    reading_teds: bool,
    #[wire(bits = 1)]
    error_teds: bool,
    #[wire(bits = 1)]
    heartbeat: bool,
}
    
#[derive(Debug, EtherCrabWireRead)]
#[wire(bits=32)]
pub struct MeasurementStatus {
    #[wire(pre_skip = 2, bits = 1)]
    electrical_value: bool,
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
    clipx_bus1: bool,
    #[wire(bits = 1)]
    clipx_bus2: bool,
    #[wire(bits = 1)]
    clipx_bus3: bool,
    #[wire(bits = 1)]
    clipx_bus4: bool,
    #[wire(bits = 1)]
    clipx_bus5: bool,
    #[wire(bits = 1)]
    clipx_bus6: bool,
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
