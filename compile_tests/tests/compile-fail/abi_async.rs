extern crate savefile;
extern crate savefile_abi;
extern crate savefile_derive;
use std::collections::HashMap;
use savefile::prelude::*;
use savefile::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Debug;
use std::io::{BufWriter, Cursor, Write};
use savefile_abi::AbiConnection;
use savefile_derive::savefile_abi_exportable;

#[savefile_abi_exportable(version = 0)]
pub trait ExampleTrait {
    async fn set(&mut self, x: u32) -> u32;
//~^ 14:5: 14:10: savefile-abi does not support async methods. You can try returning a boxed future instead: Pin<Box<Future<Output=u32>>>
}

fn main() {}