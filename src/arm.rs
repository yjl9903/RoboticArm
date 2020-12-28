use serialport::SerialPort;
use std::io::{ Result, Error, ErrorKind };

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

  pub fn name(&self) -> Option<String> {
    self.serialport.name()
  }

  pub fn rotate(&mut self, index: i32, clockwise: i32) -> Result<()> {
    let command = match (index, clockwise) {
      (0, 0) => 0x51,
      (0, 1) => 0x57,
      (1, 0) => 0x41,
      (1, 1) => 0x53,
      (2, 0) => 0x5a,
      (2, 1) => 0x58,
      (3, 0) => 0x52,
      (3, 1) => 0x45,
      (4, 0) => 0x46,
      (4, 1) => 0x44,
      (5, 0) => 0x56,
      (5, 1) => 0x43,
      _ => {
        return Err(Error::new(ErrorKind::InvalidInput, "Error: not a valid rotate"))
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
}