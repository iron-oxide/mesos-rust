use proto;

pub trait Scheduler {

    fn registered(
        &mut self,
        driver: &SchedulerDriver,
        framework_id: &proto::FrameworkID,
        master_info: &proto::MasterInfo) -> ();

    fn reregistered(
        &mut self,
        driver: &SchedulerDriver,
        master_info: &proto::MasterInfo) -> ();

    fn resource_offers(
        &mut self,
        driver: &SchedulerDriver,
        offers: Vec<proto::Offer>) -> ();

    fn test(&mut self) { println!("in scheduler method!!!") }

}

pub trait SchedulerDriver {
    fn run(&mut self) -> i32;

    fn launch_tasks(&self,
        offerId: &proto::OfferID,
        tasks: Vec<proto::TaskInfo>,
        filters: &proto::Filters) -> i32;
}
