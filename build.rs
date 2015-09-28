extern crate gcc;

fn main() {
    gcc::Config::new()
        .cpp(true)
        .file("resources/mesos-c/scheduler_driver.cpp")
        .include("resources/mesos-c")
        .compile("libmesosc.a");
}
