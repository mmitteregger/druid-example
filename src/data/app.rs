use std::fmt::Debug;

use druid::{Data, Lens};

use crate::data::{View1Data, View2Data, ViewData};

#[derive(Debug, Clone)]
pub struct AppData {
    view: AppView,
    view_data: Box<dyn ViewData>,
    // ... plus some settings and other stuff
}

impl Data for AppData {
    fn same(&self, other: &Self) -> bool {
        self.view == other.view && self.view_data.same(&*other.view_data)
    }
}

impl AppData {
    pub const VIEW: ViewLens = ViewLens;
    pub const VIEW_DATA: ViewDataLens = ViewDataLens;
}

impl AppData {
    pub fn new() -> AppData {
        AppData {
            view: AppView::View1,
            view_data: Box::new(View1Data::new()),
        }
    }

    pub fn view(&self) -> AppView {
        self.view
    }

    pub fn set_view(&mut self, view: AppView) {
        self.view_data = match view {
            AppView::View1 => Box::new(View1Data::new()),
            AppView::View2 => Box::new(View2Data::new()),
        };
        self.view = view;
    }
}

#[derive(Debug, Data, Copy, Clone, Eq, PartialEq)]
pub enum AppView {
    View1,
    View2,
    // ... you get the idea
}

pub struct ViewLens;

impl Lens<AppData, AppView> for ViewLens {
    fn with<V, F: FnOnce(&AppView) -> V>(&self, data: &AppData, f: F) -> V {
        f(&data.view)
    }

    fn with_mut<V, F: FnOnce(&mut AppView) -> V>(&self, data: &mut AppData, f: F) -> V {
        let mut view = data.view.clone();
        let value = f(&mut view);
        data.view = view;
        value
    }
}

pub struct ViewDataLens;

impl<T: Data> Lens<AppData, T> for ViewDataLens {
    fn with<V, F: FnOnce(&T) -> V>(&self, data: &AppData, f: F) -> V {
        match data.view_data.as_any().downcast_ref::<T>() {
            Some(data) => f(data),
            None => panic!(
                "attempted to downcast {:?} to {}",
                &data.view_data,
                std::any::type_name::<T>()
            ),
        }
    }

    fn with_mut<V, F: FnOnce(&mut T) -> V>(&self, data: &mut AppData, f: F) -> V {
        match data.view_data.as_any_mut().downcast_mut::<T>() {
            Some(data) => f(data),
            None => panic!(
                "attempted to downcast {:?} to {}",
                &data.view_data,
                std::any::type_name::<T>()
            ),
        }
    }
}
