use wbsl::ser_servlet::SerializeServlet;

use ser_servlet::{GET, POST};

fn main() {
    SerializeServlet::builder()
        .with_func(basic_func)
        .bind("0.0.0.0:6969")
        .map(|f| f.start())
        .expect("Error starting the service");
}

fn basic_func(post: POST) -> GET {
    GET(post.0, increment())
}

fn increment() -> usize {
    unsafe {
        NUMBER += 1;
        NUMBER
    }
}

static mut NUMBER: usize = 0;
