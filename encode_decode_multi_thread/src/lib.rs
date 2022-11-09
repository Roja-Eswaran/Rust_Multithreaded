use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;
// use this if depending on local crate
use libsteg;
use std::fs::OpenOptions;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
//use criterion::*;
    use std::sync::mpsc;
    use std::sync::Arc;
    use std::sync::Mutex;//extern crate project04;
//use project04::ThreadPool;
// use this if depending on precompiled library
// extern crate libsteg;
enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
//            println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

//          println!("Shutting down all workers.");

        for worker in &mut self.workers {
//            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                      //  println!("Worker {} got a job; executing.", id);

                        job.call_box();
                        //println!("{:?}",job);
                    },
                    Message::Terminate => {
                        //println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}














#[derive(Debug)]
pub enum StegError {
    BadDecode(String),
    BadEncode(String),
}
pub struct ordering_decode_message {
    arrival: u32,
    message: String,
}
/*
pub fn bench(c: &mut Criterion) {
    //let bytes : &[u8] = ...;

    let mut group = c.benchmark_group("throughput-example");
    group.throughput(Throughput::Bytes(bytes.len() as u64));
    group.bench_function("decode", |b| b.iter(|| decode(bytes));
    group.finish();
}*/


pub fn encode_module(
    thread_count: u32,
    secret_file: String,
    in_dir: String,
    out_dir: String,
) -> Result<(), StegError> {
    let mut valid_files: Vec<String> = Vec::new();
    let paths = fs::read_dir(&in_dir).unwrap();
    match fs::create_dir(&out_dir) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    let names = paths
        .map(|entry| {
            let entry = entry.unwrap();

            let entry_path = entry.path();
            let test = entry_path.to_str().unwrap();

            let file_name_as_string = String::from(test);

            file_name_as_string
        })
        .collect::<Vec<String>>();
    //  println!("File Names:");
    for i in 0..names.len() {
        let mut value = &names[i];
        let mut flag = 0;
       // println!("File Name:{}",value);
        let mut ppm = match libsteg::PPM::new(value.to_string()) {
            Ok(ppm) => {
                flag = 1;
                ppm;
            }
            Err(_) => {
                
                
         //       println!("wrong");
               // error!("{} is not a Valid PPM file and don't have secret hidden message",value);
                continue},
        };
        if flag == 1 {
            //println!("Valid:{}",names[i]);
            valid_files.push(names[i].to_string());
        }
    }

    for i in 0..valid_files.len() {
      //  println!("{}", valid_files[i]);
    }

    let message = match fs::read_to_string(&secret_file) {
        Ok(s) => s,
        Err(err) => return Err(StegError::BadEncode(err.to_string())),
    };

    let fc = required_number_files(&valid_files, &message);
   // println!("{}", fc);
    let new_message = message.clone();

    let count = &thread_count;
    // Creating Thread Pool
    let pool=ThreadPool::new(count.clone() as usize);
  //  println!("Thread Count:{}", count);
    let mut start = 0;
    let mut start_index = 0;
    let mut file_number: u128 = 0000000_u128;
    let mut total_character_message = message.chars().count();
  //  println!("Total Character Message:{}", total_character_message);
    let mut end = 0;
    let message_vec: Vec<char> = message.chars().collect();
    let mut vector_tuples_content_encode: Vec<(
        String,
        libsteg::PPM,
        u128,
        usize,
        usize,
        String,
        String,
    )> = Vec::new();
    let mut final_file_count = required_number_files(&valid_files, &message);
    for i in 0..final_file_count {
        for j in (0..valid_files.len()).step_by(*count as usize) {
            //     println!("{}", valid_files[j]);ls
         //   println!("Main Index:{}", j);
            //   /  let mut children = vec![];
            for k in 0..*count {
                let index = j + k as usize;
                if (index <= valid_files.len() - 1) {
                  //  println!("Inside Index:{}", index);
                    let ppm = match libsteg::PPM::new(valid_files[index].to_string()) {
                        Ok(ppm) => ppm,
                        Err(_) => continue,
                    };
                    end = end + (ppm.pixels.len() / 8 as usize) - 1;
                    if end > total_character_message {
                      //  println!("Greater so assignmning Back");
                        end = total_character_message - 1;
                    }
                    let out_directory_clone = out_dir.clone();
                    let end_clone = end.clone();
                    let message_clone = message.clone();
                    // let ppm_clone = ppm.clone();
                    let start_clone = start.clone();
                    let file_number_clone = file_number.clone();
                    let file_name_clone = valid_files[index].clone();
                    let tuple_values = (
                        message_clone.clone(),
                        ppm,
                        file_number_clone.clone(),
                        start_clone.clone(),
                        end_clone.clone(),
                        out_directory_clone.clone(),
                        file_name_clone,
                    );
                    vector_tuples_content_encode.push(tuple_values);
                    /* let handle = thread::spawn(move || {
                        encode_message(
                            &message_clone,
                            &ppm,
                            file_number_clone,
                            start_clone,
                            end_clone,
                            out_directory_clone,
                        );
                    });
                    children.push(handle);*/

                    if end == total_character_message - 1 {
                      //  println!("End:{}", end);
                      //  println!("Breaking");
                        break;
                    }

                    file_number = file_number + 1;
                    start = end;
                    start_index = 0;
                } else {
                    break;
                }
            }
        }
    }
    
    // Thread Joining Operation
    let max_value = vector_tuples_content_encode.len().clone();

    for j in (0..vector_tuples_content_encode.len()).step_by(*count as usize) {
        //     println!("{}", valid_files[j]);ls
        //let mut children = vec![];
        for k in 0..*count {
            let index = j + k as usize;
            if (index <= max_value - 1) {
                let message_clone = vector_tuples_content_encode[index].0.clone();
                let file_name = vector_tuples_content_encode[index].6.clone();
                let ppm = match libsteg::PPM::new(file_name) {
                    Ok(ppm) => ppm,
                    Err(_) => continue,
                };
                //let ppm_clone = &vector_tuples_content_encode[index].1;
                let file_number_clone = vector_tuples_content_encode[index].2.clone();
                let start_clone = vector_tuples_content_encode[index].3.clone();
                let end_clone = vector_tuples_content_encode[index].4.clone();
                let out_directory_clone = vector_tuples_content_encode[index].5.clone();
             //   println!("File_ Number:{}", file_number_clone);
                let handle = pool.execute(move || {
                    encode_message(
                        &message_clone,
                        &ppm,
                        file_number_clone,
                        start_clone,
                        end_clone,
                        out_directory_clone,
                    );
                });
                
            }
        }
       
    }
    Ok(())
  }
pub fn decode_module(thread_count_value: u32, in_dir: String) -> Result<(), StegError> 
{
    let mut valid_files: Vec<String> = Vec::new();
    let paths = fs::read_dir(&Path::new(&in_dir)).unwrap();
    let names = paths
        .map(|entry| {
            let entry = entry.unwrap();

            let entry_path = entry.path();
            let test = entry_path.to_str().unwrap();

            let file_name_as_string = String::from(test);

            file_name_as_string
        })
        .collect::<Vec<String>>();
        
    for i in 0..names.len() {
        let mut value = &names[i];
        
        let mut flag = 0;
        let mut ppm = match libsteg::PPM::new(value.to_string()) {
            Ok(ppm) => ppm,
            Err(_) => {
               // error!("{} is not a Valid PPM file and don't have secret hidden message",value);
                continue
            },
        };
        valid_files.push(names[i].to_string());
    }
    valid_files.sort();
    let count = &thread_count_value;
    let pool=ThreadPool::new(count.clone() as usize);

    let mut thread_count = 0;
    let (tx, rx) = mpsc::channel();
    for j in (0..valid_files.len()){
                let ppm = match libsteg::PPM::new(valid_files[j].to_string()) {
                    Ok(ppm) => ppm,
                    Err(_) => continue,
                };
        
                thread_count = thread_count + 1;
                let thread_count_clone = thread_count.clone();
                let sender_clone=tx.clone();
                let handle = pool.execute(move || {
                         
                    let content = decode_message(&ppm.pixels);
                    let mut final_content = ordering_decode_message {
                        arrival: thread_count_clone as u32,
                        message: content.clone(),
                    };
                    sender_clone.send(final_content).unwrap();
                });
            }
        let mut arrange_message = vec![];
        for i in 0..valid_files.len(){
            let recieved=rx.recv().unwrap();
            arrange_message.push(recieved);
        }
        arrange_message.sort_by_key(|x| x.arrival);
        
        for i in arrange_message {
       //     io::stdout().write(&i.message.as_bytes());
        }
    Ok(())   
}
fn write_into_file(ppm: &libsteg::PPM, bytes: &[u8], filename: u128, out_directory: String) {
    let mut file_format = ".ppm";
    let mut concat_file = format!("{:07}{}", filename, file_format.to_string());
    let mut out_file = File::create(out_directory + &"/".to_owned() + &concat_file.clone())
        .expect("unable to create file");
    //let file_path = PathBuf::from("./out").join(concat_file.clone());
    out_file.write_all(&ppm.header.magic_number);
    out_file.write_all(&"\n".as_bytes());
    out_file.write_all(ppm.header.width.to_string().as_bytes());
    out_file.write_all(&" ".as_bytes());
    out_file.write_all(ppm.header.height.to_string().as_bytes());
    out_file.write_all(&"\n".as_bytes());
    out_file.write_all(ppm.header.max_color_value.to_string().as_bytes());
    out_file.write_all(&"\n".as_bytes());
    out_file.write_all(&bytes);
    out_file.write_all(&"\n".as_bytes());
    //let file_path = PathBuf::from("/out/").join(concat_file.clone());
    // drop(out_file);*/
}

fn required_number_files(valid_files: &Vec<String>, message: &str) -> u64 {
    let mut total_character_message = message.chars().count();
    let mut required_bytes = total_character_message * 8;
    let mut available_bytes: u64 = 0;
    for i in 0..valid_files.len() {
        let mut ppm = match libsteg::PPM::new(valid_files[i].to_string()) {
            Ok(ppm) => ppm,
            Err(_) => continue,
        };
        available_bytes = available_bytes + ppm.pixels.len() as u64;
        //    println!("{}", available_bytes);
    }
    //  println!("Required Bytes:{}", required_bytes);
    // println!("Available Bytes:{}", available_bytes);
    let mut file_count: f64 = required_bytes as f64 / available_bytes as f64;
    // println!("File_count:{}", file_count);
    let null_count_possible: u64 =
        ((1_f64 - (file_count - (file_count.floor()))) * (available_bytes as f64)).floor() as u64;
    // println!("null_count_possible:{}", null_count_possible);
    let mut total_required_null_bytes: u64 = file_count.ceil() as u64 * 8;
    //  println!("Total_required_null_bytes:{}", total_required_null_bytes);
    let mut final_file_count: u64 = file_count.ceil() as u64;
    if total_required_null_bytes > null_count_possible {
        final_file_count = final_file_count + 1;
    }
    // println!("Total Required File for folding:{}", final_file_count);
    final_file_count
}
fn encode_message(
    message: &str,
    ppm: &libsteg::PPM,
    file_number: u128,
    start: usize,
    end: usize,
    out_directory: String,
) {
    let mut encoded = vec![0u8; 0];
    let mut start_index = 0;
    //    let mut file_number: u128 = 0000000_u128;
    let mut total_character_message = message.chars().count();
    let message_vec: Vec<char> = message.chars().collect();
    // println!("Total character count:{}", total_character_message);
    for char_value in start..end {
        //println!("char_value:{}",char_value);
        encoded.extend(&encode_character(
            message_vec[char_value],
            &ppm.pixels[start_index..start_index + 8],
        ));

        start_index += 8;
    }
    // println!("ended");
    encoded.extend(&encode_character(
        '\0',
        &ppm.pixels[start_index..start_index + 8],
    ));
    start_index += 8;
    encoded.extend(&ppm.pixels[start_index..]);
    // println!("File Number:{}", file_number);
    // /println!("encoded:{}",encoded);
     write_into_file(&ppm, &encoded, file_number, out_directory);
    encoded = vec![0u8; 0];
    // file_number = file_number + 1;
    //start = end;
    //start_index = 0;
}
fn encode_character(c: char, bytes: &[u8]) -> [u8; 8] {
    let c = c as u8;

    let mut ret = [0u8; 8];

    for i in 0..bytes.len() {
        if bit_set_at(c, i) {
            ret[i] = bytes[i] | 00000_0001;
        } else {
            ret[i] = bytes[i] & 0b1111_1110;
        }
    }

    ret
}

fn bit_set_at(c: u8, position: usize) -> bool {
    bit_at(c, position) == 1
}

fn bit_at(c: u8, position: usize) -> u8 {
    (c >> (7 - position)) & 0b0000_0001
}

fn decode_message(pixels: &Vec<u8>) -> String {
    let mut message = String::from("");

    for bytes in pixels.chunks(8) {
        // eprintln!("chunk!");
        if bytes.len() < 8 {
            panic!("There were less than 8 bytes in chunk");
        }

        let character = decode_character(bytes);

        if character > 127 {
            break;
        }

        if char::from(character) == '\0' {
            // eprintln!("Found terminating null!");
            break;
        }
        message.push(char::from(character));
    }
    //   println!("{}",message);
    message
    //Ok(message)
}

fn decode_character(bytes: &[u8]) -> u8 {
    if bytes.len() != 8 {
        panic!("Tried to decode from less than 8 bytes!");
    }

    let mut character: u8 = 0b0000_0000;

    for (i, &byte) in bytes.iter().enumerate() {
        if lsb(byte) {
            match i {
                0 => character ^= 0b1000_0000,
                1 => character ^= 0b0100_0000,
                2 => character ^= 0b0010_0000,
                3 => character ^= 0b0001_0000,
                4 => character ^= 0b0000_1000,
                5 => character ^= 0b0000_0100,
                6 => character ^= 0b0000_0010,
                7 => character ^= 0b0000_0001,
                _ => panic!("uh oh!"),
            }
        }
    }

    character
}

fn lsb(byte: u8) -> bool {
    (0b0000_0001 & byte) == 1
}
