use std::fmt::Debug;

use chrono::prelude::*;

pub type MyResult<T> = Result<T, String>;
type NumericType = f32;

pub trait StatItem {
    fn from_value<T>(val : T) -> Self;
    fn update<T>(&mut self, val: T);
}

pub trait JsonConv {
    fn to_json(&self) -> String;
}

pub trait IStatsGetter {
    fn update_stats(&mut self) -> MyResult<()>;
    fn get_stats(&self) -> MyResult<Stats>;
}

#[derive(Default, Clone, std::fmt::Debug)]
pub struct NumericStat {
    pub last: NumericType,
    pub avg: NumericType,
    pub min: NumericType,
    pub max: NumericType,
    observations: u64,
    name: String,
}

#[derive(Default, Clone, std::fmt::Debug)]
pub struct BoolStat {
    pub triggered: bool,
    pub last_triggered: i64,
    pub total_triggers: u64,
    name: String,
}

#[derive(Default, Debug, Clone)]
pub struct Stats {
    stat_list : Vec<dyn StatItem + Debug + Default + Clone>
}

impl StatItem for BoolStat {
    fn from_value(val : bool) -> Self {
        let mut res = Self::default();
        res.name = "unknown_bool_stat".to_string();
        res.update(val);
        res
    }

    fn update(&mut self, trig: bool) {
        self.triggered = trig;
        if trig {
            self.last_triggered = Utc::now().timestamp();
            self.total_triggers += 1;
        }
    }
}

impl StatItem<NumericType> for NumericStat {
    fn from_value(val : NumericType) -> Self {
        let mut res = Self::default();
        res.name = "unknown_numeric_stat".to_string();
        res.update(val);
        res
    }

    fn update(&mut self, val: NumericType) {
        if self.observations == u64::MAX {
            *self = Self::default();
        }
        self.last = val;
        self.avg = self.avg * self.observations as f32 + val / (self.observations + 1) as f32;
        self.min = self.min.min(val);
        self.max = self.max.max(val);
        self.observations += 1;
    }
}

impl StatItem<Stats> for Stats {
    fn update(&mut self, val: Stats) {
        self.temperature.update(val.temperature.last);
        self.humidity.update(val.humidity.last);
        self.smoke_alarm.update(val.smoke_alarm.triggered);
        self.motion_detect.update(val.motion_detect.triggered);
    }
}

impl JsonConv for BoolStat {
    fn to_json(&self) -> String {
        format!(
            "{}:{{triggered:{},last_triggered:{},total_triggers:{}}}",
            self.name, self.triggered, self.last_triggered, self.total_triggers
        )
    }
}

impl JsonConv for NumericStat {
    fn to_json(&self) -> String {
        format!(
            "{}:{{value:{},avg:{},min:{},max:{}}}",
            self.name, self.last, self.avg, self.min, self.max
        )
    }
}

impl JsonConv for Stats {
    fn to_json(&self) -> String {
        format!(
            "{{stats:{{{},{},{},{}}}",
            self.temperature.to_json(),
            self.humidity.to_json(),
            self.smoke_alarm.to_json(),
            self.motion_detect.to_json()
        )
    }
}
