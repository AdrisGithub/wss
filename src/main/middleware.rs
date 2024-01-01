use whdp::{Request, Response};
/// # Safety
/// this is really unsafe
pub unsafe trait Middleware {
    fn on_request(&self, req: Request) -> Request {
        req
    }
    fn on_response(&self, resp: Response) -> Response {
        resp
    }
}
