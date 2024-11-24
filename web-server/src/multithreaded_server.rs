use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread,
};

const THREAD_COUNT: usize = 4;

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind the addr");
    println!("Server is running on http://127.0.0.1:8080");

    //create a Thread pool
    let pool = ThreadPool::new(THREAD_COUNT);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed : {}", e);
            }
        }
    }
}

//handles a single client connection
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            // parse the Http request (basics)
            let request = String::from_utf8_lossy(&buffer);
            println!("Received request:\n {}", request);

            //Basic response
            let response = if request.starts_with("GET / HTTP/1.1") {
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nWelcome to the custom Rust Server"
            } else {
                "HTTP/1.1 404 NOT_FOUND\r\nContent-Type: text/plain\r\n\r\n404 not Found"
            };

            stream
                .write(response.as_bytes())
                .expect("Failed to wriet to stream");
            stream.flush().unwrap();
        }
        Err(e) => {
            eprintln!("Failed to read from stream {}", e);
        }
    }
}
// Custome Threadpool implmentation

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).expect("Failed to send job to worker")
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    println!("Worker {} got a job , executing", id);
                    job();
                }
                Err(_) => {
                    println!("Worker {} disconnected: Shutting Down.", id);
                    break;
                }
            }
        });
        Self {
            id,
            thread: Some(thread),
        }
    }
}
