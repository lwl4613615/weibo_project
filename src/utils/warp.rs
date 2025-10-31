use axum::response::IntoResponse;
use validator::ValidationErrors;

pub struct ErrorWrap(pub ValidationErrors);
impl IntoResponse for ErrorWrap {
    fn into_response(self) -> axum::response::Response {
        let hm = self.0.field_errors();
        let mut code: usize = 1;
        for (_, v) in hm {
            code = v.get(0).unwrap().code.parse::<usize>().unwrap();
            break;
        }
        crate::utils::result::response_json(code, ())
    }
}
