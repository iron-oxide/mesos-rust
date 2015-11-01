#![allow(dead_code)]

//! # Native bindings for Apache Mesos.
//!
//! This module links dynamically against `libmesos` and provides a native
//! `Scheduler` implementation, which delegates to a user-supplied Rust
//! `Scheduler` for all callbacks.
//!
//! Additionally, this module provides a native `SchedulerDriver` that manages
//! the backing native state and hides the required function delegate
//! wiring.

mod mesos_c;
mod tests;

use libc::{c_char, c_int, c_void, size_t};
use proto;
use scheduler::{Scheduler, SchedulerDriver};
use std::boxed::Box;
use std::ffi::{CStr, CString};
use std::mem;
use std::option::Option;
use std::slice;
use std::str;

#[derive(Clone)]
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
        Box::new(
            MesosSchedulerDriver {
                scheduler: scheduler,
                framework_info: framework_info,
                master: master,
                native_ptr_pair: None,
            }
        )
    }

    // Returns a C struct containing nullable C function pointers, where
    // each such pointer refers to a wrapper function that unmarshals native
    // data structures and delegates to this driver's (Rust) scheduler
    // implementation.
    //
    // A pointer to the result struct is eventually passed to the native
    // function `scheduler_init`.
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

    fn run(&mut self) -> i32 {

        let callbacks: *mut mesos_c::SchedulerCallBacks =
            &mut self.create_callbacks();

        let native_payload: *mut c_void = unsafe {
            // Super-unsafe!  This violates Rust's reference aliasing and
            // memory safety guarantees.  The MesosSchedulerDriver data
            // structure is opaque to the underlying native code (notice how
            // it's not annotated with #[repr(C)]; but anyway we promise not
            // to modify this structure from foreign code).
            mem::transmute(&mut *self)
        };

        // The lifetime of `pb_data` must exceed that of
        // `native_framework_info`.
        let pb_data = &mut vec![];

        let native_framework_info =
            &mut mesos_c::ProtobufObj::from_message(
                self.framework_info,
                pb_data);

        let native_master = CString::new(self.master.clone()).unwrap();

        self.native_ptr_pair = Some(
            unsafe {
                mesos_c::scheduler_init(
                    callbacks,
                    native_payload,
                    native_framework_info as *mut mesos_c::ProtobufObj,
                    native_master.as_ptr() as *const i8)
            }
        );

        let native_ptr_pair = self.native_ptr_pair.unwrap();

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

    fn request_resources(
        &self,
        requests: &Vec<&proto::Request>) -> i32 {

        assert!(self.native_ptr_pair.is_some());
        let native_driver = self.native_ptr_pair.unwrap().driver;

        let native_request_data = &mut vec![];
        for request in requests {
            let request_data = &mut vec![];
            mesos_c::ProtobufObj::from_message(*request, request_data);

            // write length of vec as a u64 to native_task_data
            let length_pointer: *const u8 = unsafe {
                mem::transmute(&(request_data.len() as u64))
            };

            let length_data: Vec<u8> = unsafe {
                slice::from_raw_parts(
                    length_pointer,
                    8 as usize).to_vec() // 8 bytes in a u64
            };

            native_request_data.extend(length_data);

            // write vec content to native_task_data
            native_request_data.extend(request_data.iter().cloned());
        }

        let native_requests =
            &mut mesos_c::ProtobufObj::from_vec(native_request_data);

        let scheduler_status = unsafe {
            mesos_c::scheduler_requestResources(
                native_driver,
                native_requests as *mut mesos_c::ProtobufObj)
        };

        scheduler_status
    }


    fn launch_tasks(
        &self,
        offer_id: &proto::OfferID,
        tasks: &Vec<&proto::TaskInfo>,
        filters: &proto::Filters) -> i32 {

        assert!(self.native_ptr_pair.is_some());
        let native_driver = self.native_ptr_pair.unwrap().driver;

        let offer_id_data = &mut vec![];
        let native_offer_id = &mut mesos_c::ProtobufObj::from_message(
            offer_id,
            offer_id_data);

        let native_task_data = &mut vec![];
        for task in tasks {
            let task_data = &mut vec![];
            mesos_c::ProtobufObj::from_message(*task, task_data);

            // write length of vec as a u64 to native_task_data
            let length_pointer: *const u8 = unsafe {
                mem::transmute(&(task_data.len() as u64))
            };

            let length_data: Vec<u8> = unsafe {
                slice::from_raw_parts(
                    length_pointer,
                    8 as usize).to_vec() // 8 bytes in a u64
            };

            native_task_data.extend(length_data);

            // write vec content to native_task_data
            native_task_data.extend(task_data.iter().cloned());
        }

        let native_tasks =
            &mut mesos_c::ProtobufObj::from_vec(native_task_data);

        let filters_data = &mut vec![];
        let native_filters = &mut mesos_c::ProtobufObj::from_message(
            filters,
            filters_data);

        let scheduler_status = unsafe {
            mesos_c::scheduler_launchTasks(
                native_driver,
                native_offer_id as *mut mesos_c::ProtobufObj,
                native_tasks as *mut mesos_c::ProtobufObj,
                native_filters as *mut mesos_c::ProtobufObj)
        };

        scheduler_status
    }

    fn revive_offers(&self) -> i32 {

        assert!(self.native_ptr_pair.is_some());
        let native_driver = self.native_ptr_pair.unwrap().driver;

        let scheduler_status = unsafe {
            mesos_c::scheduler_reviveOffers(native_driver)
        };

        scheduler_status
    }

    fn kill_task(
        &self,
        task_id: &proto::TaskID) -> i32 {

        assert!(self.native_ptr_pair.is_some());
        let native_driver = self.native_ptr_pair.unwrap().driver;

        let task_id_data = &mut vec![];
        let native_task_id = &mut mesos_c::ProtobufObj::from_message(
            task_id,
            task_id_data);

        let scheduler_status = unsafe {
            mesos_c::scheduler_killTask(
                native_driver,
                native_task_id as *mut mesos_c::ProtobufObj)
        };

        scheduler_status
    }

    fn send_framework_message(
        &self,
        executor_id: &proto::ExecutorID,
        slave_id: &proto::SlaveID,
        data: &Vec<u8>) -> i32 {

        assert!(self.native_ptr_pair.is_some());
        let native_driver = self.native_ptr_pair.unwrap().driver;

        let executor_id_data = &mut vec![];
        let native_executor_id = &mut mesos_c::ProtobufObj::from_message(
            executor_id,
            executor_id_data);

        let slave_id_data = &mut vec![];
        let native_slave_id = &mut mesos_c::ProtobufObj::from_message(
            slave_id,
            slave_id_data);

        let native_data = data.as_ptr() as *mut c_char;

        let scheduler_status = unsafe {
            mesos_c::scheduler_sendFrameworkMessage(
                native_driver,
                native_executor_id as *mut mesos_c::ProtobufObj,
                native_slave_id as *mut mesos_c::ProtobufObj,
                native_data)
        };

        scheduler_status
    }

    fn stop(
        &self,
        failover: bool) -> i32 {

        assert!(self.native_ptr_pair.is_some());
        let native_driver = self.native_ptr_pair.unwrap().driver;

        let scheduler_status = unsafe {
            mesos_c::scheduler_stop(
                native_driver,
                failover as c_int)
        };

        scheduler_status
    }

}

// Clean up backing native data structures when a MesosSchedulerDriver
// instance leaves scope.
impl<'a> Drop for MesosSchedulerDriver<'a> {
    fn drop(&mut self) {
        if self.native_ptr_pair.is_some() {
            let native_driver = self.native_ptr_pair.unwrap().driver;
            let native_scheduler = self.native_ptr_pair.unwrap().scheduler;
            unsafe {
                mesos_c::scheduler_destroy(native_driver, native_scheduler);
            }
        }
    }
}
