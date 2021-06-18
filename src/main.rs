use std::{thread, time};
use rust_raspi_led_strip::APA102;
use blink_dbus_server::BlinkDbusService;

fn main() {
    println!("Starting BlinkDbusService!");

    let mut srv = BlinkDbusService::<APA102>::new();
    srv.start(APA102::new(50));

    loop {
        thread::sleep(time::Duration::from_millis(5000));
    }
}
