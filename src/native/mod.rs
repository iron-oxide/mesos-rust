mod mesos_c;
mod tests;

use libc::c_void;
use proto;
use protobuf::core::Message;
use scheduler::{Scheduler, SchedulerDriver};
use std::ffi::CString;
use std::mem;
use std::ops::Deref;
use std::option::Option;
use std::ptr;
use std::sync::RwLock;

#[derive(Copy, Clone)]
#[repr(C)]
struct TraitPtr {
    upper: u64,
    lower: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct SchedulerPtr(pub TraitPtr);
unsafe impl Send for SchedulerPtr {}
unsafe impl Sync for SchedulerPtr {}

#[derive(Copy, Clone)]
#[repr(C)]
struct SchedulerDriverPtr(pub *mut c_void);
unsafe impl Send for SchedulerDriverPtr {}
unsafe impl Sync for SchedulerDriverPtr {}

lazy_static! {
    // TODO(CD): Clean this up (using a lookup and synchronization barriers) to
    // make it possible for a single process to activate multiple scheduler
    // drivers simultaneously. HACK.
    static ref SCHEDULER_PTR: RwLock<Option<SchedulerPtr>> =
        RwLock::new(None);

    static ref SCHEDULER_DRIVER_PTR: RwLock<Option<SchedulerDriverPtr>> =
        RwLock::new(None);
}

pub struct MesosSchedulerDriver<'a> {
    scheduler: &'a Scheduler,
    framework_info: proto::FrameworkInfo,
    master: String,
    native_ptr_pair: Option<mesos_c::SchedulerPtrPair>,
}

struct WrappedScheduler<'a> {
    scheduler: &'a Scheduler,
}

impl<'a> MesosSchedulerDriver<'a> {

    pub fn new(
        scheduler: &Scheduler,
        framework_info: proto::FrameworkInfo,
        master: String
    ) -> MesosSchedulerDriver {

        let driver = MesosSchedulerDriver {
            scheduler: scheduler,
            framework_info: framework_info,
            master: master,
            native_ptr_pair: None,
        };

        // Save pointers, to be used when constructing C funtions that
        // delegate to the Scheduler implementation.
        let mut sched_ptr = SCHEDULER_PTR.write().unwrap();
        *sched_ptr = Some(
            unsafe { mem::transmute(scheduler) }
        );

        let mut driver_ptr = SCHEDULER_DRIVER_PTR.write().unwrap();
        *driver_ptr = Some(
            unsafe { mem::transmute(&driver) }
        );

        driver
    }

    fn create_callbacks(&self) -> mesos_c::SchedulerCallBacks {
        println!("MesosSchedulerDriver::create_callbacks");

        extern "C" fn wrapped_registered_callback(
            driver: mesos_c::SchedulerDriverPtr,
            unsafe_framework_id: *mut mesos_c::ProtobufObj,
            unsafe_master_info: *mut mesos_c::ProtobufObj,
        ) -> () {
            println!("wrapped_registered_callback");

            // Convert unsafe pointers to references (unsafely)
            let native_framework_id = unsafe {
                let x: &mesos_c::ProtobufObj = &*unsafe_framework_id;
                x
            };

            let native_master_info = unsafe {
                let x: &mesos_c::ProtobufObj = &*unsafe_master_info;
                x
            };

            // Unmarshal protobuf objects
            let mut framework_id = proto::FrameworkID::new();
            framework_id.merge_from_bytes(
                native_framework_id.to_bytes())
                .unwrap();

            let mut master_info = proto::MasterInfo::new();
            master_info.merge_from_bytes(
                native_master_info.to_bytes()).unwrap();

            println!("delegating to scheduler.registered...");

            // Two unwrap() calls -- one for the lock acquisition,
            // another for the option.
            let scheduler: &Scheduler = unsafe {
                let SchedulerPtr(raw) =
                    SCHEDULER_PTR.read().unwrap().unwrap();
                mem::transmute(raw)
            };

            let scheduler_driver: &MesosSchedulerDriver = unsafe {
                let SchedulerDriverPtr(raw) =
                    SCHEDULER_DRIVER_PTR.read().unwrap().unwrap();
                mem::transmute(raw)
            };

            scheduler.registered(
                scheduler_driver,
                &framework_id,
                &master_info);
        }

        mesos_c::SchedulerCallBacks {
            registeredCallBack: Some(wrapped_registered_callback),
            reregisteredCallBack: None,
            resourceOffersCallBack: None,
            statusUpdateCallBack: None,
            disconnectedCallBack: None,
            offerRescindedCallBack: None,
            frameworkMessageCallBack: None,
            slaveLostCallBack: None,
            executorLostCallBack: None,
            errorCallBack: None,
        }
    }

}

impl<'a> SchedulerDriver for MesosSchedulerDriver<'a> {

    fn run(&mut self) -> i32 {
        assert!(self.native_ptr_pair.is_none());

        let callbacks: *mut mesos_c::SchedulerCallBacks =
            &mut self.create_callbacks();

        let native_payload: *mut c_void = ptr::null_mut();

        // lifetime of pb_data must exceed native_framework_info
        let pb_data = &mut vec![];

        let native_framework_info =
            &mut mesos_c::ProtobufObj::from_message(
                &self.framework_info,
                pb_data);

        let native_master = CString::new(self.master.clone()).unwrap();

        let scheduler_ptr_pair = unsafe {
            mesos_c::scheduler_init(callbacks,
                native_payload,
                native_framework_info as *mut mesos_c::ProtobufObj,
                native_master.as_ptr() as *const i8)
        };

        self.native_ptr_pair = Some(scheduler_ptr_pair);

        println!("Starting scheduler driver");
        let scheduler_status = unsafe{
            mesos_c::scheduler_start(scheduler_ptr_pair.driver)
        };
        println!("scheduler_status: [{}]", scheduler_status);

        println!("Joining scheduler driver");
        let scheduler_status = unsafe{
            mesos_c::scheduler_join(scheduler_ptr_pair.driver)
        };
        println!("scheduler_status: [{}]", scheduler_status);

        scheduler_status
    }

}
