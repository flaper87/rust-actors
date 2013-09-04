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

use std::comm::{Port, SharedChan};

pub struct Actor {

    priv port: Port<~str>,

    priv chan: SharedChan<~str>

}

/**
 * Actors Implementation
 *
 * This trait defines some default methods
 * like start, stop, receive.
 */
impl Actor {
    
    pub fn new(port: Port<~str>, chan: SharedChan<~str>) -> Actor {
        Actor{port: port, chan: chan}
    }

    pub fn start(&self) {
        //TODO: Put in a task
        self.receive();
    }

    pub fn receive(&self) {
        loop {
            let msg = self.port.recv();

            match msg {
                ~"execute" => println("processing message"),
                ~"stop" => fail!("Task was stopped by callee."),
                _     => println("something else")
            }
        }
    }

    pub fn stop(&self) {}
}
