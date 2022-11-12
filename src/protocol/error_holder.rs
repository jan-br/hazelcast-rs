use crate::protocol::stack_trace_element::StackTraceElement;

pub struct ErrorHolder {
  pub error_code: i32,
  pub class_name: String,
  pub message: Option<String>,
  pub stack_trace_elements: Vec<StackTraceElement>
}