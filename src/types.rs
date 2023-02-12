pub type MyResult<T> = Result<T, String>;

#[derive(Default, Debug, Clone, Copy)]
pub struct Stats {
    pub temperature: f32,
    pub humidity: f32,
}

impl Stats {
    pub fn to_json(&self) -> String {
        let data = format!(
            "temperature:\"{}\",humidity:\"{}\"",
            &self.temperature, &self.humidity
        );
        "{stats:{".to_string() + data.as_str() + "}}"
    }
}

pub trait IStatsGetter {
    fn get_stats(&self) -> Stats;
}

pub trait ITemperatureHumidityModule {
    fn get_temperature_humidity(&self) -> (f32, f32);
}
