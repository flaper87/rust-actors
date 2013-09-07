extern mod actors;

use actors::actor::{Actor};
use std::comm::{stream};

#[test]
#[should_fail]
fn test_actor() {
    let (port, chan) = stream::<~str>();

    let actor_ref = Actor::new(port, chan);
    
    actor_ref.start();
    actor_ref.stop();
}
