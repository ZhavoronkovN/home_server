use crate::types::*;
use am2320;
use gpio::{sysfs::SysFsGpioInput, GpioIn};
use linux_embedded_hal::{Delay, I2cdev};

pub trait IModule {
    fn get_measurement(&mut self) -> MyResult<StatType>;
    fn get_key(&self) -> String;
    fn get_stat_item(&self) -> Box<dyn StatItem>;
}

pub struct ModuleStatsGetter {
    modules: Vec<Box<dyn IModule + Sync + Send>>,
    last_stats: Stats,
}

impl ModuleStatsGetter {
    pub fn new() -> Self {
        ModuleStatsGetter {
            modules: Vec::new(),
            last_stats: Stats::default(),
        }
    }

    pub fn add_module<M: IModule + Sync + Send + 'static>(&mut self, module: M) {
        self.last_stats.add_stat_item(module.get_stat_item());
        self.modules.push(Box::new(module));
    }
}

impl IStatsGetter for ModuleStatsGetter {
    fn update_stats(&mut self) -> MyResult<()> {
        for m in &mut self.modules {
            self.last_stats
                .update_stat_item(&m.get_key(), m.get_measurement()?)?
        }
        Ok(())
    }

    fn get_stats(&self) -> MyResult<Stats> {
        Ok(self.last_stats.clone())
    }
}
pub type TGetter = dyn IStatsGetter + Sync + Send;

pub struct DebugTemperatureModule {}

impl IModule for DebugTemperatureModule {
    fn get_measurement(&mut self) -> MyResult<StatType> {
        Ok(StatType::Numeric(0.0))
    }

    fn get_key(&self) -> String {
        "debug_temperature".to_string()
    }

    fn get_stat_item(&self) -> Box<dyn StatItem> {
        Box::new(NumericStat::default())
    }
}

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
        Box::new(NumericStat::default())
    }
}

pub struct SysfsPinReader {
    pin_number: u16,
    pin: SysFsGpioInput,
    name: String,
}

impl SysfsPinReader {
    pub fn new(pin: u16, name: String) -> MyResult<Self> {
        Ok(SysfsPinReader {
            name,
            pin_number: pin,
            pin: gpio::sysfs::SysFsGpioInput::open(pin)
                .map_err(|_| format!("Failed to connect to pin {}", pin))?,
        })
    }
}

impl IModule for SysfsPinReader {
    fn get_measurement(&mut self) -> MyResult<StatType> {
        Ok(StatType::Bool(
            self.pin
                .read_value()
                .map_err(|_| format!("Failed to read pin {}", self.pin_number))?
                == gpio::GpioValue::High,
        ))
    }

    fn get_key(&self) -> String {
        self.name.clone()
    }

    fn get_stat_item(&self) -> Box<dyn StatItem> {
        Box::new(BoolStat::default())
    }
}
