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

#[link(name = "actors",
       vers = "0.1",
       uuid = "9e62bfb1-3d85-4f36-8056-3271bbc7d174",
       author = "Flavio Percoco",
       url = "https://github.com/flaper87/rust-actors")];

#[crate_type = "lib"];


pub mod actor;
