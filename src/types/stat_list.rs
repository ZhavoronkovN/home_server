use crate::common::*;
use crate::stat_item::*;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Stats {
    stat_list: HashMap<String, Box<dyn StatItem>>,
}

impl Stats {
    pub fn add_stat_item(&mut self, key: String, new_stat: Box<dyn StatItem>) {
        self.stat_list.insert(key, new_stat);
    }

    pub fn update_stat_item(&mut self, key: &String, val: StatType) -> MyResult<()> {
        match self.stat_list.get_mut(key) {
            Some(item) => Ok(item.update(&val)),
            None => Err(format!(
                "Failed to update stat item, key {} was not found",
                key
            )),
        }
    }
}

impl Clone for Stats {
    fn clone(&self) -> Self {
        Self {
            stat_list: self
                .stat_list
                .iter()
                .map(|(k, v)| (k.clone(), v.box_clone()))
                .collect(),
        }
    }
}

impl JsonConv for Stats {
    fn to_json(&self) -> String {
        format!(
            "{{stats:{{{}}}}}",
            self.stat_list
                .values()
                .map(|i| i.to_json())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
