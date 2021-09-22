use crate::utils::decode::decode;
use notify::{watcher, RecursiveMode, Watcher};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::channel;
use std::time::Duration;
/// This function is going to notify everytime there is any new write operation happens in the
/// itm.txt file.
///
/// #Arguments
/// None
///
/// #Return
/// Function notify is going to return a string.
pub fn notify() -> String {
    let (tx, rx) = channel();

    let mut watcher = match watcher(tx, Duration::from_secs(10)) {
        Ok(watcher) => watcher,
        Err(error) => panic!("SORRY! I can't watch this file {}", error),
    };

    match watcher.watch("/tmp/itm.txt", RecursiveMode::NonRecursive) {
        Ok(watcher) => watcher,
        Err(error) => panic!("No file to watch {}", error),
    };

    loop {
        match rx.recv() {
            Ok(_event) => {
                let file = match File::open("/tmp/itm.txt") {
                    Ok(file) => file,
                    Err(error) => panic!("SORRY! No such file here {}", error),
                };
                let buf_reader = BufReader::new(file);
                let content = match buf_reader.lines().last() {
                    Some(data) => match data {
                        Ok(str) => str,
                        Err(error) => {
                            panic!("Can't unwrap here {}", error)
                        }
                    },
                    None => String::from("Error here"),
                };
                println!("raw str: {:?}", content);
                println!("raw hex: {:x?}", content);
                let decoded = match decode(content.as_bytes()) {
                    Ok(decoded) => decoded,
                    Err(error) => panic!("Cannot decode this data {}", error),
                };
                let decoded_output = match std::str::from_utf8(&decoded) {
                    Ok(output) => output,
                    Err(error) => panic!("Failed to covert to str {}", error),
                };
                println!("decoded str: {:?}", decoded_output);
                return decoded_output.to_string();
            }
            Err(error) => println!("watch error: {:?}", error),
        }
    }
}
