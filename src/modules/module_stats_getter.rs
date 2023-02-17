use crate::modules::common::*;
use crate::types::stat_list::Stats;

pub struct ModuleStatsGetter {
    modules: Vec<Box<dyn IModule + Sync + Send>>,
    last_stats: Stats,
}

impl ModuleStatsGetter {
    pub fn new() -> Self {
        ModuleStatsGetter {
            modules: Vec::new(),
            last_stats: Stats::default(),
        }
    }

    pub fn add_module<M: IModule + Sync + Send + 'static>(&mut self, module: M) {
        self.last_stats
            .add_stat_item(module.get_key(), module.get_stat_item());
        self.modules.push(Box::new(module));
    }
}

impl IStatsGetter for ModuleStatsGetter {
    fn update_stats(&mut self) -> MyResult<()> {
        for m in &mut self.modules {
            self.last_stats
                .update_stat_item(&m.get_key(), m.get_measurement()?)?
        }
        Ok(())
    }

    fn get_stats(&self) -> MyResult<Stats> {
        Ok(self.last_stats.clone())
    }
}
