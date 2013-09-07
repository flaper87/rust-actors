// Copyright 2013 The rust-actors developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::task;
use std::comm::{Port, SharedChan};

pub struct Actor {

    priv port: Port<~str>,
    priv chan: SharedChan<~str>

}

/**
 * Actors Implementation
 *
 * Implements the main functions for Actor.
 *
 * An actor consists in a Port and a SharedChan, both
 * accepting / receiving ~str. This pipes will be used
 * for 
 */
impl Actor {
    
    pub fn new(port: Port<~str>, chan: Chan<~str>) -> ActorRef {
        let shared = SharedChan::new(chan);
        let actor = Actor{port: port, chan: shared};
        ActorRef::new(actor)
    }

    fn start(&self) { println("Starting Actor") }

    fn listen(self) {
        do task::spawn {
            loop {
                let msg = self.port.recv();

                match msg {
                    ~"start" => self.start(),
                    ~"execute" => println("processing message"),
                    ~"stop" => self.stop(),
                    _     => println("something else")
                }
            }
        }
    }

    fn stop(&self) {
        println("Stopping Actor");
        fail!("Task was stopped by callee.");
    }
}


pub struct ActorRef { 
    priv chan: SharedChan<~str>,
}

impl ActorRef {
    
    fn new(actor: Actor) -> ActorRef {
        let chan = actor.chan.clone();
        actor.listen();
        ActorRef{chan: chan}
    }


    pub fn start(&self) {
        self.chan.send(~"start");
    }

    pub fn stop(&self) {
        self.chan.send(~"stop");
    }
}
