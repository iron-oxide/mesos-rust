extern crate mesos;

use mesos::scheduler::{Scheduler, SchedulerDriver};
use mesos::native::MesosSchedulerDriver;
use mesos::proto;

struct MyScheduler;

impl Scheduler for MyScheduler {
    fn registered(
        &mut self,
        driver: &SchedulerDriver,
        framework_id: &proto::FrameworkID,
        master_info: &proto::MasterInfo) {

        println!("MyScheduler::registered");
        println!("framework_id: {:?}", framework_id);
        println!("master_info: {:?}", master_info);
    }

    fn reregistered(
        &mut self,
        driver: &SchedulerDriver,
        master_info: &proto::MasterInfo) {

        println!("MyScheduler::reregistered");
        println!("master_info: {:?}", master_info);
    }

    fn resource_offers(
        &mut self,
        driver: &SchedulerDriver,
        offers: Vec<proto::Offer>) {

        println!("MyScheduler::resource_offers");
        println!("Received [{}] offers", offers.len());
        for offer in offers {
            println!("  Offer id: [{:?}]", offer.get_id());
            println!("offer: {:?}", offer);
            let mut task_id = proto::TaskID::default_instance().clone();
            task_id.set_value("rust-task-1".to_string());
            let mut task = proto::TaskInfo::default_instance().clone();
            task.set_name("rust-task-1".to_string());
            task.set_task_id(task_id.clone());
            task.set_slave_id(offer.get_slave_id().clone());
            let filters = proto::Filters::new();
            driver.launch_tasks(offer.get_id(), vec![task.clone()], &filters);
        }
    }

    fn test(&mut self) { println!("in scheduler method!!!") }
}

fn main() -> () {
    let mut scheduler = MyScheduler;

    println!("scheduler address: {:?}", &scheduler as *const Scheduler as *const u64);

    let mut framework_info = proto::FrameworkInfo::new();
    framework_info.set_name("mesos-rust-test".to_string());
    framework_info.set_user("root".to_string());

    println!("framework_info: [{:?}]", framework_info);

    let mut driver = MesosSchedulerDriver::new(
        &mut scheduler,
        framework_info,
        "localhost:5050".to_string(),
    );

    driver.run();
}

