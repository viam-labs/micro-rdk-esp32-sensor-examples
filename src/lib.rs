use micro_rdk::common::registry::{ComponentRegistry, RegistryError};

pub mod free_heap_sensor;
pub mod wifi_rssi_sensor;

pub fn register_models(registry: &mut ComponentRegistry) -> anyhow::Result<(), RegistryError> {
    wifi_rssi_sensor::register_model(registry)?;
    free_heap_sensor::register_model(registry)?;
    Ok(())
}
