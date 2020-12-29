use serialport::SerialPort;
use std::io::{Result, Error, ErrorKind};

pub struct RoboticArm {
  serialport: Box<dyn SerialPort>
}

fn make_command(command: i8) -> Vec<u8> {
  command.to_be_bytes().to_vec()
}

impl Clone for RoboticArm {
  fn clone(&self) -> RoboticArm {
    RoboticArm {
      serialport: self.serialport.try_clone().expect("Error: clone fail")
    }
  }
}

impl RoboticArm {
  pub fn new(serialport: Box<dyn SerialPort>) -> RoboticArm {
    RoboticArm {
      serialport
    }
  }

  fn send(&mut self, command: i8) -> Result<()> {
    match self.serialport.write(make_command(command).as_slice()) {
      Ok(_) => Ok(()),
      Err(e) => Err(e)
    }
  }

  fn receive(&mut self) -> Result<String> {
    let mut serial_buf: Vec<u8> = vec![0; 128];
    match self.serialport.read(serial_buf.as_mut_slice()) {
      Ok(_) => match String::from_utf8(serial_buf) {
        Ok(str) => Ok(str),
        Err(_) => {
          println!("Error: receive invalid data");
          Err(Error::new(ErrorKind::InvalidData, "Error: receive invalid data"))
        }
      },
      Err(e) => {
        println!("Error: failed to receive data");
        Err(e)
      }
    }
  }

  pub fn get_angles(&mut self) -> Result<String> {
    match self.send(0x7f) {
      Ok(_) => self.receive(),
      Err(e) => Err(e)
    }
  }

  pub fn name(&self) -> Option<String> {
    self.serialport.name()
  }

  pub fn rotate(&mut self, index: i32, clockwise: i32) -> Result<()> {
    let command = match (index, clockwise) {
      (0, 0) => 0x57,
      (0, 1) => 0x51,
      (1, 0) => 0x53,
      (1, 1) => 0x41,
      (2, 0) => 0x58,
      (2, 1) => 0x5a,
      (3, 0) => 0x45,
      (3, 1) => 0x52,
      (4, 0) => 0x44,
      (4, 1) => 0x46,
      (5, 0) => 0x43,
      (5, 1) => 0x56,
      _ => {
        return Err(Error::new(ErrorKind::InvalidInput, "Error: not a valid rotate"));
      }
    };
    self.send(command)
  }

  pub fn reset(&mut self) -> Result<()> {
    self.send(0x66)
  }

  pub fn hold(&mut self) -> Result<()> {
    self.send(0x4f)
  }

  pub fn put(&mut self) -> Result<()> {
    self.send(0x50)
  }

  pub fn start_conveyor_belt(&mut self) -> Result<()> {
    self.send(0x4b)
  }

  pub fn stop_conveyor_belt(&mut self) -> Result<()> {
    self.send(0x4c)
  }

  pub fn carry_one_box(&mut self) -> Result<()> {
    self.send(0x64)
  }

  pub fn carry_many_boxes(&mut self) -> Result<()> {
    self.send(0x65)
  }

  pub fn change_mode(&mut self) -> Result<()> {
    self.send(0x7c)
  }
}