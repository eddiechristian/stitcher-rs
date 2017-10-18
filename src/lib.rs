// Copyright © 2016 IronNet Cybersecurity, Inc. All rights reserved.
//
// Unauthorized copying of this file, via any medium, is strictly prohibited.
//
// PROPRIETARY AND CONFIDENTIAL
//

extern crate byteorder;
extern crate common;
extern crate cpu_cycle_counter;
extern crate libc;
#[macro_use]
extern crate log;

extern crate time;


use std::error::Error;
use std::io::{self, Write};
use std::fs::{self, File};

use config::Config;

#[macro_use]
pub mod macros;
pub mod config;

static mut SHUTDOWN: bool = false;

pub enum StitcherError {
    InvalidConfig(String),
    Application(String),
}

#[allow(unused_variables)]
pub fn run(config: &Config) -> Result<(), StitcherError> {
    name_thread!("stit:main_loop");
    // let mut pfring = try!(PFRingZC::new(config));
    // pfring.run()
}

pub fn shutdown() {
    unsafe {
        SHUTDOWN = true;
    }
}

fn is_shutdown() -> bool {
    unsafe { SHUTDOWN }
}
