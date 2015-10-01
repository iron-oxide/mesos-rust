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

    fn status_update(
        &self,
        driver: &SchedulerDriver,
        task_status: &proto::TaskStatus) -> ();

    fn disconnected(
        &self,
        driver: &SchedulerDriver) -> ();

    fn offer_rescinded(
        &self,
        driver: &SchedulerDriver,
        offer_id: &proto::OfferID) -> ();

    fn slave_lost(
        &self,
        driver: &SchedulerDriver,
        slave_id: &proto::SlaveID) -> ();

    fn executor_lost(
        &self,
        driver: &SchedulerDriver,
        executor_id: &proto::ExecutorID,
        slave_id: &proto::SlaveID,
        status: i32) -> ();

    fn framework_message(
        &self,
        driver: &SchedulerDriver,
        data: &String) -> ();
}

pub trait SchedulerDriver {
    fn run(&mut self) -> i32;
}
