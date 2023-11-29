use std::process::Command;

pub fn clear() {
    match Command::new("clear").status() {
        Ok(_) => {},
        Err(_) => {
            match Command::new("cl").status() {
                Ok(_) => {},
                Err(_) => {
                    for _ in 0..100 {
                        println!()
                    }
                },
            }
        },
    }
}