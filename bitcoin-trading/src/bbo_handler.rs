use actix::*;

pub trait BBOHandler = actix::Actor + Handler<UpdateBBO>;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct UpdateBBO {
    pub bbo: BBO,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BBO {
    pub bid: (f64, f64), // price, size
    pub ask: (f64, f64), // price, size
}
