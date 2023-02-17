use crate::types::{IStatsGetter, MyResult, Stats};
use log;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

const UPDATE_RATE: u64 = 1000;

pub struct ContinousStatsGetter<G: IStatsGetter + Sync + Send + 'static> {
    inner_getter: Arc<RwLock<G>>,
    is_running: Arc<RwLock<bool>>,
}

impl<G: IStatsGetter + Sync + Send + 'static> ContinousStatsGetter<G> {
    pub fn new(inner_getter: G) -> MyResult<Self> {
        let inner_getter = Arc::new(RwLock::new(inner_getter));
        let is_running = Arc::new(RwLock::new(false));
        let mut g = ContinousStatsGetter {
            inner_getter,
            is_running,
        };
        g.start_update()?;
        Ok(g)
    }

    fn start_update(&mut self) -> MyResult<()> {
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
            let g = self.inner_getter.clone();
            let r = self.is_running.clone();
            std::thread::spawn(move || Self::inner_get(g, r));
            Ok(())
        }
    }

    fn inner_get(getter: Arc<RwLock<G>>, running: Arc<RwLock<bool>>) {
        log::info!("Starting stat getter thread...");
        let sleep_time = std::env::var("UPDATE_RATE")
            .unwrap_or(UPDATE_RATE.to_string())
            .parse()
            .unwrap_or(UPDATE_RATE);
        while let Ok(r) = running.read() && *r {
            thread::sleep(Duration::from_millis(sleep_time));
            if let Ok(mut g) = getter.write() {
                if let Err(e) = g.update_stats() {
                    log::error!("Failed to update stats, error : {}", e);
                    continue;
                }
            }
            else {
                log::error!("Failed to update stats, getter is posioned");
                continue;
            }
        }
        log::info!("Thread stopped");
    }
}

impl<G: IStatsGetter + Sync + Send + 'static> IStatsGetter for ContinousStatsGetter<G> {
    fn get_stats(&self) -> MyResult<Stats> {
        Ok(self
            .inner_getter
            .read()
            .map_err(|_| "Inner getter is poisoned")?
            .get_stats()?
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

impl<G: IStatsGetter + Sync + Send + 'static> Drop for ContinousStatsGetter<G> {
    fn drop(&mut self) {
        if let Ok(mut r) = self.is_running.write() {
            *r = false
        } else {
            log::error!("Failed to write running false");
        }
    }
}
