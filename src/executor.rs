use proto;

/// Callback interface to be implemented by frameworks' executors. Note that
/// only one callback will be invoked at a time, so it is not recommended that
/// you block within a callback because it may cause a deadlock.
///
/// Each callback includes a reference to the executor driver that was used
/// to run this executor. The reference will not change for the duration
/// of an executor (i.e., from the point you do `ExecutorDriver::start` to
/// the point that `ExecutorDriver::join` returns. This is intended for
/// convenience so that an executor doesn't need to store a reference to the
/// driver itself.
pub trait Executor {

    /// Invoked once the executor driver has been able to successfully
    /// connect with Mesos. In particular, a scheduler can pass some data
    /// to its executors through the ExecutorInfo's `data` field.
    fn registered(
        &self,
        driver: &ExecutorDriver,
        executor_info: &proto::ExecutorInfo,
        framework_info: &proto::FrameworkInfo,
        slave_info: &proto::SlaveInfo);

    /// Invoked when the executor re-registers with a restarted slave.
    fn reregistered(
        &self,
        driver: &ExecutorDriver,
        slave_info: &proto::SlaveInfo);

    /// Invoked when the executor becomes "disconnected" from the slave
    /// (e.g., the slave was restarted due to an upgrade).
    fn disconnected(&self, driver: &ExecutorDriver);

    /// Invoked when a task has been launched on this executor (initiated
    /// via `SchedulerDriver::launch_tasks`. Note that this task can be
    /// realized with a thread, a process, or some simple computation,
    /// however, no other callbacks will be invoked on this executor until
    /// this callback has returned.
    fn launch_task(
        &self,
        driver: &ExecutorDriver,
        task: &proto::TaskInfo);

    /// Invoked when a task running within this executor has been killed
    /// (via `SchedulerDriver::kill_task`). Note that no status update will
    /// be sent on behalf of the executor, the executor is responsible for
    /// creating a new TaskStatus (i.e., with TASK_KILLED) and invoking
    /// `ExecutorDriver::send_status_update`.
    fn kill_task(
        &self,
        driver: &ExecutorDriver,
        task_id: &proto::TaskID);

    /// Invoked when a framework message has arrived for this executor.
    /// These messages are best effort; do not expect a framework message
    /// to be retransmitted in any reliable fashion.
    fn framework_message(
        &self,
        driver: &ExecutorDriver,
        data: &Vec<u8>);

    /// Invoked when the executor should terminate all of it's currently
    /// running tasks. Note that after Mesos has determined that an executor
    /// has terminated any tasks that the executor did not send terminal
    /// status updates for (e.g. TASK_KILLED, TASK_FINISHED, TASK_FAILED,
    /// TASK_LOST, TASK_ERROR, etc) a TASK_LOST status update will be
    /// created.
    fn shutdown(&self, driver: &ExecutorDriver);

    /// Invoked when a fatal error has occurred with the executor and/or
    /// executor driver. The driver will be aborted BEFORE invoking this
    /// callback.
    fn error(&self, driver: &ExecutorDriver, message: String);

}

/// Abstract interface for connecting an executor to Mesos. This interface
/// is used both to manage the executor's lifecycle (start it, stop it, or
/// wait for it to finish) and to interact with Mesos (e.g., send status
/// updates, send framework messages, etc.).
pub trait ExecutorDriver {

    /// Starts and immediately joins (i.e., blocks on) the driver.
    fn run(&mut self) -> i32;

    /// Stops the executor driver.
    fn stop(&self) -> i32;

    /// Sends a status update to the framework scheduler, retrying as
    /// necessary until an acknowledgement has been received or the executor
    /// is terminated (in which case, a TASK_LOST status update will be
    /// sent). See `Scheduler::status_update` for more information about
    /// status update acknowledgements.
    fn send_status_update(
        &self,
        task_status: &proto::TaskStatus) -> i32;

    /// Sends a message to the framework scheduler. These messages are best
    /// effort; do not expect a framework message to be retransmitted in
    /// any reliable fashion.
    fn send_framework_message(
        &self,
        data: &Vec<u8>) -> i32;

}
