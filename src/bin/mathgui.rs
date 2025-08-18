use iced::{
    widget::{button, column, row, text, text_input},
    Alignment, Element, Sandbox, Settings,
};
use std::collections::BTreeMap;

// --- Helper Functions ---

/// Rounds a floating-point number to a specified number of decimal places.
fn round_to_decimal_places(n: f64, decimal_places: u32) -> f64 {
    let multiplier = 10.0_f64.powi(decimal_places as i32);
    (n * multiplier).round() / multiplier
}

/// Calculates the greatest common divisor (GCD) of two numbers using the Euclidean algorithm.
fn hcf(a: u32, b: u32) -> u32 {
    let mut temp_a = a;
    let mut temp_b = b;
    while temp_b != 0 {
        let t = temp_b;
        temp_b = temp_a % temp_b;
        temp_a = t;
    }
    temp_a
}

/// Calculates the least common multiple (LCM) of two numbers.
fn lcm(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / hcf(a, b)) * b
    }
}

/// Calculates the first `count` multiples of a given `number`.
fn get_multiples(number: u32, count: usize) -> Vec<u32> {
    (1..=count).map(|i| number * (i as u32)).collect()
}

/// Checks if a given number is prime using an optimized method.
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    let limit = (n as f64).sqrt() as u32;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Calculates the prime factorization of a given number `n`.
fn prime_factorization(mut n: u32) -> BTreeMap<u32, u32> {
    let mut factors = BTreeMap::new();
    let mut d = 2;

    while n > 1 {
        while n % d == 0 {
            *factors.entry(d).or_insert(0) += 1;
            n /= d;
        }
        d += 1;
        if d * d > n {
            if n > 1 {
                *factors.entry(n).or_insert(0) += 1;
            }
            break;
        }
    }
    factors
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
    EquivalentFraction,
    SimplifyingFractions,
    MixedNumbers,
    OrderingFractions,
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
        Calculator::EquivalentFraction,
        Calculator::SimplifyingFractions,
        Calculator::MixedNumbers,
        Calculator::OrderingFractions,
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
            Calculator::EquivalentFraction => "Equivalent Fractions",
            Calculator::SimplifyingFractions => "Simplifying Fractions",
            Calculator::MixedNumbers => "Mixed Numbers",
            Calculator::OrderingFractions => "Ordering Fractions",
        }
    }
}

// --- Application State and Messages ---

/// State for the BODMAS calculator.
#[derive(Debug, Clone, Default)]
struct BodmasState {
    expression: String,
    result: Option<String>,
}

/// State for the Decimal Places calculator.
#[derive(Debug, Clone, Default)]
struct DecimalPlacesState {
    number_input: String,
    places_input: String,
    result: Option<f64>,
}

/// State for the HCF calculator.
#[derive(Debug, Clone, Default)]
struct HcfState {
    numbers_input: String,
    result: Option<String>,
}

/// State for the LCM calculator.
#[derive(Debug, Clone, Default)]
struct LcmState {
    numbers_input: String,
    result: Option<String>,
}

/// State for the Multiples calculator.
#[derive(Debug, Clone)]
struct MultiplesState {
    numbers_input: String,
    count_input: String,
    result: Option<String>,
}

impl Default for MultiplesState {
    fn default() -> Self {
        Self {
            numbers_input: String::new(),
            count_input: "20".to_string(),
            result: None,
        }
    }
}

/// State for the Prime Numbers calculator.
#[derive(Debug, Clone, Default)]
struct PrimeNumbersState {
    numbers_input: String,
    result: Option<String>,
}

/// State for the Product of Prime Factors calculator.
#[derive(Debug, Clone, Default)]
struct ProdPrimeFactorState {
    number_input: String,
    result: Option<String>,
}

/// State for the Equivalent Fraction calculator.
#[derive(Debug, Clone, Default)]
struct EquivalentFractionState {
    a_input: String,
    b_input: String,
    c_input: String,
    d_input: String,
    result: Option<f64>,
}

/// State for the Simplifying Fractions calculator.
#[derive(Debug, Clone, Default)]
struct SimplifyingFractionsState {
    numerator_input: String,
    denominator_input: String,
    result: Option<String>,
}

/// State for the Mixed Numbers calculator.
#[derive(Debug, Clone, Default)]
struct MixedNumbersState {
    // For Mixed to Improper
    whole_input: String,
    numerator_input: String,
    denominator_input: String,
    result_improper: Option<String>,

