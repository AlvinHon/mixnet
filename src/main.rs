use mixnet::mixnet::{Mixnet, MixnetConfig};
use mixnet::preliminaries::SecurityParams;

fn main() {
    let params = SecurityParams::baseline();
    let config = MixnetConfig::new(params).expect("baseline params should validate");
    let instance = Mixnet::new(config);

    println!("mixnet scaffold initialized: {:?}", instance.config().params);
}
