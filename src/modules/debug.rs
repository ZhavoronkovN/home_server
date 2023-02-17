use crate::{modules::common::*, types::stat_item::NumericType};

pub struct DebugNumericModule {
    pub return_value: NumericType,
    pub name: String,
}
pub struct DebugBoolModule {
    pub return_value: bool,
    pub name: String,
}

impl IModule for DebugNumericModule {
    fn get_measurement(&mut self) -> MyResult<StatType> {
        Ok(StatType::Numeric(self.return_value))
    }

    fn get_measurement_name(&self) -> String {
        self.name.clone()
    }

    fn get_base_stat_item(&self) -> Box<dyn StatItem> {
        Box::new(NumericStat::new(self.get_measurement_name()))
    }
}

impl IModule for DebugBoolModule {
    fn get_measurement(&mut self) -> MyResult<StatType> {
        Ok(StatType::Bool(self.return_value))
    }

    fn get_measurement_name(&self) -> String {
        self.name.clone()
    }

    fn get_base_stat_item(&self) -> Box<dyn StatItem> {
        Box::new(BoolStat::new(self.get_measurement_name()))
    }
}
