pub type MyResult<T> = Result<T, String>;

pub trait JsonConv {
    fn to_json(&self) -> String;
}

pub trait IStatsGetter {
    fn update_stats(&mut self) -> MyResult<()>;
    fn get_stats(&self) -> MyResult<crate::stat_list::Stats>;
}
