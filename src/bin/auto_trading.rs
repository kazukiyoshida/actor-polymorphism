use actix::*;
use actor_polymorphism::{bitflyer::*, collector::*, strategy::*};

// Bitflyer BTC のリアルタイムデータをストラテジーに入力し、自動取引を実行する
fn auto_trading() {
    let mut sys = actix::System::new("sys");
    let strategy_addr = sys.block_on(async { Strategy.start() });
    let addr = sys.block_on(async { Bitflyer::new(strategy_addr).await });
    sys.run();
}
