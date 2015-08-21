/*
 * This file will not stick around, only here for smoke testing the linker.
 */
extern crate mesos;

use mesos::scheduler::{Scheduler, SchedulerDriver};
use mesos::native::MesosSchedulerDriver;
use mesos::proto;

struct MyScheduler;

impl Scheduler for MyScheduler {
    fn registered(
        &self,
        driver: &SchedulerDriver,
        frameworkID: &proto::FrameworkID,
        masterInfo: &proto::MasterInfo
    ) {
        println!("MyScheduler::registered");
    }
}

fn main() -> () {
    let scheduler = MyScheduler;
    let mut driver = MesosSchedulerDriver::new(
        &scheduler,
        proto::FrameworkInfo::new(),
        "localhost:5050".to_string(),
    );
    println!("Starting scheduler driver");
    driver.run();
}

