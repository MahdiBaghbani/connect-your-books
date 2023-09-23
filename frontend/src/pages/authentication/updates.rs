use seed::prelude::*;

use crate::pages::authentication::models;

// `Msg` describes the different events you can modify state with.
pub enum Msg {}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut models::Model, orders: &mut impl Orders<Msg>) {}