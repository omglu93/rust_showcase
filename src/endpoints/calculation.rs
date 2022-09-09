use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

#[derive(Deserialize)]
pub struct NewCheck {
    data: NewCheckData
}

#[derive(Deserialize)]
struct NewCheckData {
    pub number: Option<u32>,
}

#[derive(Serialize)]
pub struct JsonResult {
    pub bool: bool,
}


#[get("/prime", data="<new_check>")]
pub fn get_if_prime(
    new_check: Json<NewCheck>) -> Json<JsonResult> {
        
        let number = new_check.into_inner().data.number;

        let check = check_if_prime(number.unwrap());

        let response = JsonResult {
            bool: check
        };

        Json(response)
}


fn check_if_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    true
}
