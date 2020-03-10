use derive_example::ImplOkHandler;

trait OkHandler {
    fn ok(&self) -> Result<(), std::io::Error>;
}

#[derive(Default, ImplOkHandler)]
struct MyAsyncHandler {}

#[test]
fn test_derive() {
    let v = MyAsyncHandler::default();
    v.ok().unwrap();
}
