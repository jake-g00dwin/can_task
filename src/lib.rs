/*
 * Filename: 
 * Date: 2024
 * Description: This acts as the shared API crate for sensor modules on the CAN
 * bus. The two kinds of structures are for the sensorsmodules and a handler
 * for them on the SBC side of things.
 *
 * This should allow plug and play capibility where the type and format of 
 * data can be discovered by queriing the can bus sensor modules on the fly.
 *
 * It will also allow the front-end/SBC team to loop through an array or vector
 * of sensor module handlers as needed to get sensor infomation without
 * having to write sensor specific code for each kind of sensor module to parse
 * their data and re-format it.
 */


//ideally we should put a sensor name here.
use embedded_hal::can;

pub const NAME: &'static str = "sensor_module";

#[allow(dead_code)]
enum Status {
    Availble = 0,
    Busy,
    Error,
    Unknown,
}

#[allow(dead_code)]
struct CanSensorModuleController {
    name: &'static str,
    status: Status,
}

//This is mostly for the receiver side
impl CanSensorModuleController {
    #[allow(dead_code)]
    fn new(module_name: &'static str) -> CanSensorModuleController{
        CanSensorModuleController {name: module_name, status: Status::Unknown} 
    }

    //sends a request to the sensor module over can
    fn get_sensor_name() {

    }

    //sends a request to the sensor module over can
    fn get_sensor_status() {

    }

    //sends a request to the sensor module over can
    fn get_sensor_format() {

    }

    fn get_sensor_data() {

    }

}



#[allow(dead_code)]
struct CanSensorModule {
    name: &'static str,
    status: Status,
}

//This is for the actual sensor modules themselves
impl CanSensorModule {
    #[allow(dead_code)]
    fn new(module_name: &'static str) -> CanSensorModule{
        CanSensorModule {name: module_name, status: Status::Unknown} 
    }

    //sends a request to the sensor module over can
    fn send_sensor_name() {

    }

    //sends a request to the sensor module over can
    fn send_sensor_status() {

    }

    //sends a request to the sensor module over can
    fn send_sensor_format() {

    }

    fn send_sensor_data() {

    }

}



#[cfg(test)]
mod can_interface_tests {
    use super::*;

    #[test]
    fn new_instance() {
        let _s = CanSensorModuleController::new(NAME);
        assert!(true);
    }

    #[test]
    fn get_sensor_name() {
        let s = CanSensorModuleController::new(NAME);
        let res = s.get_sensor_name().unwrap();
        assert_eq!(res, NAME);
    }

    #[test]
    fn get_sensor_status() {
        
    }

    #[test]
    fn get_sensor_format() {

    }

    #[test]
    fn get_sensor_format() {

    }

    #[test]
    fn get_sensor_readings() {

    }
}


