use std::fmt::Display;

pub struct Stats<T: Display, H: Display> {
    pub temperature: T,
    pub humidity: H,
}

impl<T: Display, H: Display> Stats<T, H> {
    pub fn to_json(&self) -> String {
        let data = format!(
            "temperature:\"{}\",humidity:\"{}\"",
            &self.temperature, &self.humidity
        );
        "{stats:{".to_string() + data.as_str() + "}}"
    }
}

pub trait IStatsGetter<T: Display, H: Display> {
    fn get_stats(&self) -> Stats<T, H>;
}

pub trait ITemperatureHumidityModule<T : Display, H : Display> {
    fn get_temperature_humidity(&self) -> (T, H);
}
