#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::with_capacity(4);
    for input in inputs {
        let res = eval_input(input, &mut stack)?;
        stack.push(res);
    }
    if stack.len() == 1 {
        return Some(stack[0])
    }
    None
}

fn eval_input(input: &CalculatorInput, stack: &mut Vec<i32>) -> Option<i32> {
    match input {
        CalculatorInput::Add => return Some(stack.pop()? + stack.pop()?),
        CalculatorInput::Subtract => return Some(-stack.pop()? + stack.pop()?),
        CalculatorInput::Multiply => return Some(stack.pop()? * stack.pop()?),
        CalculatorInput::Divide => {
            let bottom = stack.pop()?;
            return stack.pop()?.checked_div(bottom);
        }
        CalculatorInput::Value(val) => return Some(*val),
    }

}