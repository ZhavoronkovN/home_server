use crate::modules::common::*;
use am2320;
use linux_embedded_hal::{Delay, I2cdev};

pub enum AM2320Usage {
    Temperature,
    Humidity,
}
pub struct AM2320Module {
    module: am2320::Am2320<I2cdev, Delay>,
    used_for: AM2320Usage,
}

impl AM2320Module {
    pub fn new(i2c_address: &str, used_for: AM2320Usage) -> MyResult<Self> {
        Ok(AM2320Module {
            used_for,
            module: am2320::Am2320::new(
                I2cdev::new(i2c_address)
                    .map_err(|_| format!("Failed to connect to i2c address {}", i2c_address))?,
                Delay {},
            ),
        })
    }
}

impl IModule for AM2320Module {
    fn get_measurement(&mut self) -> MyResult<StatType> {
        let data = self
            .module
            .read()
            .map_err(|_| "Failed to read AM2320 data".to_string())?;
        Ok(StatType::Numeric(match self.used_for {
            AM2320Usage::Temperature => data.temperature,
            AM2320Usage::Humidity => data.humidity,
        }))
    }

    fn get_key(&self) -> String {
        match self.used_for {
            AM2320Usage::Temperature => "temperature",
            AM2320Usage::Humidity => "humidity",
        }
        .to_string()
    }

    fn get_stat_item(&self) -> Box<dyn StatItem> {
        Box::new(NumericStat::new(self.get_key()))
    }
}
