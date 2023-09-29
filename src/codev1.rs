use tiny_http::{Server, Response, Method};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Serialize, Deserialize)]
struct ResponsePerson {
    name: String,
    age: u8,
    message: String,
}

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    for mut request in server.incoming_requests() {
        if request.method() == &Method::Post {
            let mut content = String::new();
            request.as_reader().read_to_string(&mut content).unwrap();
            let person: Person = serde_json::from_str(&content).unwrap();
            let response_person = ResponsePerson {
                name: person.name.clone(),
                age: person.age,
                message: format!("Halo {}, umurmu sekarang {} tahun", person.name, person.age),
            };
            let response_string = serde_json::to_string(&response_person).unwrap();
            let response = Response::from_string(response_string);
            request.respond(response).unwrap();
        }
    }
}
