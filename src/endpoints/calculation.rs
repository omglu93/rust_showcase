use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

#[derive(Deserialize)]
pub struct NewCheck {
    data: NewCheckData
}

#[derive(Deserialize)]
struct NewCheckData {
    pub number: Option<i32>,
}

#[derive(Serialize)]
pub struct JsonResult {
    pub results: i32,
}


#[post("/cpu-operation", data="<data_in>")]
pub fn post_cpu_operation(
    data_in: Json<NewCheck>) -> Json<JsonResult> {
        
        let number = data_in.into_inner().data.number;

        let check = cpu_intensive_function(number.unwrap());

        let response = JsonResult {
            results: check
        };

        Json(response)
}


fn cpu_intensive_function(i: i32) -> i32 {
    let mut n = 0;
    while n < i {
        n += 1
    }
    n
}
