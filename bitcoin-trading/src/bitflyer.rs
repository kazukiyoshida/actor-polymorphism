use crate::bbo_handler::*;
use crate::websocket::*;
use actix::{dev::ToEnvelope, io::SinkWrite, *};
use awc::{
    error::WsProtocolError,
    ws::{Frame, Message},
};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::VecDeque;

// Bitflyer Realtime API の WebSocket エンドポイント
pub const WEBSOCKET_ENDPOINT: &str = "wss://ws.lightstream.bitflyer.com/json-rpc";

// 「BTC 現物」の板情報スナップショットを購読するためのメッセージ
pub const SUBSCRIBE_MSG: &str = r###"{
    "method":"subscribe",
    "params":{"channel": "lightning_board_snapshot_BTC_JPY"}
}"###;

// Bitflyer 構造体
// Bitflyer が WebSocket で配信しているリアルタイムデータを取得する.
// データを受信したらそれをサービスアクターT に転送する.
pub struct Bitflyer<T: BBOHandler> {
    sender: Sender,
    service_actor: Addr<T>,
}

// Bitflyer 構造体に Actix のアクターを実装する
impl<T: BBOHandler> Actor for Bitflyer<T> {
    type Context = Context<Self>;
}

impl<T> Bitflyer<T>
where
    T: BBOHandler,
    T::Context: ToEnvelope<T, UpdateBBO>,
{
    // Bitflyer アクターを生成・起動する
    pub async fn new(service_actor: Addr<T>) -> Result<Addr<Bitflyer<T>>, ()> {
        match new_websocket_client()
            .ws(WEBSOCKET_ENDPOINT)
            .connect()
            .await
        {
            Ok((_, framed)) => {
                let (sink, stream) = framed.split();
                let conn = Bitflyer::create(|ctx| {
                    Bitflyer::add_stream(stream, ctx);
                    Bitflyer {
                        sender: SinkWrite::new(sink, ctx),
                        service_actor,
                    }
                });
                Ok(conn)
            }
            Err(_) => Err(()),
        }
    }
}

// Bitflyer アクターに WebSocket ストリームハンドラを実装する
impl<T> StreamHandler<Result<Frame, WsProtocolError>> for Bitflyer<T>
where
    T: BBOHandler,
    T::Context: ToEnvelope<T, UpdateBBO>,
{
    // WebSocket メッセージを受信した際の振る舞い
    fn handle(&mut self, msg: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
        match msg {
            Ok(Frame::Text(msg)) => {
                let msg = String::from_utf8(msg.to_vec()).unwrap();
                if let Some(msg) = serde_json::from_str::<Msg>(&msg).ok() {
                    let mut bid_tree = BTreeMap::new();
                    for bid in &msg.params.message.bids {
                        bid_tree.insert(bid.price as u64, bid.size);
                    }

                    let mut ask_tree = BTreeMap::new();
                    for ask in &msg.params.message.asks {
                        ask_tree.insert(ask.price as u64, ask.size);
                    }

                    println!("{:?}", bid_tree);

                    // let orderbook = msg.params.message.create_orderbook();
                    // self.service_actor
                    //     .do_send(UpdateOrderbook { asset, orderbook });
                }
            }
            _ => println!("undefined message: {:?}", msg),
        }
    }

    // WebSocket 接続が確立した際の振る舞い.
    // 接続が確立したらリアルタイムデータを購読するためのメッセージを
    // Bitflyer サーバーに送信する.
    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("connected");
        self.sender.write(Message::Text(SUBSCRIBE_MSG.to_string()));
    }
}

impl<T: BBOHandler> actix::io::WriteHandler<WsProtocolError> for Bitflyer<T> {}

//
// Bitflyer の WebSocket メッセージのデータ構造
//
#[derive(Serialize, Deserialize, Debug)]
pub struct Msg {
    pub params: Params,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    pub message: OrderbookMsg,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookMsg {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub price: f64,
    pub size: f64,
}
