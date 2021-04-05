#![feature(proc_macro_hygiene, decl_macro)]

use std::path::PathBuf;
use structopt::StructOpt;
mod db;
mod http_server;
mod proto;
use anyhow::{Context, Result};
// use exitfailure::ExitFailure;
// use proto::*;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;

#[derive(StructOpt, Debug, PartialEq)]
#[structopt(name = "basic")]
pub struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Ask server for status
    #[structopt(short, long)]
    status: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Port for http server
    #[structopt(short, long, default_value = "38000")]
    port: u32,

    /*
    /// Set speed
    #[structopt(short, long, default_value = "42")]
    speed: f64,
    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
     */
    // the long option will be translated by default to kebab case,
    // i.e. `--nb-cars`.
    /// Number of cars
    #[structopt(short = "c", long)]
    nb_cars: Option<i32>,

    /// admin_level to consider
    #[structopt(short, long)]
    level: Vec<String>,

    /// Generate a test message
    #[structopt(short, long)]
    test_message: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    println!("Hello, world!");
    let opt = Opt::from_args();
    if opt.status {
        let url = format!("http://localhost:{}/event_router/status", opt.port);
        let resp = reqwest::blocking::get(url)?.json::<proto::EventManagerStatusResponse>()?;
        println!("{:#?}", resp);
    } else if opt.test_message {
        proto::send_test_message().context("Failed to send test message")?;
    } else {
        proto::udp::start_listener()?;
        http_server::build_app(opt).launch();
    }
    Ok(())
}
