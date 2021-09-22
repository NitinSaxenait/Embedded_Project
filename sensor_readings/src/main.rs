#![no_main]
#![no_std]

use sensor_readings::config::initialization::{
    entry, init, iprintln, switch_hal::OutputSwitch, Direction,
};
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use cortex_m::asm;

/// This program is going to print the (x,y) axis values on itm terminal and will blink the led
/// based on the (x,y)->Magnetic Field Direction.
/// This program will use the discovery board as a Compass.
///
/// #Return
/// Program is using [no_std] & [no_main] therefore it will neither end nor return anything.
#[entry]
fn main() -> ! {
    let (leds, mut lsm303agr, mut delay, mut itm) = init();
    let mut stm_leds = leds.into_array();

    loop {
        // Reading the magnetometer register's (x,y) value using mag_data().
        let magnetometer_data = lsm303agr.mag_data();
        let x_y_axis = match magnetometer_data {
            Ok(x_y_axis) => x_y_axis,
            Err(error) => {
                panic!("Reading not found {:?}", error)
            }
        };

        iprintln!(&mut itm.stim[0], "x = {} y = {} z = {}", x_y_axis.x, x_y_axis.y, x_y_axis.z);

        asm::delay(50_000_000);
    }
}