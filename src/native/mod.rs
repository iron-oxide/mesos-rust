#![allow(dead_code)]

mod mesos_c;
mod tests;

use libc::{c_char, c_void, size_t};
use proto;
use scheduler::{Scheduler, SchedulerDriver};
use std::boxed::Box;
use std::ffi::{CStr, CString};
use std::mem;
use std::option::Option;
use std::slice;
use std::str;

#[derive(Clone)]
#[repr(C)]
pub struct MesosSchedulerDriver<'a> {
    scheduler: &'a Scheduler,
    framework_info: &'a proto::FrameworkInfo,
    master: String,
    native_ptr_pair: Option<mesos_c::SchedulerPtrPair>,
}

impl<'a> MesosSchedulerDriver<'a> {

    pub fn new<'d>(
        scheduler: &'d Scheduler,
        framework_info: &'d proto::FrameworkInfo,
        master: String
    ) -> Box<MesosSchedulerDriver<'d>> {

        let mut driver = Box::new(
            MesosSchedulerDriver {
                scheduler: scheduler,
                framework_info: framework_info,
                master: master,
                native_ptr_pair: None,
            }
        );

        // Save pointers to be used when constructing C funtions that
        // delegate to the Rust scheduler implementation.
        driver.native_ptr_pair = Some(mesos_c::SchedulerPtrPair {
            driver: unsafe { mem::transmute(&driver) },
            scheduler: unsafe { mem::transmute(&scheduler) },
        });

        driver
    }

    fn create_callbacks(&self) -> mesos_c::SchedulerCallBacks {

        extern "C" fn wrapped_registered_callback(
            native_scheduler_driver: mesos_c::SchedulerDriverPtr,
            native_framework_id: *mut mesos_c::ProtobufObj,
            native_master_info: *mut mesos_c::ProtobufObj
        ) -> () {
            let driver: &MesosSchedulerDriver = unsafe {
                mem::transmute(native_scheduler_driver)
            };

            let framework_id = &mut proto::FrameworkID::new();
            mesos_c::ProtobufObj::merge(native_framework_id, framework_id);

            let master_info = &mut proto::MasterInfo::new();
            mesos_c::ProtobufObj::merge(native_master_info, master_info);

            driver.scheduler.registered(driver, &framework_id, master_info);
        }

        extern "C" fn wrapped_reregistered_callback(
            native_scheduler_driver: mesos_c::SchedulerDriverPtr,
            native_master_info: *mut mesos_c::ProtobufObj
        ) -> () {
            let driver: &MesosSchedulerDriver = unsafe {
                mem::transmute(native_scheduler_driver)
            };

            let master_info = &mut proto::MasterInfo::new();
            mesos_c::ProtobufObj::merge(native_master_info, master_info);

            driver.scheduler.reregistered(driver, master_info);
        }

        extern "C" fn wrapped_resource_offers_callback(
            native_scheduler_driver: mesos_c::SchedulerDriverPtr,
            native_offers: *mut mesos_c::ProtobufObj,
            native_num_offers: size_t
        ) -> () {
            let driver: &MesosSchedulerDriver = unsafe {
                mem::transmute(native_scheduler_driver)
            };

            let num_offers = native_num_offers as usize;

            let pbs = unsafe {
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

            driver.scheduler.resource_offers(driver, offers.clone());
        }

        extern "C" fn wrapped_status_update_callback(
            native_scheduler_driver: mesos_c::SchedulerDriverPtr,
            native_task_status: *mut mesos_c::ProtobufObj
        ) -> () {
            let driver: &MesosSchedulerDriver = unsafe {
                mem::transmute(native_scheduler_driver)
            };

            let task_status = &mut proto::TaskStatus::new();
            mesos_c::ProtobufObj::merge(native_task_status, task_status);

            driver.scheduler.status_update(driver, task_status);
        }

        extern "C" fn wrapped_disconnected_callback(
            native_scheduler_driver: mesos_c::SchedulerDriverPtr,
        ) -> () {
            let driver: &MesosSchedulerDriver = unsafe {
                mem::transmute(native_scheduler_driver)
            };

            driver.scheduler.disconnected(driver);
        }

        extern "C" fn wrapped_offer_rescinded_callback(
            native_scheduler_driver: mesos_c::SchedulerDriverPtr,
            native_offer_id: *mut mesos_c::ProtobufObj
        ) -> () {
            let driver: &MesosSchedulerDriver = unsafe {
                mem::transmute(native_scheduler_driver)
            };

            let offer_id = &mut proto::OfferID::new();
            mesos_c::ProtobufObj::merge(native_offer_id, offer_id);

            driver.scheduler.offer_rescinded(driver, offer_id);
        }

        extern "C" fn wrapped_framework_message_callback(
            native_scheduler_driver: mesos_c::SchedulerDriverPtr,
            native_executor_id: *mut mesos_c::ProtobufObj,
            native_slave_id: *mut mesos_c::ProtobufObj,
            native_data: *const c_char
        ) -> () {
            let driver: &MesosSchedulerDriver = unsafe {
                mem::transmute(native_scheduler_driver)
            };

            let executor_id = &mut proto::ExecutorID::new();
            mesos_c::ProtobufObj::merge(native_executor_id, executor_id);

            let slave_id = &mut proto::SlaveID::new();
            mesos_c::ProtobufObj::merge(native_slave_id, slave_id);

            let data_slice = unsafe {
                CStr::from_ptr(native_data).to_bytes()
            };

            let data: String = str::from_utf8(data_slice)
                .unwrap()
                .to_string();

            driver.scheduler.framework_message(driver,
                                               executor_id,
                                               slave_id,
                                               &data);
        }

        extern "C" fn wrapped_slave_lost_callback(
            native_scheduler_driver: mesos_c::SchedulerDriverPtr,
            native_slave_id: *mut mesos_c::ProtobufObj
        ) -> () {
            let driver: &MesosSchedulerDriver = unsafe {
                mem::transmute(native_scheduler_driver)
            };

            let slave_id = &mut proto::SlaveID::new();
            mesos_c::ProtobufObj::merge(native_slave_id, slave_id);

            driver.scheduler.slave_lost(driver, slave_id);
        }

        extern "C" fn wrapped_executor_lost_callback(
            native_scheduler_driver: mesos_c::SchedulerDriverPtr,
            native_executor_id: *mut mesos_c::ProtobufObj,
            native_slave_id: *mut mesos_c::ProtobufObj,
            native_status: ::libc::c_int
        ) -> () {
            let driver: &MesosSchedulerDriver = unsafe {
                mem::transmute(native_scheduler_driver)
            };

            let executor_id = &mut proto::ExecutorID::new();
            mesos_c::ProtobufObj::merge(native_executor_id, executor_id);

            let slave_id = &mut proto::SlaveID::new();
            mesos_c::ProtobufObj::merge(native_slave_id, slave_id);

            let status = native_status as i32;

            driver.scheduler.executor_lost(
                driver, executor_id, slave_id, status);
        }

        extern "C" fn wrapped_error_callback(
            native_scheduler_driver: mesos_c::SchedulerDriverPtr,
            native_message: *const c_char
        ) -> () {
            let driver: &MesosSchedulerDriver = unsafe {
                mem::transmute(native_scheduler_driver)
            };

            let message_slice = unsafe {
                CStr::from_ptr(native_message).to_bytes()
            };

            let message: String = str::from_utf8(message_slice)
                .unwrap()
                .to_string();

            driver.scheduler.error(driver, &message);
        }

        mesos_c::SchedulerCallBacks {
            registeredCallBack: Some(wrapped_registered_callback),
            reregisteredCallBack: Some(wrapped_reregistered_callback),
            resourceOffersCallBack: Some(wrapped_resource_offers_callback),
            statusUpdateCallBack: Some(wrapped_status_update_callback),
            disconnectedCallBack: Some(wrapped_disconnected_callback),
            offerRescindedCallBack: Some(wrapped_offer_rescinded_callback),
            frameworkMessageCallBack: Some(wrapped_framework_message_callback),
            slaveLostCallBack: Some(wrapped_slave_lost_callback),
            executorLostCallBack: Some(wrapped_executor_lost_callback),
            errorCallBack: Some(wrapped_error_callback),
        }
    }

}

