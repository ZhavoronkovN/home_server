use crate::modules::common::*;

pub struct DebugTemperatureModule {}

impl IModule for DebugTemperatureModule {
    fn get_measurement(&mut self) -> MyResult<StatType> {
        Ok(StatType::Numeric(0.0))
    }

    fn get_measurement_name(&self) -> String {
        "debug_temperature".to_string()
    }

    fn get_base_stat_item(&self) -> Box<dyn StatItem> {
        Box::new(NumericStat::new(self.get_measurement_name()))
    }
}
