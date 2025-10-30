use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use axum::response::Response;

use crate::config::status_code::STATUS_CODE_MSG;
#[derive(Serialize)]
struct ResponseData<T:Serialize>{
    code:usize,
    message:String,
    data:T
}
impl<T:Serialize> ResponseData<T>
{
    pub fn new(code:usize,data:T)->Self{
        let message=STATUS_CODE_MSG.get().unwrap().get(&code).unwrap_or(&"Unknown Status Code").to_string();
        ResponseData{
            code,
            message,
            data
        }
    }
}

pub fn response_json<T:Serialize>(code:usize,data:T)->Response{
    let res=ResponseData::new(code,data);
    let mut data=String::from("");
    if let Ok(json)=serde_json::to_string(&res){
        data=json;
    }

    Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type","application/json;charset=utf-8")
    .body(data)
    .unwrap()
    .into_response()
    
}