impl<'a> SchedulerDriver for MesosSchedulerDriver<'a> {

    fn print_debug_info(&self) {
        println!("MesosSchedulerDriver:");
        println!("  framework_info: [{:?}]", self.framework_info);
        println!("  master: [{:?}]", self.master);
        println!("  native_ptr_pair.is_some(): [{:?}]",
            self.native_ptr_pair.is_some());
    }

    fn run(&mut self) -> i32 {

        let callbacks: *mut mesos_c::SchedulerCallBacks =
            &mut self.create_callbacks();

        let native_payload: *mut c_void = unsafe {
            mem::transmute(&self.clone())
        };

        // lifetime of pb_data must exceed native_framework_info
        let pb_data = &mut vec![];

        let native_framework_info =
            &mut mesos_c::ProtobufObj::from_message(
                self.framework_info,
                pb_data);

        let native_master = CString::new(self.master.clone()).unwrap();

        let native_ptr_pair = unsafe {
            mesos_c::scheduler_init(callbacks,
                native_payload,
                native_framework_info as *mut mesos_c::ProtobufObj,
                native_master.as_ptr() as *const i8)
        };

        println!("Starting scheduler driver");
        let scheduler_status = unsafe{
            mesos_c::scheduler_start(native_ptr_pair.driver)
        };
        println!("scheduler_status: [{}]", scheduler_status);

        println!("Joining scheduler driver");
        let scheduler_status = unsafe{
            mesos_c::scheduler_join(native_ptr_pair.driver)
        };
        println!("scheduler_status: [{}]", scheduler_status);

        scheduler_status
    }

    fn decline_offer(
        &self,
        offer_id: &proto::OfferID,
        filters: &proto::Filters) -> i32 {

        println!("MesosSchedulerDriver::decline_offer");
        self.print_debug_info();

        assert!(self.native_ptr_pair.is_some());
        let native_driver = self.native_ptr_pair.unwrap().driver;

        let offer_id_data = &mut vec![];
        let native_offer_id = &mut mesos_c::ProtobufObj::from_message(
            offer_id,
            offer_id_data);

        let filters_data = &mut vec![];
        let native_filters = &mut mesos_c::ProtobufObj::from_message(
            filters,
            filters_data);

        let scheduler_status = unsafe {
            mesos_c::scheduler_declineOffer(
                native_driver,
                native_offer_id as *mut mesos_c::ProtobufObj,
                native_filters as *mut mesos_c::ProtobufObj)
        };

        scheduler_status
    }
}
