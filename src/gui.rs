

use influxdb::{Client, InfluxDbWriteable};
use iced::{button, Button, Column, Element, Sandbox, Settings, Text, TextInput};

use crate::trading::{monte_carlo_trading, SimulationResult};

// Define the GUI state
#[derive(Default)]
pub struct GuiState {
    initial_investment_input: String,
    num_simulations_input: String,
    start_simulation_button_state: button::State,
    simulation_result: String,
}

// Define the GUI messages
#[derive(Debug, Clone)]
pub enum Message {
    InitialInvestmentChanged(String),
    NumSimulationsChanged(String),
    StartSimulation,
}

// Implement the GUI
impl Sandbox for GuiState {
    type Message = Message;

    fn new() -> Self {
        GuiState::default()
    }

    fn title(&self) -> String {
        String::from("Monte Carlo Trading App")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InitialInvestmentChanged(value) => {
                self.initial_investment_input = value;
            }
            Message::NumSimulationsChanged(value) => {
                self.num_simulations_input = value;
            }
            Message::StartSimulation => {
                if let (Ok(initial_investment), Ok(num_simulations)) = (
                    self.initial_investment_input.parse::<f64>(),
                    self.num_simulations_input.parse::<u32>(),
                ) {
                    // Call the Monte Carlo trading function and store the result in InfluxDB
                    let simulation_result = monte_carlo_trading(initial_investment, num_simulations);

                    // Store the result in the InfluxDB database
                    let influxdb_client = Client::new("http://localhost:8086", "database_name");
                    influxdb_client
                        .write(&simulation_result, None)
                        .expect("Error writing to InfluxDB");

                    // Display the simulation result on the GUI
                    self.simulation_result =
                        format!("Simulation result: {:?}", simulation_result);
                } else {
                    self.simulation_result =
                        String::from("Invalid input. Please enter valid numbers.");
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let initial_investment_input = TextInput::new(
            &mut self.initial_investment_input,
            "Initial Investment",
            &self.initial_investment_input,
            Message::InitialInvestmentChanged,
        );

        let num_simulations_input = TextInput::new(
            &mut self.num_simulations_input,
            "Number of Simulations",
            &self.num_simulations_input,
            Message::NumSimulationsChanged,
        );

        let start_simulation_button = Button::new(
            &mut self.start_simulation_button_state,
            Text::new("Start Simulation"),
        )
            .on_press(Message::StartSimulation);

        Column::new()
            .push(Text::new("Monte Carlo Trading App").size(30))
            .push(Text::new("Enter initial investment:"))
            .push(initial_investment_input)
            .push(Text::new("Enter number of simulations:"))
            .push(num_simulations_input)
            .push(start_simulation_button)
            .push(Text::new(&self.simulation_result))
            .spacing(10)
            .padding(10)
            .into()
    }
}
