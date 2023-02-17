use crate::types::{IStatsGetter, MyResult, Stats};
use log;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

const SECOND_TIME: u64 = 1000;
const MINUTE_TIME: u64 = SECOND_TIME * 60;
const HOUR_TIME: u64 = MINUTE_TIME * 60;
const DAY_TIME: u64 = HOUR_TIME * 24;
const WEEK_TIME: u64 = DAY_TIME * 7;
const MONTH_TIME: u64 = WEEK_TIME * 4;

const UPDATE_RATE: u64 = SECOND_TIME;

pub struct ContinousStatsGetter {
    last_stats: Arc<RwLock<Stats>>,
    is_running: Arc<RwLock<bool>>,
}

impl ContinousStatsGetter {
    pub fn new<G: IStatsGetter + Send + 'static>(inner_getter: G) -> MyResult<Self> {
        let last_stats = Arc::new(RwLock::new(Stats::default()));
        let is_running = Arc::new(RwLock::new(false));
        let mut g = ContinousStatsGetter {
            last_stats,
            is_running,
        };
        g.start_update(inner_getter)?;
        Ok(g)
    }

    fn start_update<G: IStatsGetter + Send + 'static>(&mut self, inner_getter: G) -> MyResult<()> {
        if *self
            .is_running
            .read()
            .map_err(|_| "Failed to read is_running to check if thread is running".to_string())?
            == true
        {
            Err("Update thread is already running".to_string())
        } else {
            *self
                .is_running
                .write()
                .map_err(|_| "Failed to write is_running to run thread".to_string())? = true;
            let l = self.last_stats.clone();
            let r = self.is_running.clone();
            std::thread::spawn(move || Self::inner_get(l, inner_getter, r));
            Ok(())
        }
    }

    fn inner_get(
        last: Arc<RwLock<Stats>>,
        mut getter: impl IStatsGetter,
        running: Arc<RwLock<bool>>,
    ) {
        log::info!("Starting stat getter thread...");
        let sleep_time = std::env::var("UPDATE_RATE").unwrap_or(UPDATE_RATE.to_string()).parse().unwrap_or(UPDATE_RATE);
        while let Ok(r) = running.read() && *r {
            match getter.update_stats() {
                Ok(()) => {
                    match getter.get_stats() {
                        Ok(s) => {
                            match last.write() {
                                Ok(mut l) => {l.update(s); log::debug!("Stats were updated");}
                                Err(_) => log::error!("Last stats are poisoned, won't do update"),
                            }
                        },
                        Err(e) => log::error!("Failed to get stats, error : {}", e)
                    }
                },
                Err(e) => log::error!("Failed to update stats, error : {}", e)
            }
            thread::sleep(Duration::from_millis(sleep_time));
        }
        log::info!("Thread stopped");
    }
}

impl IStatsGetter for ContinousStatsGetter {
    fn get_stats(&self) -> MyResult<Stats> {
        Ok(self
            .last_stats
            .read()
            .map_err(|_| "Last stats are poisoned, failed to read".to_string())?
            .clone())
    }

    fn update_stats(&mut self) -> MyResult<()> {
        if *self.is_running.read().map_err(|_| {
            "Is_running is poisoned, failed to check if thread is running".to_string()
        })? {
            Ok(())
        } else {
            Err("Update thread is not running".to_string())
        }
    }
}

impl Drop for ContinousStatsGetter {
    fn drop(&mut self) {
        if let Ok(mut r) = self.is_running.write() {
            *r = false
        } else {
            log::error!("Failed to write running false");
        }
    }
}
