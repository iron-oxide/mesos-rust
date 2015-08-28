mod mesos_c;
mod tests;

use libc::c_void;
use proto;
use protobuf::core::Message;
use scheduler::{Scheduler, SchedulerDriver};
use std::ffi::CString;
use std::option::Option;
use std::ptr;

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
        MesosSchedulerDriver {
            scheduler: scheduler,
            framework_info: framework_info,
            master: master,
            native_ptr_pair: None,
        }
    }

    fn create_callbacks(
        &self,
        scheduler: &Scheduler
    ) -> mesos_c::SchedulerCallBacks {
        println!("MesosSchedulerDriver::create_callbacks");

        extern "C" fn wrapped_registered_callback(
            driver: mesos_c::SchedulerDriverPtr,
            framework_id: *mut mesos_c::ProtobufObj,
            master_info: *mut mesos_c::ProtobufObj,
        ) -> () {
            println!("wrapped_registered_callback");
            // marshal frameworkID
            // marshal masterInfo
            println!("delegating to scheduler.registered...");
            // invoke scheduler.registered(self, fwId, mInfo);
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
            &mut self.create_callbacks(self.scheduler);

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
