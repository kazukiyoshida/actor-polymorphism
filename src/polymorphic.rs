#![feature(trait_alias)]

use actix::dev::ToEnvelope;
use actix::prelude::*;

//
// Messenger アクターは MsgHandler を実装した任意のアクターにメッセージを送る
//
//  Messenger  -- Msg --> MsgHandler を実装した任意のアクター
//

//
// Messenger アクター
//
struct Messenger<T: MsgHandler>(Addr<T>);

impl<T> Actor for Messenger<T>
where
    T: MsgHandler,
    T::Context: ToEnvelope<T, Msg>,
{
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("Messenger : start");
        println!("Messenger : send message");
        self.0.do_send(Msg("Hello!!".to_string()));
    }
}

//
// Receiver アクター
// このアクターは MsgHandler を実装する.
//
struct Receiver;

impl Actor for Receiver {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("Receiver : start");
    }
}

// トレイト・エイリアス
trait MsgHandler = actix::Actor + Handler<Msg>;

#[derive(Message)]
#[rtype(result = "()")]
struct Msg(String);

impl Handler<Msg> for Receiver {
    type Result = ();

    fn handle(&mut self, msg: Msg, _: &mut Context<Self>) -> Self::Result {
        println!("Receiver : got a message! >> {:?}", msg.0);
    }
}

//
// main 関数
//
fn main() {
    let mut sys = System::new("sys");

    let addr_receiver = sys.block_on(async { Receiver.start() });
    let addr_messenger = sys.block_on(async { Messenger(addr_receiver).start() });

    sys.run();
}
