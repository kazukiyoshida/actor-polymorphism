use crate::bbo_handler::*;
use actix::*;

pub struct Strategy;

impl Actor for Strategy {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("Strategy : I am alive!");
    }
}

impl Handler<UpdateBBO> for Strategy {
    type Result = ();

    fn handle(&mut self, msg: UpdateBBO, _: &mut Context<Self>) -> Self::Result {
        println!("###### update BBO -> {:?}", msg);
    }
}
