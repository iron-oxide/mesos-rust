mod mesos_c;

use proto;
use scheduler::{Scheduler, SchedulerDriver};
use std::ffi::CString;
use std::option::Option;

pub struct MesosSchedulerDriver<'a> {
    scheduler: &'a Scheduler,
    frameworkInfo: proto::FrameworkInfo,
    master: String,
    native_ptr_pair: Option<mesos_c::SchedulerPtrPair>,
}

impl<'a> MesosSchedulerDriver<'a> {

    fn new(
        scheduler: &Scheduler,
        frameworkInfo: proto::FrameworkInfo,
        master: String
    ) -> MesosSchedulerDriver {
        MesosSchedulerDriver {
            scheduler: scheduler,
            frameworkInfo: frameworkInfo,
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
            frameworkID: *mut mesos_c::ProtobufObj,
            masterInfo: *mut mesos_c::ProtobufObj,
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

        let native_framework_info: *mut mesos_c::ProtobufObj =
            &mut mesos_c::ProtobufObj::default();

        let native_master = CString::new(self.master.clone()).unwrap();

        let scheduler_ptr_pair = unsafe {
            mesos_c::scheduler_init(callbacks,
                                    native_framework_info,
                                    native_master.as_ptr() as *const i8)
        };

        self.native_ptr_pair = Some(scheduler_ptr_pair);

        return 0; // TODO(CD): this better!  e.g. actually start the driver

    }

}
