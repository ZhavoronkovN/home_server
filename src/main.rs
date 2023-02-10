use rocket::{get, launch, routes, State};
mod modules;
mod types;
use modules::TGetter;
use std::sync::Arc;

const I2C_ADDRESS : &str = "/dev/i2c-0";

fn build_debug_getter() -> Arc<TGetter> {
    Arc::new(modules::ModuleStatsGetter {
        temperature_humidity_module: Box::new(modules::DebugTemperatureHumidityModule {}),
    })
}

fn build_getter() -> Arc<TGetter> {
    Arc::new(modules::ModuleStatsGetter {
        temperature_humidity_module: Box::new(modules::AM2320Module::new(I2C_ADDRESS)),
    })
}

#[get("/stats")]
fn stats(getter: &State<Arc<TGetter>>) -> String {
    getter.get_stats().to_json()
}

#[launch]
fn rocket() -> _ {
    let getter = build_getter();
    rocket::build().manage(getter).mount("/", routes![stats])
}
