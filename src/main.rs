mod arm;

extern crate time;

use time::{Tm, now};
use std::{io, env};
use std::sync::Mutex;
use actix_web::{HttpServer, App, middleware, web, Result, get, post, HttpResponse, http};
use serde::{Deserialize, Serialize};
use serialport::{DataBits, StopBits, Parity};
use crate::arm::RoboticArm;
use actix_cors::Cors;
use std::time::Duration;

#[get("/name")]
async fn hello(
  share_arm: web::Data<Mutex<RoboticArm>>
) -> Result<String> {
  let arm = share_arm.lock().unwrap();
  match arm.name() {
    Some(name) => Ok(name),
    _ => Ok(String::from("Not found"))
  }
}

#[derive(Serialize)]
struct ResponseBody {
  timestamp: String,
  command: String,
  status: String
}

impl ResponseBody {
  fn new(command: &str, message: &str) -> ResponseBody {
    ResponseBody {
      timestamp: now().rfc3339().to_string(),
      command: String::from(command),
      status: String::from(message)
    }
  }
}

#[derive(Serialize)]
struct AnglesResponseBody {
  timestamp: String,
  command: String,
  status: String,
  message: String
}

#[get("/angles")]
async fn get_angles(
  share_arm: web::Data<Mutex<RoboticArm>>
) -> HttpResponse {
  let mut arm = share_arm.lock().unwrap();
  match arm.get_angles() {
    Ok(str) => {
      println!("Get angles: {}", str);
      HttpResponse::Ok().json(AnglesResponseBody {
        command: "angles".to_string(),
        status: "OK".to_string(),
        message: str,
        timestamp: now().rfc3339().to_string()
      })
    }
    _ => HttpResponse::InternalServerError().finish()
  }
}

#[derive(Serialize, Deserialize)]
struct RotateDto {
  index: i32,
  clockwise: i32
}

#[post("/rotate")]
async fn handle_rotate(
  body: web::Json<RotateDto>,
  share_arm: web::Data<Mutex<RoboticArm>>
) -> HttpResponse {
  let mut arm = share_arm.lock().unwrap();
  match arm.rotate(body.index, body.clockwise) {
    Ok(_) => {
      println!("Rotate: {} {}", body.index, body.clockwise);
      HttpResponse::Ok().json(ResponseBody::new("rotate", "OK"))
    },
    _ => HttpResponse::InternalServerError().finish()
  }
}

#[post("/command/{command}")]
async fn handle_command(
  web::Path((command, )): web::Path<(String, )>,
  share_arm: web::Data<Mutex<RoboticArm>>
) -> HttpResponse {
  let mut arm = share_arm.lock().unwrap();
  let result = if command == String::from("hold") {
    arm.hold()
  } else if command == "put" {
    arm.put()
  } else if command == "reset" {
    arm.reset()
  } else if command == "start_conveyor_belt" {
    arm.start_conveyor_belt()
  } else if command == "stop_conveyor_belt" {
    arm.stop_conveyor_belt()
  } else if command == "carry_one_box" {
    arm.carry_one_box()
  } else if command == "carry_many_boxes" {
    arm.carry_many_boxes()
  } else if command == "change_mode" {
    arm.change_mode()
  } else {
    return HttpResponse::BadRequest().finish();
  };
  match result {
    Ok(_) => {
      println!("Command: {}", command);
      HttpResponse::Ok().json(ResponseBody::new(command.as_str(), "OK"))
    },
    _ => HttpResponse::InternalServerError().finish()
  }
}

const SERIAL_PORT_NAME: &str = "COM7";

#[actix_web::main]
async fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();

  let port_name = if args.len() >= 2 {
    args[1].as_str()
  } else {
    SERIAL_PORT_NAME
  };

  let port = serialport::new(port_name, 115200)
    .data_bits(DataBits::Eight)
    .stop_bits(StopBits::One)
    .parity(Parity::None)
    .timeout(Duration::from_millis(1000))
    .open()
    .expect("Error: Failed to connect serial port");

  println!("Connect port {} success...", port_name);

  #[allow(clippy::mutex_atomic)]
  let mut arm = web::Data::new(Mutex::new(RoboticArm::new(port)));

  println!("Server will start at http://127.0.0.1:8000");

  HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::default())
      .wrap(Cors::default()
              .allowed_origin("http://localhost:3000")
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600))
      .data(web::JsonConfig::default().limit(4096))
      .app_data(arm.clone())
      .service(hello)
      .service(get_angles)
      .service(handle_rotate)
      .service(handle_command)
  })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