    // For Improper to Mixed
    improper_numerator_input: String,
    improper_denominator_input: String,
    result_mixed: Option<String>,
}

/// State for the Ordering Fractions calculator.
#[derive(Debug, Clone, Default)]
struct OrderingFractionsState {
    fractions_input: String,
    result: Option<String>,
}

/// The overall state of our application.
struct MathGui {
    selected_calculator: Option<Calculator>,
    bodmas_state: BodmasState,
    decimal_places_state: DecimalPlacesState,
    hcf_state: HcfState,
    lcm_state: LcmState,
    multiples_state: MultiplesState,
    prime_numbers_state: PrimeNumbersState,
    prod_prime_factor_state: ProdPrimeFactorState,
    equivalent_fraction_state: EquivalentFractionState,
    simplifying_fractions_state: SimplifyingFractionsState,
    mixed_numbers_state: MixedNumbersState,
    ordering_fractions_state: OrderingFractionsState,
}

/// Messages for the BODMAS calculator.
#[derive(Debug, Clone)]
pub enum BodmasMessage {
    ExpressionChanged(String),
    Calculate,
    Reset,
}

/// Messages for the Decimal Places calculator.
#[derive(Debug, Clone)]
pub enum DecimalPlacesMessage {
    NumberInputChanged(String),
    PlacesInputChanged(String),
    Calculate,
    Reset,
}

/// Messages for the HCF calculator.
#[derive(Debug, Clone)]
pub enum HcfMessage {
    NumbersInputChanged(String),
    Calculate,
    Reset,
}

/// Messages for the LCM calculator.
#[derive(Debug, Clone)]
pub enum LcmMessage {
    NumbersInputChanged(String),
    Calculate,
    Reset,
}

/// Messages for the Multiples calculator.
#[derive(Debug, Clone)]
pub enum MultiplesMessage {
    NumbersInputChanged(String),
    CountInputChanged(String),
    Calculate,
    Reset,
}

/// Messages for the Prime Numbers calculator.
#[derive(Debug, Clone)]
pub enum PrimeNumbersMessage {
    NumbersInputChanged(String),
    Calculate,
    Reset,
}

/// Messages for the Product of Prime Factors calculator.
#[derive(Debug, Clone)]
pub enum ProdPrimeFactorMessage {
    NumberInputChanged(String),
    Calculate,
    Reset,
}

/// Messages for the Equivalent Fraction calculator.
#[derive(Debug, Clone)]
pub enum EquivalentFractionMessage {
    AChanged(String),
    BChanged(String),
    CChanged(String),
    DChanged(String),
    Calculate,
    Reset,
}

/// Messages for the Simplifying Fractions calculator.
#[derive(Debug, Clone)]
pub enum SimplifyingFractionsMessage {
    NumeratorChanged(String),
    DenominatorChanged(String),
    Calculate,
    Reset,
}

/// Messages for the Mixed Numbers calculator.
#[derive(Debug, Clone)]
pub enum MixedNumbersMessage {
    WholeChanged(String),
    NumeratorChanged(String),
    DenominatorChanged(String),
    CalculateMixedToImproper,
    ImproperNumeratorChanged(String),
    ImproperDenominatorChanged(String),
    CalculateImproperToMixed,
    Reset,
}

/// Messages for the Ordering Fractions calculator.
#[derive(Debug, Clone)]
pub enum OrderingFractionsMessage {
    FractionsInputChanged(String),
    Calculate,
    Reset,
}

