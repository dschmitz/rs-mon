mod protos;

use crate::protos::telemetry::Telemetry_Disk;
use log::info;

//use protobuf::{parse_from_bytes, Message};
use protos::telemetry::{Telemetry, Telemetry_LoadAvg};
use sysinfo::{DiskExt, System, SystemExt};

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let mut system = System::new_all();

    // Encode example request
    let mut tmsg = Telemetry::new();

    tmsg.ts = 0;

    //let out_bytes: Vec<u8> = out_msg.write_to_bytes().unwrap();

    // First we update all information of our system struct.
    system.refresh_all();

    let load_avg = system.get_load_average();
    let mut load = Telemetry_LoadAvg::new();
    load.one = load_avg.one;
    load.five = load_avg.five;
    load.fifteen = load_avg.fifteen;
    log::info!(
        "1_min: {:.3}%, 5_min: {:.3}%, 15_min: {:.3}%",
        load_avg.one,
        load_avg.five,
        load_avg.fifteen
    );

    // Then let's print the temperature of the different components:
    for component in system.get_components() {
        info!("{:?}", component);
    }

    // And then all disks' information:
    for disk in system.get_disks() {
        info!("{:?}", disk);
    }

    for disk in system.get_disks() {
        let mut drive = Telemetry_Disk::new();
        //        let mount_point = "unknown".to_string(); //format!("{}", disk.get_mount_point().to_str());
        let space_total = disk.get_total_space();
        let space_available = disk.get_available_space();
        let space_used_percent = ((space_available as f64 / space_total as f64) * 100.0) as f32;
        //      drive.mount_point = mount_point;
        drive.space_total = space_total;
        drive.space_used = space_available;
        drive.space_used_percent = space_used_percent;
        //    println!("mount point        : {}", mount_point);
        info!("space total        : {}", space_total);
        info!("space available    : {}", space_available);
        info!("space used percent : {:.3}", space_used_percent);
    }

    // And finally the RAM and SWAP information:
    let total_memory = system.get_total_memory() as f64;
    let used_memory = system.get_used_memory() as f64;
    info!("total memory: {} KiB", total_memory);
    info!("used memory : {} KiB", used_memory);
    info!("usage in %  : {:.3}", (used_memory / total_memory * 100.0));
    info!("total swap  : {} KiB", system.get_total_swap());
    info!("used swap   : {} KiB", system.get_used_swap());
}
