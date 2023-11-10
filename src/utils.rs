use inquire::{required, CustomType, Text};

pub fn prompt_text(message: &str, required: bool, initial: Option<&str>) -> String {
    let mut prompt_input: Text<'_> = Text::new(message);
    if required {
        prompt_input = prompt_input.with_validator(required!());
    }
    if let Some(value) = initial {
        prompt_input = prompt_input.with_initial_value(value);
    }

    prompt_input.prompt().unwrap()
}

pub fn prompt_number(message: &str, initial: Option<i32>) -> i32 {
    let mut prompt_input =
        CustomType::new(message).with_error_message("Please type a valid number");
    if let Some(value) = initial {
        prompt_input = prompt_input.with_default(value)
    }

    prompt_input.prompt().unwrap()
}

pub fn prompt_amount(message: &str, initial: Option<f64>) -> f64 {
    let mut prompt_input = CustomType::new(message)
        .with_error_message("Please type a valid number")
        .with_formatter(&|i: f64| format!("${i}"));

    if let Some(value) = initial {
        prompt_input = prompt_input.with_default(value)
    }

    prompt_input.prompt().unwrap()
}
