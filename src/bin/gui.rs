use iced::{
    widget::{button, column, row, text, text_input},
    Alignment, Element, Sandbox, Settings,
};

// --- Helper Functions ---

/// Rounds a floating-point number to a specified number of decimal places.
fn round_to_decimal_places(n: f64, decimal_places: u32) -> f64 {
    let multiplier = 10.0_f64.powi(decimal_places as i32);
    (n * multiplier).round() / multiplier
}


// --- Calculator Definitions ---

/// Enum defining all the calculators available in the app.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Calculator {
    Bodmas,
    DecimalPlaces,
    Estimation,
    EstimationSquareRoot,
    Hcf,
    Lcm,
    Multiples,
    PrimeNumbers,
    ProdPrimeFactor,
    Rounding,
    SignificantFigures,
    UpperLowerBounds,
}

impl Calculator {
    /// A list of all calculator variants.
    const ALL: &'static [Calculator] = &[
        Calculator::Bodmas,
        Calculator::DecimalPlaces,
        Calculator::Estimation,
        Calculator::EstimationSquareRoot,
        Calculator::Hcf,
        Calculator::Lcm,
        Calculator::Multiples,
        Calculator::PrimeNumbers,
        Calculator::ProdPrimeFactor,
        Calculator::Rounding,
        Calculator::SignificantFigures,
        Calculator::UpperLowerBounds,
    ];

    /// Returns the display name of the calculator.
    fn name(&self) -> &'static str {
        match self {
            Calculator::Bodmas => "BODMAS Calculator",
            Calculator::DecimalPlaces => "Decimal Places",
            Calculator::Estimation => "Estimation",
            Calculator::EstimationSquareRoot => "Estimation of Square Root",
            Calculator::Hcf => "Highest Common Factor (HCF)",
            Calculator::Lcm => "Lowest Common Multiple (LCM)",
            Calculator::Multiples => "Multiples",
            Calculator::PrimeNumbers => "Prime Numbers",
            Calculator::ProdPrimeFactor => "Product of Prime Factors",
            Calculator::Rounding => "Rounding",
            Calculator::SignificantFigures => "Significant Figures",
            Calculator::UpperLowerBounds => "Upper and Lower Bounds",
        }
    }
}

// --- Application State and Messages ---

/// State for the Decimal Places calculator.
#[derive(Debug, Clone, Default)]
struct DecimalPlacesState {
    number_input: String,
    places_input: String,
    result: Option<f64>,
}

/// The overall state of our application.
struct MathGui {
    selected_calculator: Option<Calculator>,
    decimal_places_state: DecimalPlacesState,
}

/// Messages for the Decimal Places calculator.
#[derive(Debug, Clone)]
pub enum DecimalPlacesMessage {
    NumberInputChanged(String),
    PlacesInputChanged(String),
    Calculate,
    Reset,
}

/// The messages that can be sent to update the state.
#[derive(Debug, Clone)]
enum Message {
    CalculatorSelected(Calculator),
    BackToMenu,
    DecimalPlaces(DecimalPlacesMessage),
}

// --- Main Application Logic ---

pub fn main() -> iced::Result {
    MathGui::run(Settings::default())
}

impl Sandbox for MathGui {
    type Message = Message;

    fn new() -> Self {
        Self {
            selected_calculator: None,
            decimal_places_state: DecimalPlacesState::default(),
        }
    }

    fn title(&self) -> String {
        String::from("MathRust GUI")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CalculatorSelected(calculator) => {
                self.selected_calculator = Some(calculator);
            }
            Message::BackToMenu => {
                self.selected_calculator = None;
                // Reset the state when going back to the menu
                self.decimal_places_state = DecimalPlacesState::default();
            }
            Message::DecimalPlaces(msg) => {
                let state = &mut self.decimal_places_state;
                match msg {
                    DecimalPlacesMessage::NumberInputChanged(value) => {
                        state.number_input = value;
                    }
                    DecimalPlacesMessage::PlacesInputChanged(value) => {
                        state.places_input = value;
                    }
                    DecimalPlacesMessage::Calculate => {
                        let number: Result<f64, _> = state.number_input.parse();
                        let places: Result<u32, _> = state.places_input.parse();

                        if let (Ok(num), Ok(p)) = (number, places) {
                            state.result = Some(round_to_decimal_places(num, p));
                        } else {
                            state.result = None;
                        }
                    }
                    DecimalPlacesMessage::Reset => {
                        *state = DecimalPlacesState::default();
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = if let Some(calculator) = self.selected_calculator {
            // --- View for a selected calculator ---
            let view_content = match calculator {
                Calculator::DecimalPlaces => {
                    let state = &self.decimal_places_state;
                    let result_text = match state.result {
                        Some(res) => format!("Result: {}", res),
                        None => "Please enter valid numbers.".to_string(),
                    };

                    column![
                        text(calculator.name()).size(30),
                        text_input("Number to round", &state.number_input)
                            .on_input(|s| Message::DecimalPlaces(DecimalPlacesMessage::NumberInputChanged(s))),
                        text_input("Decimal places", &state.places_input)
                            .on_input(|s| Message::DecimalPlaces(DecimalPlacesMessage::PlacesInputChanged(s))),
                        row![
                            button("Calculate").on_press(Message::DecimalPlaces(DecimalPlacesMessage::Calculate)),
                            button("Reset").on_press(Message::DecimalPlaces(DecimalPlacesMessage::Reset)),
                        ].spacing(10),
                        text(result_text).size(25),
                        button("Back").on_press(Message::BackToMenu),
                    ]
                },
                _ => {
                    column![
                        text(calculator.name()).size(30),
                        text("This calculator has not been implemented in the GUI yet."),
                        button("Back").on_press(Message::BackToMenu),
                    ]
                }
            };
            view_content
        } else {
            // --- Main menu view ---
            let menu_buttons = Calculator::ALL.iter().fold(column![], |col, &calc| {
                col.push(button(calc.name()).on_press(Message::CalculatorSelected(calc)))
            });

            column![
                text("Select a Calculator").size(40),
                menu_buttons.spacing(10),
            ]
        }
        .spacing(20)
        .padding(20)
        .align_items(Alignment::Center);

        content.into()
    }
}
