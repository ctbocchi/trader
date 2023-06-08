use influxdb::{Client, InfluxDbWriteable};

// Function to write simulation result to InfluxDB
pub fn write_simulation_result(simulation_result: &SimulationResult) {
    // Create an InfluxDB client
    let influxdb_client = Client::new("http://localhost:8086", "database_name");

    // Write the simulation result to InfluxDB
    influxdb_client
        .write(simulation_result, None)
        .expect("Error writing to InfluxDB");
}

// Struct to hold the simulation result
#[derive(Debug, InfluxDbWriteable)]
struct SimulationResult {
    #[influxdb(tag)]
    simulation_id: u32,
    average_return: f64,
    std_deviation: f64,
}
