use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use esp_idf_sys::{esp_wifi_sta_get_ap_info, wifi_ap_record_t, EspError, ESP_OK};

use micro_rdk::DoCommand;
use micro_rdk::common::{
    config::ConfigType,
    registry::{ComponentRegistry, Dependency, RegistryError},
    sensor::{
        GenericReadingsResult, Readings, Sensor, SensorResult, SensorT, SensorType, TypedReadingsResult,
    },
    status::Status,
};

#[derive(DoCommand)]
pub struct WifiRSSISensor;

pub(crate) fn register_model(
    registry: &mut ComponentRegistry,
) -> anyhow::Result<(), RegistryError> {
    registry.register_sensor("wifi-rssi", &WifiRSSISensor::from_config)?;
    log::debug!("wifi-rssi sensor registration ok");
    Ok(())
}

impl WifiRSSISensor {
    pub fn from_config(_cfg: ConfigType, _deps: Vec<Dependency>) -> anyhow::Result<SensorType> {
        log::debug!("wifi-rssi sensor instantiated from config");
        Ok(Arc::new(Mutex::new(Self {})))
    }
}

impl Sensor for WifiRSSISensor {}

impl Readings for WifiRSSISensor {
    fn get_generic_readings(&self) -> anyhow::Result<GenericReadingsResult> {
        Ok(self
            .get_readings()?
            .into_iter()
            .map(|v| (v.0, SensorResult::<f64> { value: v.1 }.into()))
            .collect())
    }
}

impl SensorT<f64> for WifiRSSISensor {
    fn get_readings(&self) -> anyhow::Result<TypedReadingsResult<f64>> {
        log::debug!("wifi-rssi sensor - get readings called");
        let mut ap_info = wifi_ap_record_t::default();
        unsafe {
            match esp_wifi_sta_get_ap_info(&mut ap_info as *mut esp_idf_sys::wifi_ap_record_t) {
                ESP_OK => {}
                err => return Err(EspError::from(err).unwrap().into()),
            }
        };
        let mut x = HashMap::new();
        x.insert("rssi".to_string(), ap_info.rssi as f64);
        log::debug!("wifi-rssi sensor - get readings OK");
        Ok(x)
    }
}

impl Status for WifiRSSISensor {
    fn get_status(&self) -> anyhow::Result<Option<micro_rdk::google::protobuf::Struct>> {
        log::debug!("wifi-rssi sensor - get status called");
        Ok(Some(micro_rdk::google::protobuf::Struct {
            fields: HashMap::new(),
        }))
    }
}
