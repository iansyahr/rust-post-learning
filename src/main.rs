use anyhow::Result;
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

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

fn main() -> Result<()> {
    let server = Server::http("0.0.0.0:8000").map_err(|e| anyhow::anyhow!(e))?;

    for mut request in server.incoming_requests() {
        match *request.method() {
            Method::Post => {
                let mut content = String::new();
                request.as_reader().read_to_string(&mut content)?;
                let person: Result<Person, _> = serde_json::from_str(&content);
                match person {
                    Ok(person) => {
                        let response_person = ResponsePerson {
                            name: person.name.clone(),
                            age: person.age,
                            message: format!("Halo {}, umurmu sekarang {} tahun", person.name, person.age),
                        };
                        let response_string = serde_json::to_string(&response_person)?;
                        let response = Response::from_string(response_string);
                        request.respond(response)?;
                    },
                    Err(_) => {
                        let error_response = ErrorResponse {
                            message: "Invalid JSON".to_string(),
                        };
                        let response_string = serde_json::to_string(&error_response)?;
                        let response = Response::from_string(response_string);
                        request.respond(response)?;
                    }
                }
            },
            _ => {
                let response = Response::from_string("Method not allowed");
                request.respond(response)?;
            }
        }
    }

    Ok(())
}
