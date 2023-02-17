use crate::common::*;
use chrono::Utc;
use std::fmt::Debug;
pub type NumericType = f32;

pub enum StatType {
    Bool(bool),
    Numeric(NumericType),
}

pub trait StatItem: Debug + JsonConv + Sync + Send {
    fn update(&mut self, val: &StatType);
    fn get_last(&self) -> StatType;
    fn box_clone(&self) -> Box<dyn StatItem>;
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

impl NumericStat {
    pub fn new(name: String) -> Self {
        let mut res = Self::default();
        res.name = name;
        res
    }
}

#[derive(Default, Clone, std::fmt::Debug)]
pub struct BoolStat {
    pub triggered: bool,
    pub last_triggered: i64,
    pub total_triggers: u64,
    name: String,
}

impl BoolStat {
    pub fn new(name: String) -> Self {
        let mut res = Self::default();
        res.name = name;
        res
    }
}

impl StatItem for BoolStat {
    fn update(&mut self, trig: &StatType) {
        match trig {
            StatType::Numeric(_) => panic!("Attempt to update bool stat with number"),
            StatType::Bool(b) => {
                self.triggered = *b;
                if *b {
                    self.last_triggered = Utc::now().timestamp();
                    self.total_triggers += 1;
                }
            }
        }
    }

    fn get_last(&self) -> StatType {
        StatType::Bool(self.triggered)
    }

    fn box_clone(&self) -> Box<dyn StatItem> {
        Box::new(self.clone())
    }
}

impl StatItem for NumericStat {
    fn update(&mut self, val: &StatType) {
        match val {
            StatType::Bool(_) => panic!("Attempt to update numeric stat with bool"),
            StatType::Numeric(n) => {
                if self.observations == u64::MAX {
                    let name = self.name.clone();
                    *self = Self::default();
                    self.name = name;
                }
                self.last = *n;
                self.avg = self.avg * self.observations as f32 + n / (self.observations + 1) as f32;
                self.min = self.min.min(*n);
                self.max = self.max.max(*n);
                self.observations += 1;
            }
        }
    }

    fn get_last(&self) -> StatType {
        StatType::Numeric(self.last)
    }

    fn box_clone(&self) -> Box<dyn StatItem> {
        Box::new(self.clone())
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
