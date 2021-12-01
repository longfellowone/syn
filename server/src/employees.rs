use crate::AppData;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use chrono_tz::Canada;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

// https://serde.rs/attributes.html

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Employee {
    name: String,
    device_id: String,
    enabled: bool,
}

pub async fn list(req: HttpRequest, data: web::Data<Mutex<AppData>>) -> impl Responder {
    let time = Utc::now().with_timezone(&Canada::Pacific).to_rfc2822();
    println!("[{}] List employees", time);

    let data = data.lock().unwrap();
    let employees = data.employees.values().cloned().collect::<Vec<Employee>>();

    HttpResponse::Ok().json(employees)
}

pub async fn get(
    req: HttpRequest,
    employee: web::Path<String>,
    data: web::Data<Mutex<AppData>>,
) -> impl Responder {
    let employee = employee.into_inner();

    let time = Utc::now().with_timezone(&Canada::Pacific).to_rfc2822();
    println!("[{}] Find employee: {}", time, &employee);

    let data = data.lock().unwrap();
    let employee = data.employees.get(&employee);

    match employee {
        None => HttpResponse::NotFound().finish(),
        Some(e) => HttpResponse::Ok().json(e),
    }
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
    use crate::employees::Employee;
    use crate::test_util;
    use reqwest::StatusCode;

    #[actix_rt::test]
    async fn test_employee_get_returns_correct_employee() {
        let address = test_util::run_app();
        let client = reqwest::Client::new();

        let employee_name = String::from("wrightm");

        let response = client
            .get(&format!("{}/v1/syn/employees/{}", address, &employee_name))
            .send()
            .await
            .expect("failed to execute request to v1/syn/employees/wrightm");

        assert!(response.status().is_success());

        let response_employee = response
            .json::<Employee>()
            .await
            .expect("failed to get json employee from request");

        assert_eq!(response_employee.name, employee_name);
    }

    #[actix_rt::test]
    async fn test_employee_get_returns_404_when_employee_does_not_exist() {
        let address = test_util::run_app();
        let client = reqwest::Client::new();

        let response = client
            .get(&format!(
                "{}/v1/syn/employees/namethatdoesnotexist",
                address
            ))
            .send()
            .await
            .expect("failed to execute request to v1/syn/employees/wrightm");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(response.content_length(), Some(0));
    }
}
