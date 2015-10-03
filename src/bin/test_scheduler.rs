extern crate mesos;

use mesos::scheduler::{Scheduler, SchedulerDriver};
use mesos::native::MesosSchedulerDriver;
use mesos::proto;

use std::process;

struct MyScheduler;

impl Scheduler for MyScheduler {
    fn registered(
        &self,
        _: &SchedulerDriver,
        framework_id: &proto::FrameworkID,
        master_info: &proto::MasterInfo) {

        println!("MyScheduler::registered");
        println!("framework_id: {:?}", framework_id);
        println!("master_info: {:?}", master_info);
    }

    fn reregistered(
        &self,
        _: &SchedulerDriver,
        master_info: &proto::MasterInfo) {

        println!("MyScheduler::reregistered");
        println!("master_info: {:?}", master_info);
    }

    fn resource_offers(
        &self,
        driver: &SchedulerDriver,
        offers: Vec<proto::Offer>) {

        println!("MyScheduler::resource_offers");
        println!("Received [{}] offers", offers.len());

        driver.print_debug_info();

        for offer in offers {
            println!("Declining  offer: [{:?}]", offer);
            driver.decline_offer(
                offer.get_id(),
                &proto::Filters::new());
        }
    }

    fn status_update(
        &self,
        _: &SchedulerDriver,
        task_status: &proto::TaskStatus) {

        println!("MyScheduler::status_update");
        println!("task_status: {:?}", task_status);
    }

    fn disconnected(
        &self,
        _: &SchedulerDriver) {

        println!("MyScheduler::disconnected");
        println!("Goodbye!");
        process::exit(1);
    }

    fn offer_rescinded(
        &self,
        _: &SchedulerDriver,
        offer_id: &proto::OfferID) {

        println!("MyScheduler::offer_rescinded");
        println!("offer_id: {:?}", offer_id);
    }

    fn framework_message(
        &self,
        _: &SchedulerDriver,
        executor_id: &proto::ExecutorID,
        slave_id: &proto::SlaveID,
        data: &String) {

        println!("MyScheduler::framework_message");
        println!("executor_id: {:?}", executor_id);
        println!("slave_id: {:?}", slave_id);
        println!("data: {:?}", data);
    }

    fn slave_lost(
        &self,
        _: &SchedulerDriver,
        slave_id: &proto::SlaveID) {

        println!("MyScheduler::slave_lost");
        println!("slave_id: {:?}", slave_id);
    }

    fn executor_lost(
        &self,
        _: &SchedulerDriver,
        executor_id: &proto::ExecutorID,
        slave_id: &proto::SlaveID,
        status: i32) {

        println!("MyScheduler::executor_lost");
        println!("executor_id: {:?}", executor_id);
        println!("slave_id: {:?}", slave_id);
        println!("status: {:?}", status);
    }

    fn error(
        &self,
        _: &SchedulerDriver,
        message: &String) {

        println!("MyScheduler::error");
        println!("message: {:?}", message);
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

    driver.run();
}
