extern crate gcc;

fn main() {
    let sources = &[
        "resources/mesos-c/utils.hpp",
        "resources/mesos-c/scheduler_driver.cpp",
    ];

    gcc::compile_library("libmesosc.a", sources);
}
