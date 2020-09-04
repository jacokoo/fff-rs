pub enum Answer {
    Yes,
    No,
    YesToAll,
}

pub struct Context {}

impl Context {
    async fn request_input(_msg: &str) -> String {
        unimplemented!()
    }

    async fn request_answer(_msg: &str, _multiple: bool) -> Answer {
        unimplemented!()
    }
}
