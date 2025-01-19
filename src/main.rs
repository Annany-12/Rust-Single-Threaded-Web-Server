use std::fs;
use std::io;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn menu(){
    println!("Enter 1 to Run Server\nEnter 2 to Upload Your Files");
    let inp: String = input_taker(String::from("Input: "));
    if inp == String::from("1"){
        println!("Running Server...");
        let dir_content: Vec<String> = fs::read_dir("page_save")
        .unwrap().filter_map(|entry| { 
            entry.ok().and_then(|e| e.file_name().into_string().ok())
        }).collect();

        let mut ctr: i32 = 1;
        println!("\nCurrently serving Files: ");
        for file in dir_content{
            println!("{ctr}: {file}");
            ctr = ctr + 1;
        }
        main_server();
    }
    else if inp == String::from("2"){
        println!("Upload Files");
        file_upload_function();
    }
    else {
        println!("Invalid Input.");
        menu();
    }
}

fn input_taker(message: String) -> String{
    println!("{}", message);
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("");
    let inp: String = inp.trim().parse().expect("");
    inp
}

fn file_upload_function() -> Vec<String>{
    let file_name = input_taker(String::from("Enter File Name"));
    
    let file_path = input_taker(String::from("Enter Path"));

    let content = fs::read_to_string(file_path);
    let content = match content{
        Ok(val) => val,
        Err(err) => {
            err.to_string();
            panic!();
        }
    };

    let mut file_name_vec: Vec<String> = fs::read_dir("page_save")
    .unwrap().filter_map(|entry| { 
        entry.ok().and_then(|e| e.file_name().into_string().ok())
    }).collect();


    if file_name_vec.contains(&format!("{file_name}.html")){
        println!("File already Exists");
        panic!();
    }

    if !file_name_vec.contains(&file_name) {
        file_name_vec.push(format!("{file_name}.html"));
        let mut dir_save_path = fs::File::create(format!("page_save\\{file_name}.html")).unwrap();

        if let Err(err) = dir_save_path.write_all(content.as_bytes()){
            println!("Error in writing data from specified file: {}", err);
            panic!();
        }
    }
    else if file_name_vec.contains(&format!("{file_name}.html")){
        println!("File already Exists");
    }
    println!("{:?}", file_name_vec);
    file_name_vec
}

fn handle_conn(mut open_conn: TcpStream) {
    let buf_reader = BufReader::new(&open_conn);

    let request_line = buf_reader.lines().next();

    let request_line = match request_line {
        Some(rl) => match rl {
            Ok(rl) => rl,
            Err(err) => {
                println!("Error in Result Enum of Request Line: {}", err);
                String::from("Error in Result Enum of Request Line")
            }
        },
        None => {
            println!("Error in Request Line");
            String::from("Error in Request Line")
        },
    };

    let file_names: Vec<String> = match fs::read_dir("page_save") {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .collect(),
        Err(err) => {
            println!("Error reading directory 'page_save': {}", err);
            return;
        }
    };

    let requested_file = if let Some(path) = request_line.split_whitespace().nth(1) {
        path.trim_start_matches('/')
    } else {
        println!("Malformed request line: {}", request_line);
        return;
    };

    let (status_line, filename) = if requested_file.is_empty() || requested_file == "/" {
        ("HTTP/1.1 200 OK", "hello.html")
    } 
    else if file_names.contains(&requested_file.to_string()) {
        ("HTTP/1.1 200 OK", requested_file)
    } 
    else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    println!("Serving file: {}", filename);
    println!("===================");

    let content = match fs::read_to_string(format!("page_save/{}", filename)) {
        Ok(content) => content,
        Err(err) => {
            println!("Error While Reading the Content: {}", err);
            return;
        }
    };    

    let content_len = content.len();

    let resp = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, content_len, content
    );

    if let Err(err) = open_conn.write_all(resp.as_bytes()) {
        println!("Error writing to connection: {}", err);
    }
}


fn main_server() {
    let listner = TcpListener::bind("127.0.0.1:7878");

    // Comment the above listner and un-comment the below listner.
    // From devices on the same Wifi go to "http://192.168.29.212:7878" where the IP is the system's IP address and Port is the specified Port.
    // The server will serve files to the devices on the same WIFI as well.
    
    // let listner = TcpListener::bind("0.0.0.0:7878");

    let listner = match listner {
        Ok(listner) => listner,
        Err(err) => {
            println!("Error Occoured While Getting Data{}", err);
            return;
        }
    };

    for stream in listner.incoming() {
        match stream {
            Ok(open_conn) => {
                if let Ok(addr) = open_conn.peer_addr() {
                    println!("Client connected: {}", addr);
                } else {
                    println!("Failed to retrieve client's address.");
                }
                handle_conn(open_conn);
            }
            Err(err) => {
                println!("Error while listening for data: {}", err);
            }
        }
    }
    

}

fn main(){
    println!("Single Threaded Web Server: ");
    menu();
}