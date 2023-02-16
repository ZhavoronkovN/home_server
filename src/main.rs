#![feature(let_chains)]

use rocket::{get, launch, routes, State};
mod continous;
mod modules;
mod types;
use modules::TGetter;
use types::{IStatsGetter, MyResult, JsonConv};
use std::sync::Arc;
use simple_logger::SimpleLogger;

const I2C_ADDRESS: &str = "/dev/i2c-0";
const SMOKE_ALARM_PIN : u16 = 10;
const MOTION_DETECT_PIN : u16 = 20;

fn _build_debug_getter() -> MyResult<modules::ModuleStatsGetter> {
    Ok(modules::ModuleStatsGetter::new(modules::DebugTemperatureHumidityModule {}, modules::DebugPinModule {}, modules::DebugPinModule {}))
}

fn _build_getter() -> MyResult<modules::ModuleStatsGetter> {
    Ok(modules::ModuleStatsGetter::new(modules::AM2320Module::new(I2C_ADDRESS)?, modules::SysfsPinReader::new(SMOKE_ALARM_PIN)?, modules::SysfsPinReader::new(MOTION_DETECT_PIN)?))
}

fn _build_cont_getter(getter : impl IStatsGetter + std::marker::Send + 'static) -> MyResult<continous::ContinousStatsGetter> {
    continous::ContinousStatsGetter::new(getter)
}

#[get("/stats")]
fn stats(getter: &State<Arc<TGetter>>) -> String {
    match getter.get_stats() {
        Ok(s) => s.to_json(),
        Err(e) => {log::error!("Failed to get stats, error : {}",e); String::new()}
    }
}

#[launch]
fn rocket() -> _ {
    SimpleLogger::new().init().unwrap();
    let getter : Arc<TGetter> = Arc::new(_build_cont_getter(_build_debug_getter().unwrap()).unwrap());
    rocket::build().manage(getter).mount("/", routes![stats])
}
