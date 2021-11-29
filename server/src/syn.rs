use crate::AppData;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use chrono_tz::Canada;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;

// https://serde.rs/attributes.html

#[derive(Debug, Serialize, Clone)]
pub struct Employee {
    name: String,
    device_id: String,
    enabled: bool,
}

pub async fn get(req: HttpRequest, data: web::Data<Mutex<AppData>>) -> impl Responder {
    let time = Utc::now().with_timezone(&Canada::Pacific).to_rfc2822();
    println!("[{}] List employees", time);

    let data = data.lock().unwrap();

    let employees = data.employees.values().cloned().collect::<Vec<Employee>>();

    HttpResponse::Ok().json(employees)
}

pub async fn find(
    req: HttpRequest,
    employee: web::Path<String>,
    data: web::Data<Mutex<AppData>>,
) -> impl Responder {
    let time = Utc::now().with_timezone(&Canada::Pacific).to_rfc2822();
    println!("[{}] Punch in/out", time);

    let data = data.lock().unwrap();
    // return full employee
    // else http not found

    HttpResponse::Ok().json("find hit")
}

// TODO: CONVERT VEC TO HASHMAP
// TODO: Use employee struct for json input and return
// TODO: Don't toggle status on server, toggle on client and update server from request
pub async fn update(
    req: HttpRequest,
    employee: web::Path<String>,
    data: web::Data<Mutex<AppData>>,
) -> impl Responder {
    let time = Utc::now().with_timezone(&Canada::Pacific).to_rfc2822();
    println!("[{}] Enable/disable employee", time);

    let data = data.lock().unwrap();

    HttpResponse::Ok().json("put hit")
}

pub async fn punchin() -> impl Responder {
    "punchin"
}
pub async fn punchout() -> impl Responder {
    "punchout"
}

pub fn employees() -> HashMap<String, Employee> {
    let matt = Employee {
        name: "wrightm".to_string(),
        device_id: "0AD03A66-92F7-4D3B-AA15-123C3FD633F7".to_string(),
        enabled: false,
    };

    let rory = Employee {
        name: "chinr".to_string(),
        device_id: "F4C88906-5A6F-4D1E-9BF2-6265E25A6071".to_string(),
        enabled: false,
    };

    let steve = Employee {
        name: "puddisters".to_string(),
        device_id: "6016B594-9327-4525-A07A-9AD4B5CBF88E".to_string(),
        enabled: false,
    };

    let joe = Employee {
        name: "kiddj".to_string(),
        device_id: "8D1D88F1-C8EF-4A75-AD3A-7C9882169341".to_string(),
        enabled: false,
    };

    let salvador = Employee {
        name: "reyess".to_string(),
        device_id: "201EEBCC-A5DF-4A23-BEF5-94DF21BE4F93".to_string(),
        enabled: false,
    };

    let mut employees = HashMap::new();

    employees.insert(matt.name.clone(), matt);
    employees.insert(rory.name.clone(), rory);
    employees.insert(steve.name.clone(), steve);
    employees.insert(joe.name.clone(), joe);
    employees.insert(salvador.name.clone(), salvador);

    employees
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util;

    // 3.7.2 - urlencoded

    #[actix_rt::test]
    async fn test_something() {
        let address = test_util::run_app();
        let client = reqwest::Client::new();

        // let response = client
        //     .get(&format!("{}/health_check", &address))
        //     .send()
        //     .await
        //     .expect("Failed to execute request.");
        //
        // assert!(response.status().is_success());
        // assert_eq!(Some(0), response.content_length());
    }
}
