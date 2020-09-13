pub use app::*;
pub use views::*;

mod app;
mod views;

use druid::Data;
use std::any::Any;
use std::fmt::Debug;

pub trait ViewData: Debug + CloneViewData + AnyData + DynData {}
impl<T> ViewData for T where T: Debug + Data + 'static {}

pub trait CloneViewData {
    fn clone_view_data(&self) -> Box<dyn ViewData>;
}
impl<T> CloneViewData for T
where
    T: ViewData + Clone + 'static,
{
    fn clone_view_data(&self) -> Box<dyn ViewData> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn ViewData> {
    fn clone(&self) -> Self {
        self.clone_view_data()
    }
}

pub trait AnyData {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
impl<T> AnyData for T
where
    T: 'static,
{
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

// dyn trait `druid::Data` functions
pub trait DynData {
    fn same(&self, other: &dyn ViewData) -> bool;
}

impl<T> DynData for T
where
    T: ViewData + Data + 'static,
{
    fn same(&self, other: &dyn ViewData) -> bool {
        match (
            self.as_any().downcast_ref::<T>(),
            other.as_any().downcast_ref::<T>(),
        ) {
            (Some(data), Some(other_data)) => druid::Data::same(data, other_data),
            _ => false,
        }
    }
}
