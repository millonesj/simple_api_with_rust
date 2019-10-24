// rand, para generar numeros aleatorios
extern crate rand;
// iron, es un web framework
extern crate iron;
// serializa un struct a JSON
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rand::Rng;
use rustc_serialize::json;

const MY_URL: &'static str = "localhost:3009";

#[derive(RustcDecodable, RustcEncodable)]
struct LaEstructura{
    message: String,
    number: i32,
    number_float: f32,
    boolean: bool
}

fn choose_message() -> String {
    // create random number
    let num = rand::thread_rng().gen_range(1,6);

    // choose message
    let sentense_selected = match num {
        1 => "Hi JSON",
        2 => "I am using create Iron",
        3 => "I am using rusct_serialize too",
        4 => "á é í ó ú ñ",
        5 => "More special letters ü ö & °¬¬",
        _ => ""
    };

    sentense_selected.to_string()
}

fn main() {
    println!("Listening on  http://{}", MY_URL);
    Iron::new(|_: &mut Request| {
        let type_content = "application/json;charset=utf-8".parse::<Mime>().unwrap();
        let response = LaEstructura {
            message: choose_message(),
            number: rand::thread_rng().gen_range(1, 100),
            number_float: rand::thread_rng().gen_range(1.0, 100.00),
            boolean: true
        };
        let response_json = json::encode(&response).unwrap();
        Ok(Response::with((type_content, status::Ok,response_json)))
    }).http(MY_URL).unwrap();
    println!("Result: {}", choose_message());
}
