use chrono::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
pub type MyResult<T> = Result<T, String>;
pub type NumericType = f32;

pub enum StatType {
    Bool(bool),
    Numeric(NumericType),
}

pub trait StatItem: Debug + JsonConv + Sync + Send {
    fn update(&mut self, val: StatType);
    fn get_name(&self) -> String;
    fn get_last(&self) -> StatType;
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

#[derive(Default, Debug)]
pub struct Stats {
    stat_list: HashMap<String, Box<dyn StatItem>>,
}

impl Stats {
    pub fn update(&mut self, other: Stats) {
        self.stat_list.iter_mut().for_each(|(k, v)| {
            if other.stat_list.contains_key(k) {
                v.update(
                    other
                        .stat_list
                        .get(k)
                        .expect("Failed to get stat by key")
                        .get_last(),
                )
            }
        })
    }

    pub fn add_stat_item(&mut self, new_stat: Box<dyn StatItem>) {
        self.stat_list.insert(new_stat.get_name(), new_stat);
    }

    pub fn update_stat_item(&mut self, key: &String, val: StatType) -> MyResult<()> {
        match self.stat_list.get_mut(key) {
            Some(item) => Ok(item.update(val)),
            None => Err(format!(
                "Failed to update stat item, key {} was not found",
                key
            )),
        }
    }
}

impl Clone for Stats {
    fn clone(&self) -> Self {
        self.stat_list.iter().map(|(k,v)| (k.clone(), v.box_clone()));
        Self { stat_list: self.stat_list.clone() }
    }
}


impl StatItem for BoolStat {
    fn update(&mut self, trig: StatType) {
        match trig {
            StatType::Numeric(_) => panic!("Attempt to update bool stat with number"),
            StatType::Bool(b) => {
                self.triggered = b;
                if b {
                    self.last_triggered = Utc::now().timestamp();
                    self.total_triggers += 1;
                }
            }
        }
    }

    fn get_name(&self) -> String {
        self.name
    }

    fn get_last(&self) -> StatType {
        StatType::Bool(self.triggered)
    }
}

impl StatItem for NumericStat {
    fn update(&mut self, val: StatType) {
        match val {
            StatType::Bool(_) => panic!("Attempt to update numeric stat with bool"),
            StatType::Numeric(n) => {
                if self.observations == u64::MAX {
                    *self = Self::default();
                }
                self.last = n;
                self.avg = self.avg * self.observations as f32 + n / (self.observations + 1) as f32;
                self.min = self.min.min(n);
                self.max = self.max.max(n);
                self.observations += 1;
            }
        }
    }

    fn get_name(&self) -> String {
        self.name
    }

    fn get_last(&self) -> StatType {
        StatType::Numeric(self.last)
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
            "{{stats:{{{}}}",
            self.stat_list
                .values()
                .map(|i| i.to_json())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
