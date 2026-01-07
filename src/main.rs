use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use std::thread;
use std::time::Duration;
use std::io::Read;

fn get_txt() -> Vec<String>{
    let mut paths = Vec::new();
    let tmp = std::fs::File::open("./explorer.txt");
    let Ok(mut f) = tmp else {return paths};
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let ps :Vec<&str> = contents.split("\r\n").collect();
    for p in ps{
        if p.is_empty(){continue;}
        paths.push(String::from(p));
    }
    return paths;
}

fn main() {
    thread::sleep(Duration::from_secs(1));
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let paths = get_txt();
    for (u,p) in paths.iter().enumerate(){
        if u == 0{
            enigo.key(Key::Windows, Press).unwrap();
            enigo.key(Key::E, Press).unwrap();
            enigo.key(Key::Windows, Release).unwrap();
            enigo.key(Key::E, Release).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        enigo.key(Key::Alt, Press).unwrap();
        enigo.key(Key::D, Press).unwrap();
        enigo.key(Key::Alt, Release).unwrap();
        enigo.key(Key::D, Release).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
        enigo
            .text(&p)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
        enigo.key(Key::Return, Press).unwrap();
        enigo.key(Key::Return, Release).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
        if u != paths.len()-1{
            enigo.key(Key::Control, Press).unwrap();
            enigo.key(Key::T, Press).unwrap();
            enigo.key(Key::Control, Release).unwrap();
            enigo.key(Key::T, Release).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(900));
        }
    }
}