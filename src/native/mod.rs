mod mesos_c;
mod tests;

use libc::{c_void, size_t};
use proto;
use protobuf::core::Message;
use scheduler::{Scheduler, SchedulerDriver};
use std::ffi::CString;
use std::mem;
use std::ops::Deref;
use std::option::Option;
use std::ptr;
use std::slice;
use std::sync::RwLock;

// HACK. A 128 bit type to hold Rust trait references.
#[derive(Copy, Clone)]
#[repr(C)]
struct TraitPtr(u64, u64);

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
    // HACK.
    //
    // TODO(CD): Clean this up (using a lookup and synchronization
    // barriers) to make it possible for a single process to activate
    // multiple scheduler drivers simultaneously.
    static ref SCHEDULER_PTR: RwLock<Option<SchedulerPtr>> =
        RwLock::new(None);

    static ref SCHEDULER_DRIVER_PTR: RwLock<Option<SchedulerDriverPtr>> =
        RwLock::new(None);
}

fn delegate<L, T>(lambda: L) -> T
    where L : Fn(&Scheduler, &SchedulerDriver) -> T {

    let scheduler: &Scheduler = unsafe {
        let SchedulerPtr(raw) =
            SCHEDULER_PTR.read()
                .unwrap()  // Unwrap lock acquisition result.
                .unwrap(); // Unwrap option.
        mem::transmute(raw)
    };

    let scheduler_driver: &MesosSchedulerDriver = unsafe {
        let SchedulerDriverPtr(raw) =
            SCHEDULER_DRIVER_PTR.read()
                .unwrap()  // Unwrap lock acquisition result.
                .unwrap(); // Unwrap option.
        mem::transmute(raw)
    };

    lambda(scheduler, scheduler_driver)
}

pub struct MesosSchedulerDriver<'a> {
    scheduler: &'a Scheduler,
    framework_info: proto::FrameworkInfo,
    master: String,
    native_ptr_pair: Option<mesos_c::SchedulerPtrPair>,
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

        // Save pointers to be used when constructing C funtions that
        // delegate to the Rust scheduler implementation.
        let mut sched_ptr = SCHEDULER_PTR.write().unwrap();
        *sched_ptr = Some(unsafe { mem::transmute(scheduler) });

        let mut driver_ptr = SCHEDULER_DRIVER_PTR.write().unwrap();
        *driver_ptr = Some(unsafe { mem::transmute(&driver) });

        driver
    }

    fn create_callbacks(&self) -> mesos_c::SchedulerCallBacks {

        extern "C" fn wrapped_registered_callback(
            _: mesos_c::SchedulerDriverPtr,
            native_framework_id: *mut mesos_c::ProtobufObj,
            native_master_info: *mut mesos_c::ProtobufObj
        ) -> () {
            let framework_id = &mut proto::FrameworkID::new();
            mesos_c::ProtobufObj::merge(native_framework_id, framework_id);

            let master_info = &mut proto::MasterInfo::new();
            mesos_c::ProtobufObj::merge(native_master_info, master_info);

            delegate(|scheduler, driver| {
                scheduler.registered(driver, &framework_id, &master_info);
            });
        }

        extern "C" fn wrapped_reregistered_callback(
            _: mesos_c::SchedulerDriverPtr,
            native_master_info: *mut mesos_c::ProtobufObj
        ) -> () {
            let master_info = &mut proto::MasterInfo::new();
            mesos_c::ProtobufObj::merge(native_master_info, master_info);

            delegate(|scheduler, driver| {
                scheduler.reregistered(driver, &master_info);
            });
        }

        extern "C" fn wrapped_resource_offers_callback(
            _: mesos_c::SchedulerDriverPtr,
            native_offers: *mut mesos_c::ProtobufObj,
            native_num_offers: size_t
        ) -> () {
            let num_offers = native_num_offers as usize;

            let mut pbs = unsafe {
                slice::from_raw_parts(
                    native_offers as *const mesos_c::ProtobufObj,
                    num_offers as usize).to_vec()
            };

            let mut offers = vec![];

            for mut pb in pbs {
                let mut offer = proto::Offer::new();
                mesos_c::ProtobufObj::merge(&mut pb, &mut offer);
                offers.push(offer);
            }

            delegate(|scheduler, driver| {
                scheduler.resource_offers(driver, offers.clone());
            });
        }

        mesos_c::SchedulerCallBacks {
            registeredCallBack: Some(wrapped_registered_callback),
            reregisteredCallBack: Some(wrapped_reregistered_callback),
            resourceOffersCallBack: Some(wrapped_resource_offers_callback),
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
