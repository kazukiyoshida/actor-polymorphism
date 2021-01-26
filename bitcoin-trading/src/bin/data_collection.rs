use actix::*;
use actor_polymorphism::{bitflyer::*, collector::*, strategy::*};

// Bitflyer BTC のリアルタイムデータを収集する
fn data_collection() {
    let mut sys = actix::System::new("sys");
    let collector_addr = sys.block_on(async { Collector.start() });
    let addr = sys.block_on(async { Bitflyer::new(collector_addr).await });
    sys.run();
}
