use actix::prelude::*;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Clock;