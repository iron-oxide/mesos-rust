#ifndef __MESOS_C_API_HPP__
#define __MESOS_C_API_HPP__

#include <stddef.h>

typedef struct {
  void* data;
  size_t size;
} ProtobufObj;


///////////////////////////////////////////////////////////////////////////////
// Function pointer type definitions for scheduler callbacks.
///////////////////////////////////////////////////////////////////////////////


typedef void* SchedulerDriverPtr;

typedef void (*scheduler_registeredCallBack_t)(
    SchedulerDriverPtr,  // SchedulerDriver
    ProtobufObj*,        // FrameworkID
    ProtobufObj*);       // MasterInfo

typedef void (*scheduler_reregisteredCallBack_t)(
    SchedulerDriverPtr,  // SchedulerDriver
    ProtobufObj*);       // MasterInfo

typedef void (*scheduler_resourceOffersCallBack_t)(
    SchedulerDriverPtr,  // SchedulerDriver
    ProtobufObj*,        // Vector<Offer>
    size_t);             // Number of offers

typedef void (*scheduler_statusUpdateCallBack_t)(
    SchedulerDriverPtr,  // SchedulerDriver
    ProtobufObj*);       // TaskStatus

typedef void (*scheduler_disconnectedCallBack_t)(
    SchedulerDriverPtr); // SchedulerDriver

typedef void (*scheduler_offerRescindedCallBack_t)(
    SchedulerDriverPtr,  // SchedulerDriver
    ProtobufObj*);       // OfferID

typedef void (*scheduler_frameworkMessageCallBack_t)(
    SchedulerDriverPtr,  // SchedulerDriver
    ProtobufObj*,        // ExecutorID
    ProtobufObj*,        // SlaveID
    ProtobufObj*);       // data part is a C string

typedef void (*scheduler_slaveLostCallBack_t)(
    SchedulerDriverPtr,  // SchedulerDriver
    ProtobufObj*);       // SlaveID

typedef void (*scheduler_executorLostCallBack_t)(
    SchedulerDriverPtr,  // SchedulerDriver
    ProtobufObj*,        // ExecutorID
    ProtobufObj*,        // SlaveID
    int);                // status

typedef void (*scheduler_errorCallBack_t)(
    SchedulerDriverPtr,  // SchedulerDriver
    ProtobufObj*);       // data part is a C string

typedef struct {
  scheduler_registeredCallBack_t        registeredCallBack;
  scheduler_reregisteredCallBack_t      reregisteredCallBack;
  scheduler_resourceOffersCallBack_t    resourceOffersCallBack;
  scheduler_statusUpdateCallBack_t      statusUpdateCallBack;
  scheduler_disconnectedCallBack_t      disconnectedCallBack;
  scheduler_offerRescindedCallBack_t    offerRescindedCallBack;
  scheduler_frameworkMessageCallBack_t  frameworkMessageCallBack;
  scheduler_slaveLostCallBack_t         slaveLostCallBack;
  scheduler_executorLostCallBack_t      executorLostCallBack;
  scheduler_errorCallBack_t             errorCallBack;
} SchedulerCallBacks;

typedef void* SchedulerCallBacksPtr;

typedef struct {
  SchedulerCallBacksPtr scheduler;
  SchedulerDriverPtr driver;
} SchedulerPtrPair;


///////////////////////////////////////////////////////////////////////////////
// Function pointer type definitions for executor callbacks.
///////////////////////////////////////////////////////////////////////////////


typedef void* ExecutorDriverPtr;

typedef void (*executor_registeredCallBack_t)(
    ExecutorDriverPtr,  // ExecutorDriver
    ProtobufObj*,       // ExecutorInfo
    ProtobufObj*,       // FrameworkInfo
    ProtobufObj*);      // SlaveInfo

typedef void (*executor_reregisteredCallBack_t)(
    ExecutorDriverPtr,  // ExecutorDriver
    ProtobufObj*);      // SlaveInfo

typedef void (*executor_disconnectedCallBack_t)(
    ExecutorDriverPtr); // ExecutorDriver

typedef void (*executor_launchTaskCallBack_t)(
    ExecutorDriverPtr,  // ExecutorDriver
    ProtobufObj*);      // TaskInfo

typedef void (*executor_killTaskCallBack_t)(
    ExecutorDriverPtr,  // ExecutorDriver
    ProtobufObj*);      // TaskID

typedef void (*executor_frameworkMessageCallBack_t)(
    ExecutorDriverPtr,  // ExecutorDriver
    const char*);       // std::string& data

