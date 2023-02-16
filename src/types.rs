use chrono::prelude::*;

pub type MyResult<T> = Result<T, String>;
type NumericType = f32;

pub trait StatItem<T> {
    fn update(&mut self, val: T);
}

pub trait JsonConv {
    fn to_json(&self) -> String;
}

#[derive(Default, Clone, Copy, std::fmt::Debug)]
pub struct NumericStat {
    pub last: NumericType,
    pub avg: NumericType,
    pub min: NumericType,
    pub max: NumericType,
    observations: u64,
}

#[derive(Default, Clone, Copy, std::fmt::Debug)]
pub struct BoolStat {
    pub triggered: bool,
    pub last_triggered: i64,
    pub total_triggers: u64,
}

impl StatItem<bool> for BoolStat {
    fn update(&mut self, trig: bool) {
        self.triggered = trig;
        if trig {
            self.last_triggered = Utc::now().timestamp();
            self.total_triggers += 1;
        }
    }
}

impl StatItem<NumericType> for NumericStat {
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

impl JsonConv for BoolStat {
    fn to_json(&self) -> String {
        format!(
            "{{triggered:{},last_triggered:{},total_triggers:{}}}",
            self.triggered, self.last_triggered, self.total_triggers
        )
    }
}

impl JsonConv for NumericStat {
    fn to_json(&self) -> String {
        format!(
            "{{value:{},avg:{},min:{},max:{}}}",
            self.last, self.avg, self.min, self.max
        )
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Stats {
    pub temperature: NumericStat,
    pub humidity: NumericStat,
    pub smoke_alarm: BoolStat,
    pub motion_detect: BoolStat,
}

impl JsonConv for Stats {
    fn to_json(&self) -> String {
        format!(
            "{{stats:{{temperature:\"{}\",humidity:\"{}\",smoke:\"{}\",motion:\"{}\"}}",
            self.temperature.to_json(),
            self.humidity.to_json(),
            self.smoke_alarm.to_json(),
            self.motion_detect.to_json()
        )
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

pub trait IStatsGetter {
    fn update_stats(&mut self) -> MyResult<()>;
    fn get_stats(&self) -> MyResult<Stats>;
}

pub trait ITemperatureHumidityModule {
    fn get_temperature_humidity(&mut self) -> MyResult<(f32, f32)>;
}

pub trait IPinReader {
    fn get_triggered(&mut self) -> MyResult<bool>;
}
