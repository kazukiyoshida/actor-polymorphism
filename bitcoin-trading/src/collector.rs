use crate::bbo_handler::*;
use actix::*;

pub struct Collector;

impl Actor for Collector {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("Collector : I am alive!");
    }
}

impl Handler<UpdateBBO> for Collector {
    type Result = ();

    fn handle(&mut self, msg: UpdateBBO, _: &mut Context<Self>) -> Self::Result {
        println!("###### update BBO -> {:?}", msg);
    }
}
