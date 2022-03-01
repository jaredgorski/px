use crate::runtime::job::task::log_monitor::message::{LogMonitorCmd, LogMonitorMessage};
use crossbeam_channel::{unbounded, Receiver, Select, Sender};
use log::{error, info};
use std::{io, process::Child, string::FromUtf8Error, thread::spawn};

#[derive(Debug)]
pub enum PipeError {
    IO(io::Error),
    NotUtf8(FromUtf8Error),
}

#[derive(Debug)]
pub enum PipedLine {
    Line(String),
    Eof,
}

#[derive(Debug)]
pub struct PipeStreamReader {
    pub lines: Receiver<Result<PipedLine, PipeError>>,
}

impl PipeStreamReader {
    pub fn init(mut stream: Box<dyn io::Read + Send>) -> Self {
        Self {
            lines: {
                let (tx, rx) = unbounded();

                spawn(move || {
                    let mut buf = Vec::new();
                    let mut byte = [0_u8];
                    loop {
                        match stream.read(&mut byte) {
                            Ok(0) => {
                                let _ = tx.send(Ok(PipedLine::Eof));
                                break;
                            }
                            Ok(_) => {
                                if byte[0] == 0x0A {
                                    if let Err(error) =
                                        tx.send(match String::from_utf8(buf.clone()) {
                                            Ok(line) => Ok(PipedLine::Line(line)),
                                            Err(err) => Err(PipeError::NotUtf8(err)),
                                        })
                                    {
                                        panic!("{}", error);
                                    }

                                    buf.clear();
                                } else {
                                    buf.push(byte[0]);
                                }
                            }
                            Err(error) => {
                                if let Err(error) = tx.send(Err(PipeError::IO(error))) {
                                    panic!("{}", error);
                                }
                            }
                        }
                    }
                });

                rx
            },
        }
    }

    pub fn stream_child_output(
        child: &mut Child,
        log_monitor_senders: &[Sender<LogMonitorMessage>],
    ) {
        let channels = vec![
            Self::init(Box::new(match child.stdout.take() {
                Some(o) => o,
                None => panic!("!stdout"),
            })),
            Self::init(Box::new(match child.stderr.take() {
                Some(e) => e,
                None => panic!("!stderr"),
            })),
        ];

        let mut select = Select::new();

        for channel in &channels {
            select.recv(&channel.lines);
        }

        let mut stream_eof = false;

        while !stream_eof {
            let operation = select.select();
            let index = operation.index();
            let received = operation.recv(match &channels.get(index) {
                Some(c) => &c.lines,
                None => panic!("!lines"),
            });

            if let Ok(remote_result) = received {
                match remote_result {
                    Ok(piped_line) => match piped_line {
                        PipedLine::Line(line) => {
                            if index == 0 {
                                info!("{}", line);
                            } else {
                                error!("{}", line);
                            }

                            for sender in log_monitor_senders.iter() {
                                if let Err(error) = sender.send(
                                    LogMonitorMessage::new()
                                        .cmd(LogMonitorCmd::Log)
                                        .message(line.clone()),
                                ) {
                                    panic!("{}", error);
                                }
                            }
                        }
                        PipedLine::Eof => {
                            stream_eof = true;
                            select.remove(index);
                        }
                    },
                    Err(error) => {
                        error!("Error streaming process output: {:?}", error);
                    }
                }
            } else {
                stream_eof = true;
                select.remove(index);
            }
        }
    }
}
