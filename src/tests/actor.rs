extern mod actors;

use actors::actor::{Actor, ActorRef};
use std::comm::{stream};
use std::task;

#[test]
#[should_fail]
fn test_actor() {
    let (port, chan) = stream::<~str>();
    let actor_ref = ActorRef::new(chan);

    let actor = Actor::new(port, actor_ref.get_channel());
    
    do task::spawn {
        actor.start();
    };

    actor_ref.stop();
}
