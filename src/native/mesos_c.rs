use libc::{c_void, size_t};
use std::option::Option;

#[repr(C)]
#[derive(Copy)]
pub struct ProtobufObj {
    pub data: *mut c_void,
    pub size: size_t,
}

impl Clone for ProtobufObj {
    fn clone(&self) -> Self { *self }
}

impl Default for ProtobufObj {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type SchedulerDriverPtr = *mut c_void;

pub type scheduler_registeredCallBack_t =
    Option<extern "C" fn(arg1: SchedulerDriverPtr,
                                        arg2: *mut ProtobufObj,
                                        arg3: *mut ProtobufObj) -> ()>;

pub type scheduler_reregisteredCallBack_t =
    Option<extern "C" fn(arg1: SchedulerDriverPtr,
                                        arg2: *mut ProtobufObj) -> ()>;

pub type scheduler_resourceOffersCallBack_t =
    Option<extern "C" fn(arg1: SchedulerDriverPtr,
                                        arg2: *mut ProtobufObj, arg3: size_t)
                              -> ()>;

pub type scheduler_statusUpdateCallBack_t =
    Option<extern "C" fn(arg1: SchedulerDriverPtr,
                                        arg2: *mut ProtobufObj) -> ()>;

pub type scheduler_disconnectedCallBack_t =
    Option<extern "C" fn(arg1: SchedulerDriverPtr) -> ()>;

pub type scheduler_offerRescindedCallBack_t =
    Option<extern "C" fn(arg1: SchedulerDriverPtr,
                                        arg2: *mut ProtobufObj) -> ()>;

pub type scheduler_frameworkMessageCallBack_t =
    Option<extern "C" fn(arg1: SchedulerDriverPtr,
                                        arg2: *mut ProtobufObj,
                                        arg3: *mut ProtobufObj,
                                        arg4: *const ::libc::c_char) -> ()>;

pub type scheduler_slaveLostCallBack_t =
    Option<extern "C" fn(arg1: SchedulerDriverPtr,
                                        arg2: *mut ProtobufObj) -> ()>;

pub type scheduler_executorLostCallBack_t =
    Option<extern "C" fn(arg1: SchedulerDriverPtr,
                                        arg2: *mut ProtobufObj,
                                        arg3: *mut ProtobufObj,
                                        arg4: ::libc::c_int) -> ()>;

pub type scheduler_errorCallBack_t =
    Option<extern "C" fn(arg1: SchedulerDriverPtr,
                                        arg2: *const ::libc::c_char) -> ()>;

#[repr(C)]
#[derive(Copy)]
pub struct SchedulerCallBacks {
    pub registeredCallBack: scheduler_registeredCallBack_t,
    pub reregisteredCallBack: scheduler_reregisteredCallBack_t,
    pub resourceOffersCallBack: scheduler_resourceOffersCallBack_t,
    pub statusUpdateCallBack: scheduler_statusUpdateCallBack_t,
    pub disconnectedCallBack: scheduler_disconnectedCallBack_t,
    pub offerRescindedCallBack: scheduler_offerRescindedCallBack_t,
    pub frameworkMessageCallBack: scheduler_frameworkMessageCallBack_t,
    pub slaveLostCallBack: scheduler_slaveLostCallBack_t,
    pub executorLostCallBack: scheduler_executorLostCallBack_t,
    pub errorCallBack: scheduler_errorCallBack_t,
}

impl Clone for SchedulerCallBacks {
    fn clone(&self) -> Self { *self }
}

impl Default for SchedulerCallBacks {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type SchedulerCallBacksPtr = *mut c_void;

#[repr(C)]
#[derive(Copy)]
pub struct SchedulerPtrPair {
    pub scheduler: SchedulerCallBacksPtr,
    pub driver: SchedulerDriverPtr,
}

impl Clone for SchedulerPtrPair {
    fn clone(&self) -> Self { *self }
}

impl Default for SchedulerPtrPair {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type ExecutorDriverPtr = *mut c_void;

pub type executor_registeredCallBack_t =
    Option<extern "C" fn(arg1: ExecutorDriverPtr,
                                        arg2: *mut ProtobufObj,
                                        arg3: *mut ProtobufObj,
                                        arg4: *mut ProtobufObj) -> ()>;

pub type executor_reregisteredCallBack_t =
    Option<extern "C" fn(arg1: ExecutorDriverPtr,
                                        arg2: *mut ProtobufObj) -> ()>;

pub type executor_disconnectedCallBack_t =
    Option<extern "C" fn(arg1: ExecutorDriverPtr) -> ()>;

pub type executor_launchTaskCallBack_t =
    Option<extern "C" fn(arg1: ExecutorDriverPtr,
                                        arg2: *mut ProtobufObj) -> ()>;

pub type executor_killTaskCallBack_t =
    Option<extern "C" fn(arg1: ExecutorDriverPtr,
                                        arg2: *mut ProtobufObj) -> ()>;

pub type executor_frameworkMessageCallBack_t =
    Option<extern "C" fn(arg1: ExecutorDriverPtr,
                                        arg2: *const ::libc::c_char) -> ()>;

pub type executor_shutdownCallBack_t =
    Option<extern "C" fn(arg1: ExecutorDriverPtr) -> ()>;

pub type executor_errorCallBack_t =
    Option<extern "C" fn(arg1: ExecutorDriverPtr,
                                        arg2: *const ::libc::c_char) -> ()>;

#[repr(C)]
#[derive(Copy)]
pub struct ExecutorCallBacks {
    pub registeredCallBack: executor_registeredCallBack_t,
    pub reregisteredCallBack: executor_reregisteredCallBack_t,
    pub disconnectedCallBack: executor_disconnectedCallBack_t,
    pub launchTaskCallBack: executor_launchTaskCallBack_t,
    pub killTaskCallBack: executor_killTaskCallBack_t,
    pub frameworkMessageCallBack: executor_frameworkMessageCallBack_t,
    pub shutdownCallBack: executor_shutdownCallBack_t,
    pub errorCallBack: executor_errorCallBack_t,
}

impl Clone for ExecutorCallBacks {
    fn clone(&self) -> Self { *self }
}

impl Default for ExecutorCallBacks {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type ExecutorCallBacksPtr = *mut c_void;

#[repr(C)]
#[derive(Copy)]
pub struct ExecutorPtrPair {
    pub executor: ExecutorCallBacksPtr,
    pub driver: ExecutorDriverPtr,
}

impl Clone for ExecutorPtrPair {
    fn clone(&self) -> Self { *self }
}

impl Default for ExecutorPtrPair {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type SchedulerDriverStatus = ::libc::c_int;

pub type ExecutorDriverStatus = ::libc::c_int;

#[link(name = "mesos")]
extern "C" {
    pub fn scheduler_launchTasks(
        driver: SchedulerDriverPtr,
        offerId: *mut ProtobufObj,
        tasks: *mut ProtobufObj,
        filters: *mut ProtobufObj) -> SchedulerDriverStatus;

    pub fn scheduler_start(
        driver: SchedulerDriverPtr) -> SchedulerDriverStatus;

    pub fn scheduler_stop(
        driver: SchedulerDriverPtr,
        failover: ::libc::c_int) -> SchedulerDriverStatus;

    pub fn scheduler_abort(
        driver: SchedulerDriverPtr) -> SchedulerDriverStatus;

    pub fn scheduler_join(
        driver: SchedulerDriverPtr) -> SchedulerDriverStatus;

    pub fn scheduler_run(
        driver: SchedulerDriverPtr) -> SchedulerDriverStatus;

    pub fn scheduler_requestResources(
        driver: SchedulerDriverPtr,
        requestsData: *mut ProtobufObj) -> SchedulerDriverStatus;

    pub fn scheduler_declineOffer(
        driver: SchedulerDriverPtr,
        offerId: *mut ProtobufObj,
        filters: *mut ProtobufObj) -> SchedulerDriverStatus;

    pub fn scheduler_killTask(
        driver: SchedulerDriverPtr,
        taskId: *mut ProtobufObj) -> SchedulerDriverStatus;

    pub fn scheduler_reviveOffers(
        driver: SchedulerDriverPtr) -> SchedulerDriverStatus;

    pub fn scheduler_sendFrameworkMessage(
        driver: SchedulerDriverPtr,
        executor: *mut ProtobufObj,
        slaveId: *mut ProtobufObj,
        data: *const ::libc::c_char) -> SchedulerDriverStatus;

    pub fn scheduler_init(
        callbacks: *mut SchedulerCallBacks,
        framework: *mut ProtobufObj,
        master: *const ::libc::c_char) -> SchedulerPtrPair;

    pub fn scheduler_destroy(
        driver: *mut c_void,
        scheduler: *mut c_void) -> ();

    pub fn executor_start(
        driver: ExecutorDriverPtr) -> ExecutorDriverStatus;

    pub fn executor_stop(
        driver: ExecutorDriverPtr) -> ExecutorDriverStatus;

    pub fn executor_abort(
        driver: ExecutorDriverPtr) -> ExecutorDriverStatus;

    pub fn executor_join(
        driver: ExecutorDriverPtr) -> ExecutorDriverStatus;

    pub fn executor_run(
        driver: ExecutorDriverPtr) -> ExecutorDriverStatus;

    pub fn executor_sendStatusUpdate(
        driver: ExecutorDriverPtr,
        status: *mut ProtobufObj) -> ExecutorDriverStatus;

    pub fn executor_sendFrameworkMessage(
        driver: ExecutorDriverPtr,
        data: *const ::libc::c_char) -> ExecutorDriverStatus;

    pub fn executor_init(
        callbacks: *mut ExecutorCallBacks,
        payload: *mut c_void) -> ExecutorPtrPair;

    pub fn executor_destroy(
        driver: *mut c_void,
        executor: *mut c_void) -> ();
}
