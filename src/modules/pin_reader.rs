use crate::modules::common::*;

use gpio::{sysfs::SysFsGpioInput, GpioIn};

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
        Box::new(BoolStat::new(self.get_key()))
    }
}
