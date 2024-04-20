use actix::prelude::*;

use crate::cycle::Cycle;

use super::memory::{ReadRespMessage, WriteRespMessage};

pub trait MemCache:
    Handler<ReadRespMessage> + Handler<WriteRespMessage> + Handler<Cycle> + Actor
{
}

pub struct L2Cache {}

impl Actor for L2Cache {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {}
}

impl Handler<WriteRespMessage> for L2Cache {
    type Result = ();

    fn handle(&mut self, msg: WriteRespMessage, ctx: &mut Self::Context) {
        println!("MESSAGE RECIEVED: {:?}", msg);
    }
}

impl Handler<ReadRespMessage> for L2Cache {
    type Result = ();

    fn handle(&mut self, msg: ReadRespMessage, ctx: &mut Self::Context) {
        println!("MESSAGE RECIEVED: {:?}", msg);
    }
}

impl Handler<Cycle> for L2Cache {
    type Result = ();

    fn handle(&mut self, msg: Cycle, ctx: &mut Self::Context) {}
}
