extern crate serde;

use serde::{Serialize, Deserialize};

use serde_json::from_str as json_from_str;
use serde_json::to_string as json_to_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRequest{
    pub method: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonResponse{
    pub code: u16,
    pub message: String,
}

impl JsonResponse {
    pub fn error(message: String) -> JsonResponse {
        JsonResponse{code: 0, message}
    }

    pub fn success(message: String) -> JsonResponse {
        JsonResponse{code: 1, message}
    }

    pub fn new(code: u16, message: String) -> JsonResponse {
        JsonResponse{code, message}
    }

    pub fn serialize(&self) -> String {
        json_to_string(&self).unwrap()
    }
}

pub fn invoke(request: String) -> String {
    let request: JsonRequest = match json_from_str::<JsonRequest>(&request) {
        Ok(request) => request,
        Err(e) => { return JsonResponse::new(400, format!("Bad Request: {}", e)).serialize(); }
    };

    match request.method.as_str() {
        "marco" => JsonResponse::new(200, "polo".to_string()).serialize(),
        _ => JsonResponse::new(400, format!("Bad Request: Unknown Method: {}", request.method)).serialize()
    }
}
