use proto;

pub trait Scheduler {

    fn registered(
        &self,
        driver: &SchedulerDriver,
        framework_id: &proto::FrameworkID,
        master_info: &proto::MasterInfo) -> ();

}

pub trait SchedulerDriver {
    fn run(&mut self) -> i32;
}