/// The messages that can be sent to update the state.
#[derive(Debug, Clone)]
enum Message {
    CalculatorSelected(Calculator),
    BackToMenu,
    Bodmas(BodmasMessage),
    DecimalPlaces(DecimalPlacesMessage),
    Hcf(HcfMessage),
    Lcm(LcmMessage),
    Multiples(MultiplesMessage),
    PrimeNumbers(PrimeNumbersMessage),
    ProdPrimeFactor(ProdPrimeFactorMessage),
    EquivalentFraction(EquivalentFractionMessage),
    SimplifyingFractions(SimplifyingFractionsMessage),
    MixedNumbers(MixedNumbersMessage),
    OrderingFractions(OrderingFractionsMessage),
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
            bodmas_state: BodmasState::default(),
            decimal_places_state: DecimalPlacesState::default(),
            hcf_state: HcfState::default(),
            lcm_state: LcmState::default(),
            multiples_state: MultiplesState::default(),
            prime_numbers_state: PrimeNumbersState::default(),
            prod_prime_factor_state: ProdPrimeFactorState::default(),
            equivalent_fraction_state: EquivalentFractionState::default(),
            simplifying_fractions_state: SimplifyingFractionsState::default(),
            mixed_numbers_state: MixedNumbersState::default(),
            ordering_fractions_state: OrderingFractionsState::default(),
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
                self.bodmas_state = BodmasState::default();
                self.decimal_places_state = DecimalPlacesState::default();
                self.hcf_state = HcfState::default();
                self.lcm_state = LcmState::default();
                self.multiples_state = MultiplesState::default();
                self.prime_numbers_state = PrimeNumbersState::default();
                self.prod_prime_factor_state = ProdPrimeFactorState::default();
                self.equivalent_fraction_state = EquivalentFractionState::default();
                self.simplifying_fractions_state = SimplifyingFractionsState::default();
                self.mixed_numbers_state = MixedNumbersState::default();
                self.ordering_fractions_state = OrderingFractionsState::default();
            }
            Message::Bodmas(msg) => {
                let state = &mut self.bodmas_state;
                match msg {
                    BodmasMessage::ExpressionChanged(value) => {
                        state.expression = value;
                    }
                    BodmasMessage::Calculate => {
                        match meval::eval_str(&state.expression) {
                            Ok(result) => {
                                // Check if the result is an integer.
                                if result.fract() == 0.0 {
                                    state.result = Some((result as i64).to_string());
                                } else {
                                    state.result = Some(result.to_string());
                                }
                            }
                            Err(e) => {
                                state.result = Some(format!("Error: {}", e));
                            }
                        }
                    }
                    BodmasMessage::Reset => {
                        *state = BodmasState::default();
                    }
                }
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
            Message::Hcf(msg) => {
                let state = &mut self.hcf_state;
                match msg {
                    HcfMessage::NumbersInputChanged(value) => {
                        state.numbers_input = value;
                    }
                    HcfMessage::Calculate => {
                        let numbers: Vec<u32> = state
                            .numbers_input
                            .split_whitespace()
                            .filter_map(|s| s.parse().ok())
                            .collect();

                        if numbers.is_empty() {
                            state.result = Some("Please enter valid numbers.".to_string());
                        } else {
                            let result = numbers
                                .iter()
                                .fold(numbers[0], |acc, &num| hcf(acc, num));
                            state.result = Some(result.to_string());
                        }
                    }
                    HcfMessage::Reset => {
                        *state = HcfState::default();
                    }
                }
            }
            Message::Lcm(msg) => {
                let state = &mut self.lcm_state;
                match msg {
                    LcmMessage::NumbersInputChanged(value) => {
                        state.numbers_input = value;
                    }
                    LcmMessage::Calculate => {
                        let numbers: Vec<u32> = state
                            .numbers_input
                            .split_whitespace()
                            .filter_map(|s| s.parse().ok())
                            .collect();

                        if numbers.is_empty() {
                            state.result = Some("Please enter valid numbers.".to_string());
                        }
                        else {
                            let result = numbers.iter().fold(1, |acc, &num| lcm(acc, num));
                            state.result = Some(result.to_string());
                        }
                    }
                    LcmMessage::Reset => {
                        *state = LcmState::default();
                    }
                }
            }
            Message::Multiples(msg) => {
                let state = &mut self.multiples_state;
                match msg {
                    MultiplesMessage::NumbersInputChanged(value) => {
                        state.numbers_input = value;
                    }
                    MultiplesMessage::CountInputChanged(value) => {
                        state.count_input = value;
                    }
                    MultiplesMessage::Calculate => {
                        let numbers: Vec<u32> = state
                            .numbers_input
                            .split(',')
                            .filter_map(|s| s.trim().parse().ok())
                            .collect();
                        let count: Result<usize, _> = state.count_input.parse();

                        if numbers.is_empty() || count.is_err() {
                            state.result =
                                Some("Please enter valid numbers and a valid count.".to_string());
                        } else {
                            let count = count.unwrap();
                            let mut result_str = String::new();
                            for num in numbers {
                                let multiples = get_multiples(num, count);
                                let multiples_str: Vec<String> =
                                    multiples.iter().map(|m| m.to_string()).collect();
                                result_str.push_str(&format!(
                                    "Multiples of {}: {}
",
                                    num,
                                    multiples_str.join(", ")
                                ));
                            }
                            state.result = Some(result_str);
                        }
                    }
                    MultiplesMessage::Reset => {
                        *state = MultiplesState::default();
                    }
                }
            }
            Message::PrimeNumbers(msg) => {
                let state = &mut self.prime_numbers_state;
                match msg {
                    PrimeNumbersMessage::NumbersInputChanged(value) => {
                        state.numbers_input = value;
                    }
                    PrimeNumbersMessage::Calculate => {
                        let numbers: Vec<u32> = state
                            .numbers_input
                            .split_whitespace()
                            .filter_map(|s| s.parse().ok())
                            .collect();

                        if numbers.is_empty() {
                            state.result = Some("Please enter valid numbers.".to_string());
                        } else {
                            let mut result_str = String::new();
                            for num in numbers {
                                if is_prime(num) {
                                    result_str.push_str(&format!("{} is a prime number.\n", num));
                                } else {
                                    result_str.push_str(&format!(
                                        "{} is not a prime number.\n",
                                        num
                                    ));
                                }
                            }
                            state.result = Some(result_str);
                        }
                    }
                    PrimeNumbersMessage::Reset => {
                        *state = PrimeNumbersState::default();
                    }
                }
            }
            Message::ProdPrimeFactor(msg) => {
                let state = &mut self.prod_prime_factor_state;
                match msg {
                    ProdPrimeFactorMessage::NumberInputChanged(value) => {
                        state.number_input = value;
                    }
                    ProdPrimeFactorMessage::Calculate => {
                        let number: Result<u32, _> = state.number_input.parse();
                        if let Ok(num) = number {
                            if num <= 1 {
                                state.result =
                                    Some("The number must be greater than 1.".to_string());
                            } else {
                                let factors = prime_factorization(num);
                                let result_str = factors
                                    .iter()
                                    .map(|(base, exp)| {
                                        if *exp == 1 {
                                            format!("{}", base)
                                        } else {
                                            format!("{}^{}", base, exp)
                                        }
                                    })
                                    .collect::<Vec<String>>()
                                    .join(" * ");
                                state.result = Some(result_str);
                            }
                        } else {
                            state.result = Some("Please enter a valid number.".to_string());
                        }
                    }
                    ProdPrimeFactorMessage::Reset => {
                        *state = ProdPrimeFactorState::default();
                    }
                }
            }
            Message::EquivalentFraction(msg) => {
                let state = &mut self.equivalent_fraction_state;
                match msg {
                    EquivalentFractionMessage::AChanged(value) => {
                        state.a_input = value;
                    }
                    EquivalentFractionMessage::BChanged(value) => {
                        state.b_input = value;
                    }
                    EquivalentFractionMessage::CChanged(value) => {
                        state.c_input = value;
                    }
                    EquivalentFractionMessage::DChanged(value) => {
                        state.d_input = value;
                    }
                    EquivalentFractionMessage::Calculate => {
                        let a = state.a_input.parse::<f64>();
                        let b = state.b_input.parse::<f64>();
                        let c = state.c_input.parse::<f64>();
                        let d = state.d_input.parse::<f64>();

                        if a.is_err() {
                            let b_val = b.unwrap_or(0.0);
                            let c_val = c.unwrap_or(0.0);
                            let d_val = d.unwrap_or(0.0);
                            if d_val != 0.0 {
                                state.result = Some((b_val * c_val) / d_val);
                            } else {
                                state.result = None; // Division by zero
                            }
                        } else if b.is_err() {
                            let a_val = a.unwrap_or(0.0);
                            let c_val = c.unwrap_or(0.0);
                            let d_val = d.unwrap_or(0.0);
                            if c_val != 0.0 {
                                state.result = Some((a_val * d_val) / c_val);
                            } else {
                                state.result = None; // Division by zero
                            }
                        } else if c.is_err() {
                            let a_val = a.unwrap_or(0.0);
                            let b_val = b.unwrap_or(0.0);
                            let d_val = d.unwrap_or(0.0);
                            if b_val != 0.0 {
                                state.result = Some((a_val * d_val) / b_val);
                            } else {
                                state.result = None; // Division by zero
                            }
                        } else if d.is_err() {
                            let a_val = a.unwrap_or(0.0);
                            let b_val = b.unwrap_or(0.0);
                            let c_val = c.unwrap_or(0.0);
                            if a_val != 0.0 {
                                state.result = Some((b_val * c_val) / a_val);
                            } else {
                                state.result = None; // Division by zero
                            }
                        } else {
                            state.result = None; // No 'x' found
                        }
                    }
                    EquivalentFractionMessage::Reset => {
                        *state = EquivalentFractionState::default();
                    }
                }
            }
            Message::SimplifyingFractions(msg) => {
                let state = &mut self.simplifying_fractions_state;
                match msg {
                    SimplifyingFractionsMessage::NumeratorChanged(value) => {
                        state.numerator_input = value;
                    }
                    SimplifyingFractionsMessage::DenominatorChanged(value) => {
                        state.denominator_input = value;
                    }
                    SimplifyingFractionsMessage::Calculate => {
                        let numerator: Result<u32, _> = state.numerator_input.parse();
                        let denominator: Result<u32, _> = state.denominator_input.parse();

                        if let (Ok(mut num), Ok(mut den)) = (numerator, denominator) {
                            if den == 0 {
                                state.result = Some("Error: Denominator cannot be zero.".to_string());
                            } else {
                                let common_factor = hcf(num, den);
                                num /= common_factor;
                                den /= common_factor;
                                state.result = Some(format!("{}/{}", num, den));
                            }
                        } else {
                            state.result = Some("Please enter valid numbers.".to_string());
                        }
                    }
                    SimplifyingFractionsMessage::Reset => {
                        *state = SimplifyingFractionsState::default();
                    }
                }
            }
            Message::MixedNumbers(msg) => {
                let state = &mut self.mixed_numbers_state;
                match msg {
                    MixedNumbersMessage::WholeChanged(value) => {
                        state.whole_input = value;
                    }
                    MixedNumbersMessage::NumeratorChanged(value) => {
                        state.numerator_input = value;
                    }
                    MixedNumbersMessage::DenominatorChanged(value) => {
                        state.denominator_input = value;
                    }
                    MixedNumbersMessage::CalculateMixedToImproper => {
                        let whole: Result<u32, _> = state.whole_input.parse();
                        let numerator: Result<u32, _> = state.numerator_input.parse();
                        let denominator: Result<u32, _> = state.denominator_input.parse();

                        if let (Ok(w), Ok(n), Ok(d)) = (whole, numerator, denominator) {
                            if d == 0 {
                                state.result_improper = Some("Error: Denominator cannot be zero.".to_string());
                            } else {
                                let improper_numerator = w * d + n;
                                state.result_improper = Some(format!("{}/{}", improper_numerator, d));
                            }
                        } else {
                            state.result_improper = Some("Please enter valid numbers.".to_string());
                        }
                    }
                    MixedNumbersMessage::ImproperNumeratorChanged(value) => {
                        state.improper_numerator_input = value;
                    }
                    MixedNumbersMessage::ImproperDenominatorChanged(value) => {
                        state.improper_denominator_input = value;
                    }
                    MixedNumbersMessage::CalculateImproperToMixed => {
                        let numerator: Result<u32, _> = state.improper_numerator_input.parse();
                        let denominator: Result<u32, _> = state.improper_denominator_input.parse();

                        if let (Ok(n), Ok(d)) = (numerator, denominator) {
                            if d == 0 {
                                state.result_mixed = Some("Error: Denominator cannot be zero.".to_string());
                            } else if n < d {
                                state.result_mixed = Some(format!("{} is not an improper fraction.", format!("{}/{}", n, d)));
                            } else {
                                let whole_part = n / d;
                                let remainder_numerator = n % d;
                                if remainder_numerator == 0 {
                                    state.result_mixed = Some(format!("{}", whole_part));
                                } else {
                                    state.result_mixed = Some(format!("{}({}/{})", whole_part, remainder_numerator, d));
                                }
                            }
                        } else {
                            state.result_mixed = Some("Please enter valid numbers.".to_string());
                        }
                    }
                    MixedNumbersMessage::Reset => {
                        *state = MixedNumbersState::default();
                    }
                }
            }
            Message::OrderingFractions(msg) => {
                let state = &mut self.ordering_fractions_state;
                match msg {
                    OrderingFractionsMessage::FractionsInputChanged(value) => {
                        state.fractions_input = value;
                    }
                    OrderingFractionsMessage::Calculate => {
                        let fractions_str: Vec<&str> = state.fractions_input.split(',').collect();
                        let mut fractions: Vec<(u32, u32)> = Vec::new();

                        for f_str in fractions_str {
                            let parts: Vec<&str> = f_str.trim().split('/').collect();
                            if parts.len() == 2 {
                                if let (Ok(num), Ok(den)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                                    if den == 0 {
                                        state.result = Some(format!("Error: Denominator cannot be zero for fraction {}.", f_str));
                                        fractions.clear();
                                        break;
                                    }
                                    fractions.push((num, den));
                                } else {
                                    state.result = Some(format!("Error: Invalid number in fraction {}.", f_str));
                                    fractions.clear();
                                    break;
                                }
                            } else {
                                state.result = Some(format!("Error: Invalid fraction format {}. Expected numerator/denominator.", f_str));
                                fractions.clear();
                                break;
                            }
                        }

                        if fractions.is_empty() {
                            state.result = Some("Please enter valid fractions.".to_string());
                        }
                        else {
                            let mut common_denominator = fractions[0].1;
                            for i in 1..fractions.len() {
                                common_denominator = lcm(common_denominator, fractions[i].1);
                            }

                            let mut result_str = String::new();
                            result_str.push_str(&format!("Common Denominator: {}
", common_denominator));
                            result_str.push_str("Rewritten Fractions:\n");

                            for (num, den) in fractions {
                                let multiplier = common_denominator / den;
                                let new_numerator = num * multiplier;
                                result_str.push_str(&format!("  {}/{} becomes {}/{}\n", num, den, new_numerator, common_denominator));
                            }
                            state.result = Some(result_str);
                        }
                    }
                    OrderingFractionsMessage::Reset => {
                        *state = OrderingFractionsState::default();
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = if let Some(calculator) = self.selected_calculator {
            // --- View for a selected calculator ---
            let view_content = match calculator {
                Calculator::Bodmas => {
                    let state = &self.bodmas_state;
                    let result_text = match &state.result {
                        Some(res) => format!("Result: {}", res),
                        None => "Enter an expression.".to_string(),
                    };

                    column![
                        text(calculator.name()).size(30),
                        text_input("Enter expression (e.g., 2 * (3 + 4))", &state.expression)
                            .on_input(|s| Message::Bodmas(BodmasMessage::ExpressionChanged(s))),
                        row![
                            button("Calculate").on_press(Message::Bodmas(BodmasMessage::Calculate)),
                            button("Reset").on_press(Message::Bodmas(BodmasMessage::Reset)),
                        ]
                        .spacing(10),
                        text(result_text).size(25),
                        button("Back").on_press(Message::BackToMenu),
                    ]
                }
                Calculator::DecimalPlaces => {
                    let state = &self.decimal_places_state;
                    let result_text = match state.result {
                        Some(res) => format!("Result: {}", res),
                        None => "Please enter valid numbers.".to_string(),
                    };

                    column![
                        text(calculator.name()).size(30),
                        text_input("Number to round", &state.number_input)
                            .on_input(|s| {
                                Message::DecimalPlaces(DecimalPlacesMessage::NumberInputChanged(s))
                            }),
                        text_input("Decimal places", &state.places_input)
                            .on_input(|s| {
                                Message::DecimalPlaces(DecimalPlacesMessage::PlacesInputChanged(s))
                            }),
                        row![
                            button("Calculate")
                                .on_press(Message::DecimalPlaces(DecimalPlacesMessage::Calculate)),
                            button("Reset")
                                .on_press(Message::DecimalPlaces(DecimalPlacesMessage::Reset)),
                        ]
                        .spacing(10),
                        text(result_text).size(25),
                        button("Back").on_press(Message::BackToMenu),
                    ]
                }
                Calculator::Hcf => {
                    let state = &self.hcf_state;
                    let result_text = match &state.result {
                        Some(res) => format!("Result: {}", res),
                        None => "Enter numbers separated by spaces.".to_string(),
                    };

                    column![
                        text(calculator.name()).size(30),
                        text_input("Numbers", &state.numbers_input)
                            .on_input(|s| Message::Hcf(HcfMessage::NumbersInputChanged(s))),
                        row![
                            button("Calculate").on_press(Message::Hcf(HcfMessage::Calculate)),
                            button("Reset").on_press(Message::Hcf(HcfMessage::Reset)),
                        ]
                        .spacing(10),
                        text(result_text).size(25),
                        button("Back").on_press(Message::BackToMenu),
                    ]
                }
                Calculator::Lcm => {
                    let state = &self.lcm_state;
                    let result_text = match &state.result {
                        Some(res) => format!("Result: {}", res),
                        None => "Enter numbers separated by spaces.".to_string(),
                    };

                    column![
                        text(calculator.name()).size(30),
                        text_input("Numbers", &state.numbers_input)
                            .on_input(|s| Message::Lcm(LcmMessage::NumbersInputChanged(s))),
                        row![
                            button("Calculate").on_press(Message::Lcm(LcmMessage::Calculate)),
                            button("Reset").on_press(Message::Lcm(LcmMessage::Reset)),
                        ]
                        .spacing(10),
                        text(result_text).size(25),
                        button("Back").on_press(Message::BackToMenu),
                    ]
                }
                Calculator::Multiples => {
                    let state = &self.multiples_state;
                    let result_text = match &state.result {
                        Some(res) => res.clone(),
                        None => "Enter numbers and a count.".to_string(),
                    };

                    column![
                        text(calculator.name()).size(30),
                        text_input(
                            "Numbers (comma-separated, e.g., 3, 5, 8)",
                            &state.numbers_input
                        )
                        .on_input(|s| Message::Multiples(
                            MultiplesMessage::NumbersInputChanged(s)
                        )),
                        text_input("Number of multiples", &state.count_input).on_input(|s| {
                            Message::Multiples(MultiplesMessage::CountInputChanged(s))
                        }),
                        row![
                            button("Calculate")
                                .on_press(Message::Multiples(MultiplesMessage::Calculate)),
                            button("Reset")
                                .on_press(Message::Multiples(MultiplesMessage::Reset)),
                        ]
                        .spacing(10),
                        text(result_text).size(25),
                        button("Back").on_press(Message::BackToMenu),
                    ]
                }
                Calculator::PrimeNumbers => {
                    let state = &self.prime_numbers_state;
                    let result_text = match &state.result {
                        Some(res) => res.clone(),
                        None => "Enter numbers separated by spaces.".to_string(),
                    };

                    column![
                        text(calculator.name()).size(30),
                        text_input("Numbers", &state.numbers_input).on_input(|s| {
                            Message::PrimeNumbers(PrimeNumbersMessage::NumbersInputChanged(s))
                        }),
                        row![
                            button("Calculate")
                                .on_press(Message::PrimeNumbers(PrimeNumbersMessage::Calculate)),
                            button("Reset")
                                .on_press(Message::PrimeNumbers(PrimeNumbersMessage::Reset)),
                        ]
                        .spacing(10),
                        text(result_text).size(25),
                        button("Back").on_press(Message::BackToMenu),
                    ]
                }
                Calculator::ProdPrimeFactor => {
                    let state = &self.prod_prime_factor_state;
                    let result_text = match &state.result {
                        Some(res) => format!("Result: {}", res),
                        None => "Enter a number.".to_string(),
                    };

                    column![
                        text(calculator.name()).size(30),
                        text_input("Number", &state.number_input).on_input(|s| {
                            Message::ProdPrimeFactor(ProdPrimeFactorMessage::NumberInputChanged(s))
                        }),
                        row![
                            button("Calculate").on_press(Message::ProdPrimeFactor(
                                ProdPrimeFactorMessage::Calculate
                            )),
                            button("Reset").on_press(Message::ProdPrimeFactor(
                                ProdPrimeFactorMessage::Reset
                            )),
                        ]
                        .spacing(10),
                        text(result_text).size(25),
                        button("Back").on_press(Message::BackToMenu),
                    ]
                }
                Calculator::EquivalentFraction => {
                    let state = &self.equivalent_fraction_state;
                    let result_text = match state.result {
                        Some(res) => format!("Result: {}", res),
                        None => "Enter three values and 'x' for the unknown.".to_string(),
                    };

                    column![
                        text(calculator.name()).size(30),
                        row![
                            text_input("a", &state.a_input)
                                .on_input(|s| Message::EquivalentFraction(EquivalentFractionMessage::AChanged(s))),
                            text("/"),
                            text_input("b", &state.b_input)
                                .on_input(|s| Message::EquivalentFraction(EquivalentFractionMessage::BChanged(s))),
                            text("="),
                            text_input("c", &state.c_input)
                                .on_input(|s| Message::EquivalentFraction(EquivalentFractionMessage::CChanged(s))),
                            text("/"),
                            text_input("d", &state.d_input)
                                .on_input(|s| Message::EquivalentFraction(EquivalentFractionMessage::DChanged(s))),
                        ]
                        .spacing(10)
                        .align_items(Alignment::Center),
                        row![
                            button("Calculate").on_press(Message::EquivalentFraction(EquivalentFractionMessage::Calculate)),
                            button("Reset").on_press(Message::EquivalentFraction(EquivalentFractionMessage::Reset)),
                        ]
                        .spacing(10),
                        text(result_text).size(25),
                        button("Back").on_press(Message::BackToMenu),
                    ]
                }
                Calculator::SimplifyingFractions => {
                    let state = &self.simplifying_fractions_state;
                    let result_text = match &state.result {
                        Some(res) => format!("Result: {}", res),
                        None => "Enter numerator and denominator.".to_string(),
                    };

                    column![
                        text(calculator.name()).size(30),
                        row![
                            text_input("Numerator", &state.numerator_input)
                                .on_input(|s| Message::SimplifyingFractions(SimplifyingFractionsMessage::NumeratorChanged(s))),
                            text("/"),
                            text_input("Denominator", &state.denominator_input)
                                .on_input(|s| Message::SimplifyingFractions(SimplifyingFractionsMessage::DenominatorChanged(s))),
                        ]
                        .spacing(10)
                        .align_items(Alignment::Center),
                        row![
                            button("Calculate").on_press(Message::SimplifyingFractions(SimplifyingFractionsMessage::Calculate)),
                            button("Reset").on_press(Message::SimplifyingFractions(SimplifyingFractionsMessage::Reset)),
                        ]
                        .spacing(10),
                        text(result_text).size(25),
                        button("Back").on_press(Message::BackToMenu),
                    ]
                }
                Calculator::MixedNumbers => {
                    let state = &self.mixed_numbers_state;

                    let mixed_to_improper_result_text = match &state.result_improper {
                        Some(res) => format!("Improper Fraction: {}", res),
                        None => "Enter whole number, numerator, and denominator.".to_string(),
                    };

                    let improper_to_mixed_result_text = match &state.result_mixed {
                        Some(res) => format!("Mixed Number: {}", res),
                        None => "Enter improper fraction numerator and denominator.".to_string(),
                    };

                    column![
                        text(calculator.name()).size(30),
                        // Mixed to Improper Section
                        text("Convert Mixed Number to Improper Fraction").size(20),
                        row![
                            text_input("Whole", &state.whole_input)
                                .on_input(|s| Message::MixedNumbers(MixedNumbersMessage::WholeChanged(s))),
                            text_input("Numerator", &state.numerator_input)
                                .on_input(|s| Message::MixedNumbers(MixedNumbersMessage::NumeratorChanged(s))),
                            text("/"),
                            text_input("Denominator", &state.denominator_input)
                                .on_input(|s| Message::MixedNumbers(MixedNumbersMessage::DenominatorChanged(s))),
                        ]
                        .spacing(10)
                        .align_items(Alignment::Center),
                        row![
                            button("Calculate Improper").on_press(Message::MixedNumbers(MixedNumbersMessage::CalculateMixedToImproper)),
                            button("Reset").on_press(Message::MixedNumbers(MixedNumbersMessage::Reset)),
                        ]
                        .spacing(10),
                        text(mixed_to_improper_result_text).size(25),

                        // Improper to Mixed Section
                        text("Convert Improper Fraction to Mixed Number").size(20),
                        row![
                            text_input("Numerator", &state.improper_numerator_input)
                                .on_input(|s| Message::MixedNumbers(MixedNumbersMessage::ImproperNumeratorChanged(s))),
                            text("/"),
                            text_input("Denominator", &state.improper_denominator_input)
                                .on_input(|s| Message::MixedNumbers(MixedNumbersMessage::ImproperDenominatorChanged(s))),
                        ]
                        .spacing(10)
                        .align_items(Alignment::Center),
                        row![
                            button("Calculate Mixed").on_press(Message::MixedNumbers(MixedNumbersMessage::CalculateImproperToMixed)),
                        ]
                        .spacing(10),
                        text(improper_to_mixed_result_text).size(25),

                        button("Back").on_press(Message::BackToMenu),
                    ]
                }
                Calculator::OrderingFractions => {
                    let state = &self.ordering_fractions_state;
                    let result_text = match &state.result {
                        Some(res) => res.clone(),
                        None => "Enter fractions separated by commas (e.g., 5/6, 3/8).".to_string(),
                    };

                    column![
                        text(calculator.name()).size(30),
                        text_input("Fractions (e.g., 5/6, 3/8)", &state.fractions_input)
                            .on_input(|s| Message::OrderingFractions(OrderingFractionsMessage::FractionsInputChanged(s))),
                        row![
                            button("Calculate").on_press(Message::OrderingFractions(OrderingFractionsMessage::Calculate)),
                            button("Reset").on_press(Message::OrderingFractions(OrderingFractionsMessage::Reset)),
                        ]
                        .spacing(10),
                        text(result_text).size(25),
                        button("Back").on_press(Message::BackToMenu),
                    ]
                }
                _ => column![
                    text(calculator.name()).size(30),
                    text("This calculator has not been implemented in the GUI yet."),
                    button("Back").on_press(Message::BackToMenu),
                ],
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