#[derive(serde::Serialize)]
pub struct XResponse<X> {
    pub message: Option<String>,
    pub data: Option<X>
}

impl<X> XResponse<X> {
    pub fn new(message: impl ToString, data: X) -> Self {
        XResponse { message: Some(message.to_string()), data: Some(data)}
    }
    pub fn without_data(message: impl ToString) -> Self {
        XResponse { message: Some(message.to_string()), data: None}
    }
    pub fn without_message(data: X) -> Self {
        XResponse { message: None, data: Some(data)}
    }
}
