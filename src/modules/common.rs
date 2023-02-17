pub use crate::types::common::*;
pub use crate::types::stat_item::{BoolStat, NumericStat, StatItem, StatType};

pub trait IModule {
    fn get_measurement(&mut self) -> MyResult<StatType>;
    fn get_measurement_name(&self) -> String;
    fn get_base_stat_item(&self) -> Box<dyn StatItem>;
}
