use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use crate::types::{IStatsGetter, MyResult, Stats};

pub trait IStatsHistory {
    fn add_to_history(&mut self, stats: Stats) -> MyResult<()>;
    fn get_last(&self) -> MyResult<Stats>;
}

pub struct DummyStatsHistory {
    last_stats: Stats,
}

impl DummyStatsHistory {
    pub fn new() -> Self {
        DummyStatsHistory {
            last_stats: Stats::default(),
        }
    }
}

impl IStatsHistory for DummyStatsHistory {
    fn add_to_history(&mut self, stats: Stats) -> MyResult<()> {
        self.last_stats = stats;
        Ok(())
    }
    fn get_last(&self) -> MyResult<Stats> {
        Ok(self.last_stats.clone())
    }
}

pub struct ContinousStatsGetter<H: IStatsHistory + Send + Sync + 'static> {
    history_saver: Arc<RwLock<H>>,
    is_running: Arc<RwLock<bool>>,
}

impl<H: IStatsHistory + Send + Sync + 'static> ContinousStatsGetter<H> {
    pub fn new<G : IStatsGetter + Send + 'static>(inner_getter: G, history_saver: H) -> Self {
        let history_saver = Arc::new(RwLock::new(history_saver));
        let is_running = Arc::new(RwLock::new(true));
        let h = history_saver.clone();
        let r = is_running.clone();
        std::thread::spawn(move || {
            Self::inner_get(h, inner_getter, r)
        });
        ContinousStatsGetter {
            history_saver,
            is_running,
        }
    }

    fn inner_get(
        history: Arc<RwLock<H>>,
        getter: impl IStatsGetter,
        running: Arc<RwLock<bool>>,
    ) {
        while *running.read().unwrap() {
            let stats = getter.get_stats();
            match history.write() {
                Ok(mut h) => {
                    h.add_to_history(stats).unwrap();
                }
                Err(_) => println!("Failed to add stats to history"),
            }
            thread::sleep(Duration::from_millis(1000));
        }
        println!("Thread stopped");
    }
}

impl<H: IStatsHistory + Send + Sync + 'static> IStatsGetter for ContinousStatsGetter<H> {
    fn get_stats(&self) -> Stats {
        match self.history_saver.read() {
            Ok(h) => match h.get_last() {
                Ok(s) => s,
                Err(_) => Stats::default(),
            },
            Err(_) => Stats::default(),
        }
    }
}

impl<H: IStatsHistory + Send + Sync + 'static> Drop for ContinousStatsGetter<H> {
    fn drop(&mut self) {
        *self.is_running.write().unwrap() = false;
    }
}
