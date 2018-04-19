pub trait ToJSON {
    fn to_json(&self) -> String;
}

impl ToJSON for u32 {
    fn to_json(&self) -> String {
        self.to_string()
    }
}

#[test]
fn it_works() {
    assert_eq!(5.to_json(), "5");
}
