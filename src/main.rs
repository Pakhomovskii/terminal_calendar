use chrono;
use core::time::Duration;
use std::io::{stdout, Write};
use std::process::Command;
use std::thread;
use chrono::prelude::*;

fn main() {
    loop {
        let dt: DateTime<Local> = Local::now();
        let year: u32 = dt.year() as u32;
        let month: u32 = dt.month();
        let day: u32 = dt.day();

        if year % 4 == 0 {
            let feb = 29;
        }

        print!("      Сентябрь\n");
        print!(" Пн Вт Ср Чт Пт Сб Вс \n");
        print!(" 1  2  3  4  5  6  7  \n"); //first line
        print!(" 8  9  10 11 12 13 14 \n"); //second line
        print!(" 15 16 17 [18] 19 20 21 \n"); //3th line
        print!(" 22 23 24 25 26 27 28 \n"); //4th line
        print!(" 29 30 31 1  2  3  4 \n"); //5th line
        print!("\n");


        loop {
            Command::new("bash")
                .args(["-c", r#"echo -n $(date +"%c")"#])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            print!("\r");
            stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }}

struct MountName {



//        //breake?
//        if x == 1 {
//            show_calendar();
//        }
//        x = x + 1;
//    }
//
//    fn show_calendar (){
//        Command::new("echo")
//            .args(["hello"])
//            .spawn()
//            .unwrap()
//            .wait()
//            .unwrap();
//        println!();
//        print!("\r");
//        stdout().flush().unwrap();}
//
//}

//let x = 1;
//
//if x == 1 {
//    analyze_days();

//
//fn analyze_days() {
//    let days = "Пн Вт Ср Чт Пт Сб Вс";
//    println!("{}", days);
//}
//
//enum Mounts {
//    jun, feb, mar, apr, may, jn, ju, aug, sep, oct, nov, dec
//}
//
//struct LeapOrNot {
//    kind: Leap,
//    address: String,
//}
//
//let home = IpAddr {
//kind: IpAddrKind::V4,
//address: String::from("127.0.0.1"),
//};
//
//let loopback = IpAddr {
//kind: IpAddrKind::V6,
//address: String::from("::1"),
//};
