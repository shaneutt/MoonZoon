use moon::*;

async fn init() {}

async fn frontend() -> Frontend {
    Frontend::new().title("Animation example")
}

async fn up_msg_handler(req: UpMsgRequest) {}

fn main() {
    start!(init, frontend, up_msg_handler);
}
