use axum::http::Response;

pub trait Artifact<T> {
    fn get_data(&self) -> T;
    fn into_response(&self) -> Response<T>;
}
