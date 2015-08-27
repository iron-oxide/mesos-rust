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
        framework_id: &proto::FrameworkID,
        master_info: &proto::MasterInfo
    ) {
        println!("MyScheduler::registered");
        println!("framework_id: {:?}", framework_id);
        println!("master_info: {:?}", master_info);
    }
}

fn main() -> () {
    let scheduler = MyScheduler;

    let mut framework_info = proto::FrameworkInfo::new();
    framework_info.set_name("mesos-rust-test".to_string());
    framework_info.set_user("root".to_string());

    println!("framework_info: [{:?}]", framework_info);

    let mut driver = MesosSchedulerDriver::new(
        &scheduler,
        framework_info,
        "localhost:5050".to_string(),
    );

    println!("Starting scheduler driver");

    driver.run();
}

