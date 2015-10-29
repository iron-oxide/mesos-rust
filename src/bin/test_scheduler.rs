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

        // for offer in offers {
        //     println!("Declining  offer: [{:?}]", offer);
        //     driver.decline_offer(
        //         offer.get_id(),
        //         &proto::Filters::new());
        // }

        // Launch a task that consumes all the resources from each offer.
        for offer in offers {
            println!("Launching a task on offer: [{:?}]", offer);
            let mut task = proto::TaskInfo::new();

            task.set_name("mesos-rust-task".to_string());

            let mut task_id = proto::TaskID::new();
            task_id.set_value(offer.get_id().get_value().to_string());
            task.set_task_id(task_id);

            task.set_slave_id(offer.get_slave_id().clone());

            task.set_resources(offer.clone().take_resources());

            let mut command = proto::CommandInfo::new();
            command.set_shell(true);
            command.set_value("env && sleep 10".to_string());
            task.set_command(command);

            driver.launch_tasks(
                offer.get_id(),
                &vec![&task],
                &proto::Filters::new());
        }
    }

    fn status_update(
        &self,
        driver: &SchedulerDriver,
        task_status: &proto::TaskStatus) {

        println!("MyScheduler::status_update");
        println!("task_status: {:?}", task_status);

        // Kill all running tasks.
        if task_status.get_state() == proto::TaskState::TASK_RUNNING {
            let task_id = task_status.get_task_id();
            println!("Killing task [{:?}]", task_id);
            driver.kill_task(task_id);
        }
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

    // Uncomment the following lines to allow the scheduler to fail over
    // and recover running tasks.  Capture and persist the assigned framework
    // id in the scheduler `registered` callback.
    // framework_info.set_checkpoint(true);
    // framework_info.set_failover_timeout(604800000 as f64); // 1 week in ms.
    // if previous_framework_id.is_some() {
    //     framework_info.set_framework_id(previous_framework_id.unwrap());
    // }

    println!("framework_info: [{:?}]", framework_info);

    let mut driver = MesosSchedulerDriver::new(
        &scheduler,
        &framework_info,
        "localhost:5050".to_string(),
    );

    driver.run();
}
