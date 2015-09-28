use proto;

pub trait Scheduler {

    fn registered(
        &self,
        driver: &SchedulerDriver,
        framework_id: &proto::FrameworkID,
        master_info: &proto::MasterInfo) -> ();

    fn reregistered(
        &self,
        driver: &SchedulerDriver,
        master_info: &proto::MasterInfo) -> ();

    fn resource_offers(
        &self,
        driver: &SchedulerDriver,
        offers: Vec<proto::Offer>) -> ();

}

pub trait SchedulerDriver {
    fn run(&mut self) -> i32;
}
