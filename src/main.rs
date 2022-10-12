use std::{collections::HashMap, sync::{Mutex, Arc}};

use tokio::{net::TcpListener, io::{BufReader, AsyncBufReadExt, AsyncWriteExt}};

type Db = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
async fn main() {
    println!("Starting inmemodb on 0.0.0.0:3000");

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let db:Db = Arc::new(Mutex::new(HashMap::new()));


    loop {

        let (mut socket, addr) = listener.accept().await.unwrap();
        let db = db.clone();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            println!("{} disconnected.", addr);
                            break;
                        }

                        let line = clear_line(line.clone()).await;
                        let cmd:Vec<String> = line.split_whitespace().map(str::to_string).collect();
                        if cmd.len() == 0 {
                            break;
                        }

                        let response = match cmd[0].as_str() {
                            "QUIT" => {
                                writer.shutdown().await.unwrap();
                                ("".to_string(), false)
                            },
                            "GET" => {
                                let db = db.lock().unwrap();
                                if let Some(v) = db.get(&cmd[1]) {
                                    (format!("OK {}", v.clone()), true)
                                } else {
                                    ("OK".to_string(), true)
                                }
                            },
                            "SET" => {
                                let mut db = db.lock().unwrap();
                                let c = cmd[2..cmd.len()].join(" ");
                                db.insert(cmd[1].clone(), c);
                                ("OK".to_string(), true)
                            },
                            _ => {
                                ("unimplemented".to_string(), true)
                            }
                        };
                        if response.1 {
                            let resp = format!("{}\n", response.0);
                            writer.write_all(resp.as_bytes()).await.unwrap();
                        }
                    }
                }
                line.clear();
            }
        });

    }
}


async fn clear_line(line: String) -> String {
    let line = line.trim();
    line.to_string()
}