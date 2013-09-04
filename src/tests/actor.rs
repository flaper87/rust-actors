extern mod actors;

use actors::actor::{Actor};
use std::comm::{stream, Port, SharedChan};

#[test]
fn test_actor() {
    let (port, chan) = stream::<~str>();
    let chan = SharedChan::new(chan);
    let actor = Actor::new(port, chan);
}
