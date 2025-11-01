use axum::response::IntoResponse;
use validator::ValidationErrors;

pub struct ErrorWrap(pub ValidationErrors);
impl IntoResponse for ErrorWrap {
    fn into_response(self) -> axum::response::Response {
        let hm = self.0.field_errors();
        let mut code: usize = 1;
        if let Some((_, v)) = hm.into_iter().next() {
            code = v.first().unwrap().code.parse::<usize>().unwrap();
            
        }
        crate::utils::result::response_json(code, ())
    }
}
