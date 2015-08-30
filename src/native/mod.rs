mod mesos_c;
mod tests;

use libc::{c_void, size_t};
use proto;
use protobuf::RepeatedField;
use protobuf::core::Message;
use scheduler::{Scheduler, SchedulerDriver};
use std::ffi::CString;
use std::mem;
use std::ops::Deref;
use std::option::Option;
use std::ptr;
use std::slice;
use std::sync::RwLock;

static CB_FUNCS: &'static mesos_c::SchedulerCallBacks =
    &mesos_c::SchedulerCallBacks {
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
    };

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
struct SchedulerDriverPtr(pub TraitPtr);
unsafe impl Send for SchedulerDriverPtr {}
unsafe impl Sync for SchedulerDriverPtr {}

struct Callbacks(SchedulerPtr, SchedulerDriverPtr);

fn delegate<L, T>(cb_ptr: *const c_void, lambda: L) -> T
    where L : Fn(&mut Scheduler, &mut SchedulerDriver) -> T {

    let callbacks: Box<Callbacks> = unsafe { mem::transmute(cb_ptr) };

    let mut scheduler: &mut Scheduler = unsafe {
        let SchedulerPtr(raw) = callbacks.0;
        mem::transmute(raw)
    };

    let mut scheduler_driver: &mut SchedulerDriver = unsafe {
        let SchedulerDriverPtr(raw) = callbacks.1;
        mem::transmute(raw)
    };

    lambda(scheduler, scheduler_driver)
}

extern "C" fn wrapped_registered_callback(
    cb_ptr: *const c_void,
    native_framework_id: *mut mesos_c::ProtobufObj,
    native_master_info: *mut mesos_c::ProtobufObj
) -> () {
    let framework_id = &mut proto::FrameworkID::new();
    mesos_c::ProtobufObj::merge(native_framework_id, framework_id);

    let master_info = &mut proto::MasterInfo::new();
    mesos_c::ProtobufObj::merge(native_master_info, master_info);

    delegate(cb_ptr, |scheduler, driver| {
        scheduler.registered(driver, &framework_id, &master_info);
    });
}

extern "C" fn wrapped_reregistered_callback(
    cb_ptr: *const c_void,
    native_master_info: *mut mesos_c::ProtobufObj
) -> () {
    let master_info = &mut proto::MasterInfo::new();
    mesos_c::ProtobufObj::merge(native_master_info, master_info);

    delegate(cb_ptr, |scheduler, driver| {
        scheduler.reregistered(driver, &master_info);
    });
}

extern "C" fn wrapped_resource_offers_callback(
    cb_ptr: *const c_void,
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

    delegate(cb_ptr, |scheduler, driver| {
        scheduler.resource_offers(driver, offers.clone());
    });
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

        let mut driver = MesosSchedulerDriver {
            scheduler: scheduler,
            framework_info: framework_info,
            master: master,
            native_ptr_pair: None,
        };

        // Save pointers to be used when constructing C functions that
        // delegate to the Rust scheduler implementation.
        let s_ptr: SchedulerPtr = unsafe { mem::transmute(scheduler) };
        let d_ptr: SchedulerDriverPtr = unsafe { mem::transmute(&driver as &SchedulerDriver) };
        let cb = Box::new(Callbacks(s_ptr, d_ptr));

        let native_payload: *mut c_void = unsafe {
            mem::transmute(cb)
        };

        // lifetime of pb_data must exceed native_framework_info
        let pb_data = &mut vec![];

        let native_framework_info =
            &mut mesos_c::ProtobufObj::from_message(
                &driver.framework_info,
                pb_data);

        let native_master = CString::new(driver.master.clone()).unwrap();

        let scheduler_ptr_pair = unsafe {
            mesos_c::scheduler_init(
                CB_FUNCS as *const mesos_c::SchedulerCallBacks,
                native_payload,
                native_framework_info as *mut mesos_c::ProtobufObj,
                native_master.as_ptr() as *const i8)
        };

        driver.native_ptr_pair = Some(scheduler_ptr_pair);

        driver
    }
}

impl<'a> SchedulerDriver for MesosSchedulerDriver<'a> {

    fn run(&mut self) -> i32 {
        assert!(self.native_ptr_pair.is_some());


        println!("Starting scheduler driver");
        let scheduler_status = unsafe{
            mesos_c::scheduler_start(self.native_ptr_pair.unwrap().driver)
        };

        println!("scheduler_status: [{}]", scheduler_status);

        println!("Joining scheduler driver");
        let scheduler_status = unsafe{
            mesos_c::scheduler_join(self.native_ptr_pair.unwrap().driver)
        };
        println!("scheduler_status: [{}]", scheduler_status);

        scheduler_status
    }

    fn launch_tasks(
        &self,
        offerId: &proto::OfferID,
        tasks: Vec<proto::TaskInfo>,
        filters: &proto::Filters
    ) -> i32 {

        // lifetime of offer_pb_data must exceed native_offer
        let offer_pb_data = &mut vec![];
        let native_offer =
            &mut mesos_c::ProtobufObj::from_message(
                offerId,
                offer_pb_data);

        let mut tasks_proto = RepeatedField::from_vec(tasks);
        let mut launch = proto::Offer_Operation_Launch::new();
        launch.set_task_infos(tasks_proto);

        // lifetime of tasks_pb_data must exceed native_tasks
        let tasks_pb_data = &mut vec![];
        let native_tasks =
            &mut mesos_c::ProtobufObj::from_message(
                &mut launch,
                tasks_pb_data);

        // lifetime of filters_pb_data must exceed native_filters
        let filters_pb_data = &mut vec![];
        let native_filters =
            &mut mesos_c::ProtobufObj::from_message(
                filters,
                filters_pb_data);

        println!("before native pair deref: {:?}", self.native_ptr_pair.is_some());
        let scheduler_status = unsafe {
            mesos_c::scheduler_launchTasks(
                self.native_ptr_pair.unwrap().driver,
                native_offer,
                native_tasks,
                native_filters
            )
        };
        println!("after native pair deref");

        println!("scheduler_status: [{}]", scheduler_status);

        scheduler_status
    }
}
