use crate::Route;
use yew::Html;

#[derive(Clone, PartialEq)]
pub struct NavGroup {
    pub name: &'static str,
    pub icon: &'static str,
    pub items: Vec<NavItem>,
    pub extras: Option<Vec<NavExtra>>,
}

#[derive(Clone, PartialEq)]
pub struct NavItem {
    pub name: &'static str,
    pub route: Route,
    pub icon: &'static str,
}

#[derive(Clone, PartialEq)]
pub enum NavExtra {
    Component(Html),
}
