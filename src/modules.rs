use crate::types::*;
use am2320;
use gpio::{sysfs::SysFsGpioInput, GpioIn};
use linux_embedded_hal::{Delay, I2cdev};

pub struct ModuleStatsGetter {
    pub temperature_humidity_module: Box<dyn ITemperatureHumidityModule + Sync + Send>,
    pub smoke_alarm_module: Box<dyn IPinReader + Sync + Send>,
    pub motion_detect_module: Box<dyn IPinReader + Sync + Send>,
    last_stats: Stats,
}

impl ModuleStatsGetter {
    pub fn new<
        T: ITemperatureHumidityModule + Sync + Send + 'static,
        S: IPinReader + Sync + Send + 'static,
        M: IPinReader + Sync + Send + 'static,
    >(
        temp_mod: T,
        smoke_mod: S,
        motion_mod: M,
    ) -> Self {
        ModuleStatsGetter {
            temperature_humidity_module: Box::new(temp_mod),
            smoke_alarm_module: Box::new(smoke_mod),
            motion_detect_module: Box::new(motion_mod),
            last_stats: Stats::default(),
        }
    }
}

impl IStatsGetter for ModuleStatsGetter {
    fn update_stats(&mut self) -> MyResult<()> {
        let (temperature, humidity) = self
            .temperature_humidity_module
            .get_temperature_humidity()?;
        let smoke_alarm = self.smoke_alarm_module.get_triggered()?;
        let motion_detected = self.motion_detect_module.get_triggered()?;
        self.last_stats.temperature.update(temperature);
        self.last_stats.humidity.update(humidity);
        self.last_stats.smoke_alarm.update(smoke_alarm);
        self.last_stats.motion_detect.update(motion_detected);
        Ok(())
    }

    fn get_stats(&self) -> MyResult<Stats> {
        Ok(self.last_stats.clone())
    }
}
pub type TGetter = dyn IStatsGetter + Sync + Send;

pub struct DebugTemperatureHumidityModule {}

impl ITemperatureHumidityModule for DebugTemperatureHumidityModule {
    fn get_temperature_humidity(&mut self) -> MyResult<(f32, f32)> {
        Ok((0.0, 0.0))
    }
}

pub struct DebugPinModule {}

impl IPinReader for DebugPinModule {
    fn get_triggered(&mut self) -> MyResult<bool> {
        Ok(false)
    }
}

pub struct AM2320Module {
    module: am2320::Am2320<I2cdev, Delay>,
}

impl AM2320Module {
    pub fn new(i2c_address: &str) -> MyResult<Self> {
        Ok(AM2320Module {
            module: am2320::Am2320::new(
                I2cdev::new(i2c_address)
                    .map_err(|_| format!("Failed to connect to i2c address {}", i2c_address))?,
                Delay {},
            ),
        })
    }
}

impl ITemperatureHumidityModule for AM2320Module {
    fn get_temperature_humidity(&mut self) -> MyResult<(f32, f32)> {
        let data = self
            .module
            .read()
            .map_err(|_| "Failed to read AM2320 data".to_string())?;
        Ok((data.temperature, data.humidity))
    }
}

pub struct SysfsPinReader {
    pin_number: u16,
    pin: SysFsGpioInput,
}
impl SysfsPinReader {
    pub fn new(pin: u16) -> MyResult<Self> {
        Ok(SysfsPinReader {
            pin_number: pin,
            pin: gpio::sysfs::SysFsGpioInput::open(pin)
                .map_err(|_| format!("Failed to connect to pin {}", pin))?,
        })
    }
}

impl IPinReader for SysfsPinReader {
    fn get_triggered(&mut self) -> MyResult<bool> {
        Ok(self
            .pin
            .read_value()
            .map_err(|_| format!("Failed to read pin {}", self.pin_number))?
            == gpio::GpioValue::High)
    }
}
