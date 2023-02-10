use std::fmt::Display;

use crate::types::*;
use am2320;
use linux_embedded_hal::{Delay, I2cdev};
use std::sync::RwLock;

pub struct ModuleStatsGetter<T, H> {
    pub temperature_humidity_module: Box<dyn ITemperatureHumidityModule<T, H> + Sync + Send>,
}

impl<T: Display, H: Display> IStatsGetter<T, H> for ModuleStatsGetter<T, H> {
    fn get_stats(&self) -> Stats<T, H> {
        let (temperature, humidity) = self.temperature_humidity_module.get_temperature_humidity();
        Stats {
            temperature,
            humidity,
        }
    }
}

pub type THumidity = u32;
pub type TTemperature = f32;
pub type TGetter = dyn IStatsGetter<TTemperature, THumidity> + Sync + Send;

pub struct DebugTemperatureHumidityModule {}

impl ITemperatureHumidityModule<TTemperature, THumidity> for DebugTemperatureHumidityModule {
    fn get_temperature_humidity(&self) -> (TTemperature, THumidity) {
        (TTemperature::default(), THumidity::default())
    }
}

pub struct AM2320Module {
    module: RwLock<am2320::Am2320<I2cdev, Delay>>,
}

impl AM2320Module {
    pub fn new(i2c_address: &str) -> Self {
        AM2320Module {
            module: RwLock::new(am2320::Am2320::new(I2cdev::new(i2c_address).expect("Failed to connect to i2c address"), Delay {})),
        }
    }
}

impl ITemperatureHumidityModule<TTemperature, THumidity> for AM2320Module {
    fn get_temperature_humidity(&self) -> (TTemperature, THumidity) {
        let data = self
            .module
            .write()
            .unwrap()
            .read()
            .unwrap_or(am2320::Measurement {
                temperature: -173.0,
                humidity: 0.0,
            });
        (data.temperature, data.humidity as u32)
    }
}