typedef void (*executor_shutdownCallBack_t)(
    ExecutorDriverPtr); // ExecutorDriver

typedef void (*executor_errorCallBack_t)(
    ExecutorDriverPtr,  // ExecutorDriver
    const char*);       // std::string& message

typedef struct {
  executor_registeredCallBack_t         registeredCallBack;
  executor_reregisteredCallBack_t       reregisteredCallBack;
  executor_disconnectedCallBack_t       disconnectedCallBack;
  executor_launchTaskCallBack_t         launchTaskCallBack;
  executor_killTaskCallBack_t           killTaskCallBack;
  executor_frameworkMessageCallBack_t   frameworkMessageCallBack;
  executor_shutdownCallBack_t           shutdownCallBack;
  executor_errorCallBack_t              errorCallBack;
} ExecutorCallBacks;

typedef void* ExecutorCallBacksPtr;

typedef struct {
  ExecutorCallBacksPtr executor;
  ExecutorDriverPtr driver;
} ExecutorPtrPair;

#ifdef __cplusplus
extern "C" {
#endif


///////////////////////////////////////////////////////////////////////////////
// Scheduler driver calls.
///////////////////////////////////////////////////////////////////////////////


typedef int SchedulerDriverStatus;

SchedulerDriverStatus scheduler_launchTasks(
    SchedulerDriverPtr driver,
    ProtobufObj* offerId,  // std::vector<OfferID>
    ProtobufObj* tasks,    // std::vector<TaskInfo>
    ProtobufObj* filters); // Filters

SchedulerDriverStatus scheduler_start(
    SchedulerDriverPtr driver);

SchedulerDriverStatus scheduler_stop(
    SchedulerDriverPtr driver,
    int failover);

SchedulerDriverStatus scheduler_abort(
    SchedulerDriverPtr driver);

SchedulerDriverStatus scheduler_join(SchedulerDriverPtr driver);

SchedulerDriverStatus scheduler_run(SchedulerDriverPtr driver);

SchedulerDriverStatus scheduler_requestResources(
    SchedulerDriverPtr driver,
    ProtobufObj* requestsData); // std::vector<Request>&

SchedulerDriverStatus scheduler_declineOffer(
    SchedulerDriverPtr driver,
    ProtobufObj* offerId,  // OfferID
    ProtobufObj* filters); // Filters

SchedulerDriverStatus scheduler_killTask(
    SchedulerDriverPtr driver,
    ProtobufObj* taskId); // TaskID

SchedulerDriverStatus scheduler_reviveOffers(SchedulerDriverPtr driver);

SchedulerDriverStatus scheduler_sendFrameworkMessage(
    SchedulerDriverPtr driver,
    ProtobufObj* executor, // ExecutorID
    ProtobufObj* slaveId,  // SlaveID
    const char* data);     // std::string& data

SchedulerPtrPair scheduler_init(
    SchedulerCallBacks* callbacks, // Scheduler
    void* payload,                 // Opaque Rust scheduler pointer
    ProtobufObj* framework,        // FrameworkInfo
    const char* master);           // std::string& master

// SchedulerPtrPair scheduler_init(
//     SchedulerCallBacks* callbacks, // Scheduler
//     ProtobufObj* framework,        // FrameworkInfo
//     const char* master,            // std::string& master
//     ProtobufObj* credential);      // Credential


void scheduler_destroy(void* driver, void* scheduler);


///////////////////////////////////////////////////////////////////////////////
// Executor driver calls.
///////////////////////////////////////////////////////////////////////////////


typedef int ExecutorDriverStatus;

ExecutorDriverStatus executor_start(
    ExecutorDriverPtr driver);

ExecutorDriverStatus executor_stop(
    ExecutorDriverPtr driver);

ExecutorDriverStatus executor_abort(
    ExecutorDriverPtr driver);

ExecutorDriverStatus executor_join(
    ExecutorDriverPtr driver);

ExecutorDriverStatus executor_run(
    ExecutorDriverPtr driver);

ExecutorDriverStatus executor_sendStatusUpdate(
    ExecutorDriverPtr driver, // ExecutorDriver
    ProtobufObj* status);     // TaskStatus

ExecutorDriverStatus executor_sendFrameworkMessage(
    ExecutorDriverPtr driver, // ExecutorDriver
    const char* data);        // std::string& data

ExecutorPtrPair executor_init(
    ExecutorCallBacks* callbacks,
    void* payload);

void executor_destroy(void* driver, void* executor);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // __MESOS_C_API_HPP__

