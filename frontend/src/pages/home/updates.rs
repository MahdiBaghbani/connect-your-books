use seed::prelude::*;

use crate::pages::home::models;

pub enum Msg {}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut models::Model, orders: &mut impl Orders<Msg>) {}
