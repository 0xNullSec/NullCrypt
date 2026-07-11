use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    sync::{Arc, Mutex},
};
use std::thread;

#[derive(Clone, Debug)]
pub struct Peer {
    ip: String,
    port: u16,
}

type Index = Arc<Mutex<HashMap<String, Vec<Peer>>>>;

pub fn run_server(bind: &str) -> std::io::Result<()> {
    let index: Index = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind(bind)?;
    println!("Index listening on {bind}");

    for conn in listener.incoming() {
        let idx = index.clone();
        thread::spawn(move || {
            if let Ok(mut stream) = conn {
                if let Err(e) = handle_server_conn(&mut stream, idx) {
                    eprintln!("Server err: {e}");
                }
            }
        });
    }
    Ok(())
}

fn handle_server_conn(stream: &mut TcpStream, index: Index) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();

    reader.read_line(&mut line)?;
    let parts: Vec<_> = line.trim_end().split_whitespace().collect();
    if parts.len() != 2 || parts[0] != "HELLO" {
        return Ok(());
    }
    let peer = Peer {
        ip: stream.peer_addr()?.ip().to_string(),
        port: parts[1].parse().unwrap_or(0),
    };

    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        let words: Vec<_> = line.trim_end().splitn(2, ' ').collect();
        match words.as_slice() {
            ["FILE", f] => {
                index
                    .lock()
                    .unwrap()
                    .entry(f.to_string())
                    .or_default()
                    .push(peer.clone());
            }
            ["SEARCH", f] => {
                let peers = index.lock().unwrap();
                if let Some(list) = peers.get(*f) {
                    for p in list {
                        writeln!(stream, "{}:{}", p.ip, p.port)?;
                    }
                }
                writeln!(stream, "END")?;
            }
            ["DONE"] => break,
            _ => {}
        }
    }
    Ok(())
}

fn file_server(dir: PathBuf, port: u16) {
    thread::spawn(move || {
        let listener = TcpListener::bind(("0.0.0.0", port)).expect("bind file server");
        println!("File server on 0.0.0.0:{port}");
        for conn in listener.incoming() {
            if let Ok(mut s) = conn {
                thread::spawn({
                    let d = dir.clone();
                    move || {
                        if let Err(e) = handle_file_req(&mut s, d) {
                            eprintln!("file tx err: {e}");
                        }
                    }
                });
            }
        }
    });
}

fn handle_file_req(stream: &mut TcpStream, dir: PathBuf) -> std::io::Result<()> {
    let mut line = String::new();
    BufReader::new(stream.try_clone()?).read_line(&mut line)?;
    let parts: Vec<_> = line.trim_end().splitn(2, ' ').collect();
    if parts.len() != 2 || parts[0] != "GET" {
        return Ok(());
    }
    let mut path = dir.clone();
    path.push(parts[1]);
    let mut file = fs::File::open(path)?;
    std::io::copy(&mut file, stream)?;
    Ok(())
}

pub fn run_client(index_addr: &str, share_dir: &str, listen_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let dir = PathBuf::from(share_dir);
    file_server(dir.clone(), listen_port);

    let mut idx = TcpStream::connect(index_addr)?;
    writeln!(idx, "HELLO {listen_port}")?;

    for entry in fs::read_dir(&dir)? {
        let f = entry?.file_name();
        let name = f.to_string_lossy();
        writeln!(idx, "FILE {name}")?;
    }
    writeln!(idx, "DONE")?;
    println!("Registered with index {index_addr}");

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let cmd = line?;
        let parts: Vec<_> = cmd.split_whitespace().collect();
        if parts.len() == 2 && parts[0] == "find" {
            let song = parts[1];
            writeln!(idx, "SEARCH {song}")?;
            let mut reader = BufReader::new(idx.try_clone()?);
            loop {
                let mut res = String::new();
                reader.read_line(&mut res)?;
                let target = res.trim_end();
                if target == "END" || target.is_empty() {
                    break;
                }
                println!("found at {target}");
                if let Some(p) = target.split_once(':') {
                    download(song, p.0, p.1.parse().unwrap(), &dir)?;
                    break;
                }
            }
        } else {
            println!("usage: find <filename>");
        }
    }
    Ok(())
}

fn download(song: &str, host: &str, port: u16, dir: &PathBuf) -> std::io::Result<()> {
    println!("Downloading {song} from {host}:{port}");
    let mut stream = TcpStream::connect((host, port))?;
    writeln!(stream, "GET {song}")?;
    let mut out = fs::File::create(dir.join(song))?;
    std::io::copy(&mut stream, &mut out)?;
    println!("saved {}", dir.join(song).display());
    Ok(())
}
