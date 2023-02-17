#![feature(let_chains)]

use rocket::{get, launch, routes, Config, State};
use std::env;
mod continous;
mod types;
use types::*;
mod modules;
use simple_logger::SimpleLogger;
use std::sync::Arc;

const DEFAULT_I2C_ADDRESS: &str = "/dev/i2c-0";
const DEFAULT_SMOKE_ALARM_PIN: &str = "0";
const DEFAULT_MOTION_DETECT_PIN: &str = "1";
const DEFAULT_SERVER_ADDRESS: &str = "0.0.0.0";
const DEFAULT_SERVER_PORT: &str = "80";

fn _build_debug_getter() -> MyResult<modules::ModuleStatsGetter> {
    let mut getter = modules::ModuleStatsGetter::new();
    getter.add_module(modules::DebugNumericModule {
        name: "debug_temperature".to_string(),
        return_value: 20.0,
    });
    getter.add_module(modules::DebugNumericModule {
        name: "debug_humidity".to_string(),
        return_value: 40.0,
    });
    getter.add_module(modules::DebugBoolModule {
        name: "debug_smoke".to_string(),
        return_value: false,
    });
    getter.add_module(modules::DebugBoolModule {
        name: "debug_motion".to_string(),
        return_value: true,
    });
    Ok(getter)
}

fn _build_getter() -> MyResult<modules::ModuleStatsGetter> {
    let i2c_address = env::var("I2C_ADDRESS").unwrap_or(DEFAULT_I2C_ADDRESS.to_string());
    let smoke_alarm_pin = env::var("SMOKE_ALARM_PIN")
        .unwrap_or(DEFAULT_SMOKE_ALARM_PIN.to_string())
        .parse()
        .map_err(|_| "Failed to parse SMOKE_ALARM_PIN".to_string())?;
    let motion_detect_pin = env::var("MOTION_DETECT_PIN")
        .unwrap_or(DEFAULT_MOTION_DETECT_PIN.to_string())
        .parse()
        .map_err(|_| "Failed to parse MOTION_DETECT_PIN".to_string())?;
    let mut getter = modules::ModuleStatsGetter::new();
    getter.add_module(modules::AM2320Module::new(
        i2c_address.as_str(),
        modules::AM2320Usage::Temperature,
    )?);
    getter.add_module(modules::AM2320Module::new(
        i2c_address.as_str(),
        modules::AM2320Usage::Humidity,
    )?);
    getter.add_module(modules::SysfsPinReader::new(
        smoke_alarm_pin,
        "smoke_alarm".to_string(),
    )?);
    getter.add_module(modules::SysfsPinReader::new(
        motion_detect_pin,
        "motion_detect".to_string(),
    )?);
    Ok(getter)
}

fn _build_cont_getter<G: IStatsGetter + std::marker::Sync + std::marker::Send + 'static>(
    getter: G,
) -> MyResult<continous::ContinousStatsGetter<G>> {
    continous::ContinousStatsGetter::new(getter)
}

#[get("/stats")]
fn stats(getter: &State<Arc<dyn IStatsGetter + Sync + Send>>) -> String {
    match getter.get_stats() {
        Ok(s) => s.to_json(),
        Err(e) => {
            log::error!("Failed to get stats, error : {}", e);
            String::new()
        }
    }
}

#[launch]
fn rocket() -> _ {
    SimpleLogger::new().init().unwrap();
    let g = if let Ok(_) = env::var("USE_DEBUG_GETTER") {
        _build_debug_getter().unwrap()
    } else {
        _build_getter().unwrap()
    };
    let getter: Arc<dyn IStatsGetter + Sync + Send> = Arc::new(_build_cont_getter(g).unwrap());
    let mut config = Config::default();
    config.address = std::net::IpAddr::V4(
        env::var("SERVER_ADDRESS")
            .unwrap_or(DEFAULT_SERVER_ADDRESS.to_string())
            .parse()
            .expect("Failed to parse server address"),
    );
    config.port = env::var("SERVER_PORT")
        .unwrap_or(DEFAULT_SERVER_PORT.to_string())
        .parse()
        .expect("Failed to parse server port");
    rocket::custom(config)
        .manage(getter)
        .mount("/", routes![stats])
}
