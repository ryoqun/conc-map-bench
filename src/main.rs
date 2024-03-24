#[global_allocator]
//static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
static GLOBAL: rpmalloc::RpMalloc = rpmalloc::RpMalloc;

use std::fmt::Debug;

use structopt::StructOpt;

mod adapters;
mod bench;
mod plot;
mod record;
mod workloads;

#[derive(Debug, StructOpt)]
enum Options {
    Bench(bench::Options),
    Plot(plot::Options),
}

fn main() {
    tracing_subscriber::fmt::init();

    match Options::from_args() {
        Options::Bench(options) => bench::bench(&options),
        Options::Plot(options) => plot::plot(&options),
    }
}
