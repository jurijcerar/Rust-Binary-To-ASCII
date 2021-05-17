use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::os::unix::io::{FromRawFd};

fn check_flags(args: &mut Vec<String>) -> Vec<String> {

    let mut input_file = String::from("0");
    let mut output_file = String::from("1");
    let mut input_type = String::from("desc");
    let mut output_type = String::from("desc");

    let mut i = 0;

    while i < args.len() {
        if args[i] == "-i" {
            input_type = String::from("file");
            input_file = args[i+1].clone();
        }
        else if args[i] == "-I" {
            input_type = String::from("desc");
            input_file = args[i+1].clone();
        }
        else if args[i] == "-o" {
            output_type = String::from("file");
            output_file = args[i+1].clone();
        }
        else if args[i] == "-O" {
            output_type = String::from("desc");
            output_file = args[i+1].clone();
        }

        i=i+1;
    }
        
    let vec = [input_type, input_file, output_type, output_file].to_vec();

    return vec;
}

fn write_file(filename: &String, buffer : Vec<u8>){
    let mut file = File::create(filename).unwrap();
    let mut msg = String::from("");
    let mut count = 0;
    for i in buffer {
        if count == 0 {
            msg = i.to_string();
            count += 1;
        }
        else {
            msg = format!("{} {}", msg, i.to_string());
        }
    }
    file.write_all(msg.as_bytes()).expect("Napaka pri vpisu!");
}

fn read_file(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("Ta datoteka ne obstaja"); //odpremo datoteko
    let metadata = fs::metadata(&filename).expect("Ni mozno prebrat meta podatkov"); //preberemo meta podatke
    let mut buffer = vec![0; metadata.len() as usize]; //deklariramo vektor s pravilno velikostjo in tipom
    let n = f.read(&mut buffer).expect("buffer overflow"); //podatke preberemo v vektor
    buffer = buffer[0..n].to_vec();
    return buffer; //vreno bektor
}

fn read_file_desc(filename: i32) -> Vec<u8> {
    let mut f = unsafe { File::from_raw_fd(filename) }; //odpremo datoteko
    let mut buffer = vec![0; 1000 as usize]; //deklariramo vektor s pravilno velikostjo in tipom
    let n = f.read(&mut buffer).expect("buffer overflow"); //podatke preberemo v vektor
    buffer = buffer[0..n].to_vec();
    return buffer; //vreno bektor
}

fn write_file_desc(filename: i32, buffer : Vec<u8>){
    let mut f = unsafe { File::from_raw_fd(filename) };
    let mut msg = String::from("");
    let mut count = 0;
    for i in buffer {
        if count == 0 {
            msg = i.to_string();
            count += 1;
        }
        else {
            msg = format!("{} {}", msg, i.to_string());
        }
    }
    writeln!(f, "{}", msg);
}

fn main() {

    let mut args: Vec<String> = env::args().collect();
    let vec = check_flags(&mut args);
    let buffer : Vec <u8>;

    if vec[0] == "file" {
        buffer = read_file(&vec[1]);
    }
    else{
        let num = vec[1].parse::<i32>().unwrap();
        buffer = read_file_desc(num);
    }

    if vec[2] == "file" {
        write_file(&vec[1], buffer);
    }
    else{
        let num = vec[3].parse::<i32>().unwrap();
        write_file_desc(num, buffer);
    }
    
}
