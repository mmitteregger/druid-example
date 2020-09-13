use druid::widget::Label;
use druid::Widget;

use crate::data::{View1Data, View2Data};

pub struct View1;
impl View1 {
    pub fn new() -> impl Widget<View1Data> {
        Label::dynamic(|data: &View1Data, _| format!("{}", data.text))
    }
}

pub struct View2;
impl View2 {
    pub fn new() -> impl Widget<View2Data> {
        Label::dynamic(|data: &View2Data, _| format!("{}", data.text))
    }
}
