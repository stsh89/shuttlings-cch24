use crate::core::{definitions::Route, operations::RouteFragment};

pub trait NetworkService {}

pub trait RestoreRoute: NetworkService {
    fn restore_route(&self, fragment: RouteFragment) -> Route;
}
