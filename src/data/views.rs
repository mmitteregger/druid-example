use druid::{Data, Lens};

#[derive(Debug, Clone, Data, Lens)]
pub struct View1Data {
    pub text: String,
}

impl View1Data {
    pub fn new() -> View1Data {
        View1Data {
            text: String::from("This is view #1"),
        }
    }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct View2Data {
    pub text: String,
}

impl View2Data {
    pub fn new() -> View2Data {
        View2Data {
            text: String::from("This is view #2"),
        }
    }
}
