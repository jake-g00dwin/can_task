# CAN Sensor Module Interface

 Description: This acts as the shared API crate for sensor modules on the CAN
 bus. The two kinds of structures are for the sensorsmodules and a handler
 for them on the SBC side of things.

 This should allow plug and play capibility where the type and format of 
 data can be discovered by queriing the can bus sensor modules on the fly.

 It will also allow the front-end/SBC team to loop through an array or vector
 of sensor module handlers as needed to get sensor infomation without
 having to write sensor specific code for each kind of sensor module to parse
 their data and re-format it.
 
 ## Goals
 
 * remove sensor specific code/formatting from the SBC
 * Give plug and play capability for new sensor modules.
 * Allow duplicate sensors from differnt sensor_modules.
 * give a standard interface for both teams.
 * Handle all the CAN bus code.

## Shared interface:

The required methods for each of the types:

**Controller:**

- `get_sensor_name()`, requests the sensor modules name.
- `get_sensor_status()`, requests a enum representing the sensor modules status.
- `get_sensor_format()`, requests the formatting for the sensor data.
- `get_sensor_data()`, requests the actual formated sensor data.

**SensorModule:**

- `send_sensor_name()`, sends the sensor modules name+node id
- `send_sensor_status()`, sends a enum representing the sensor modules status.
- `send_sensor_format()`, sends the formatting for the sensor data.
- `send_sensor_data()`, sends the actual formated sensor data.


