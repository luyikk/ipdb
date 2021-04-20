use serde::{Deserialize};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Deserialize,Debug)]
pub struct Meta{
    pub build:i32,
    pub ip_version:i32,
    pub node_count:usize,
    pub languages:HashMap<String,usize>,
    pub fields:Vec<String>,
    pub total_size:usize
}