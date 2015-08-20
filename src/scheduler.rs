use proto;

pub trait Scheduler {

    fn registered(
        &self,
        driver: &SchedulerDriver,
        frameworkID: &proto::FrameworkID,
        masterInfo: &proto::MasterInfo) -> ();

}

pub trait SchedulerDriver {
    fn run(&mut self) -> i32;
}
