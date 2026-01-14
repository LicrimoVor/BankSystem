use socket2::{Domain, Protocol, Socket, Type};
use std::io::{self, BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const URL: &str = "127.0.0.1:7878";
const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(10);

// Подключение к серверу
fn connect() -> io::Result<(TcpStream, BufReader<TcpStream>)> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;

    socket.set_keepalive(true)?;
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        socket.set_tcp_keepalive(
            &socket2::TcpKeepalive::new()
                .with_time(Duration::from_secs(10))
                .with_interval(Duration::from_secs(5)),
        )?;
    }

    let addr: SocketAddr = URL.parse().unwrap();
    socket.connect(&addr.into())?;

    let stream: TcpStream = socket.into();
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut reader = BufReader::new(stream.try_clone()?);

    // Читаем welcome message один раз
    let mut line = String::new();
    reader.read_line(&mut line)?;
    print!("{}", line);

    println!("Connected to server!");
    Ok((stream, reader))
}

// Реконнект
fn reconnect() -> (TcpStream, BufReader<TcpStream>) {
    loop {
        match connect() {
            Ok(pair) => return pair,
            Err(e) => {
                eprintln!("Reconnect failed: {}. Retrying in 2s...", e);
                thread::sleep(Duration::from_secs(2));
            }
        }
    }
}

// Отправка команды
fn send_command(
    stream: &mut TcpStream,
    reader: &mut BufReader<TcpStream>,
    command: &str,
) -> io::Result<String> {
    stream.write_all(command.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;

    let mut buffer = String::new();
    let bytes = reader.read_line(&mut buffer)?;
    if bytes == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Server closed connection",
        ));
    }
    Ok(buffer)
}

// Отправка PING
fn send_ping(stream: &mut TcpStream, reader: &mut BufReader<TcpStream>) -> io::Result<u64> {
    let start = Instant::now();
    stream.write_all(b"PING\n")?;
    stream.flush()?;

    let mut buffer = String::new();
    let bytes = reader.read_line(&mut buffer)?;
    if bytes == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Server closed connection",
        ));
    }

    if buffer.trim() == "PONG" {
        Ok(start.elapsed().as_millis() as u64)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid response: {}", buffer),
        ))
    }
}

fn main() -> io::Result<()> {
    let (stream, reader) = connect()?;
    let latency = Arc::new(Mutex::new(1000u64));
    let keepalive_flag_end = Arc::new(Mutex::new(false));
    let stream = Arc::new(Mutex::new(stream));
    let reader = Arc::new(Mutex::new(reader));

    // Keepalive-поток
    let thread = {
        let stream_clone = Arc::clone(&stream);
        let reader_clone = Arc::clone(&reader);
        let latency_clone = Arc::clone(&latency);
        let keepalive_flag_end = keepalive_flag_end.clone();

        thread::spawn(move || {
            loop {
                std::thread::sleep(KEEP_ALIVE_INTERVAL);

                if *keepalive_flag_end.lock().unwrap() {
                    break;
                }

                let mut stream = stream_clone.lock().unwrap();
                let mut reader = reader_clone.lock().unwrap();
                match send_ping(&mut stream, &mut reader) {
                    Ok(latency) => {
                        *latency_clone.lock().unwrap() = latency;
                    }
                    Err(err) => {
                        println!("Keepalive failed. Reconnecting...");
                        println!("{}", err);
                        if let Ok((stream_new, reader_new)) = connect() {
                            *stream = stream_new;
                            *reader = reader_new;
                        } else {
                        }
                    }
                };
            }
        })
    };

    // Основной интерактивный цикл
    let stdin = io::stdin();
    loop {
        print!("vault> ");
        io::stdout().flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let command = input.trim();

        if command.is_empty() {
            continue;
        }
        if command.eq_ignore_ascii_case("EXIT") {
            println!("Bye!");
            break;
        }

        let mut s = stream.lock().unwrap();
        let mut r = reader.lock().unwrap();

        match send_command(&mut *s, &mut *r, command) {
            Ok(resp) => print!("{}", resp),
            Err(e) => {
                eprintln!("Command failed: {}. Reconnecting...", e);
                let (new_s, new_r) = reconnect();
                *s = new_s;
                *r = new_r;
            }
        }
    }

    let mut keepalive_flag_end = keepalive_flag_end.lock().unwrap();
    *keepalive_flag_end = true;

    thread.join().unwrap();

    Ok(())
}
