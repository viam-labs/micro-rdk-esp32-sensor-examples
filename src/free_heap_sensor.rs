use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use esp_idf_sys::esp_get_free_heap_size;

use micro_rdk::common::{
    config::ConfigType,
    registry::{ComponentRegistry, Dependency, RegistryError},
    sensor::{
        GenericReadingsResult, Sensor, SensorResult, SensorT, SensorType, TypedReadingsResult,
    },
    status::Status,
};

//use micro_rdk::google::protobuf;

pub struct FreeHeapSensor;

pub fn register_model(registry: &mut ComponentRegistry) -> anyhow::Result<(), RegistryError> {
    registry.register_sensor("free-heap", &FreeHeapSensor::from_config)?;
    log::info!("free-heap sensor registration ok");
    Ok(())
}

impl FreeHeapSensor {
    pub fn from_config(_cfg: ConfigType, _deps: Vec<Dependency>) -> anyhow::Result<SensorType> {
        log::info!("free-heap sensor instantiated from config");
        Ok(Arc::new(Mutex::new(Self{})))
    }
}

impl Sensor for FreeHeapSensor {
    fn get_generic_readings(&self) -> anyhow::Result<GenericReadingsResult> {
        Ok(self
            .get_readings()?
            .into_iter()
            .map(|v| (v.0, SensorResult::<f64> { value: v.1 }.into()))
            .collect())
    }
}

impl SensorT<f64> for FreeHeapSensor {
    fn get_readings(&self) -> anyhow::Result<TypedReadingsResult<f64>> {
        log::info!("free-heap sensor - get readings called");
        let reading = unsafe {
            esp_get_free_heap_size()
        };
        let mut x = HashMap::new();
        x.insert("bytes".to_string(), reading as f64);
        log::info!("free-heap sensor - get readings OK");
        Ok(x)
    }
}

impl Status for FreeHeapSensor {
    fn get_status(&self) -> anyhow::Result<Option<micro_rdk::google::protobuf::Struct>> {
        log::info!("free-heap sensor - get status called");
        Ok(Some(micro_rdk::google::protobuf::Struct {
            fields: HashMap::new(),
        }))
    }
}
