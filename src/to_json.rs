extern crate serde_json;

pub trait ToJSON {
    fn to_json(&self) -> String;
}

impl ToJSON for u64 {
    fn to_json(&self) -> String {
        self.to_string()
    }
}

impl ToJSON for i64 {
    fn to_json(&self) -> String {
        self.to_string()
    }
}

impl ToJSON for f64 {
    fn to_json(&self) -> String {
        self.to_string()
    }
}

impl<'a> ToJSON for &'a str {
    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("valid string")
    }
}

#[test]
fn it_works() {
    assert_eq!((5 as u64).to_json(), "5");
    assert_eq!((5 as i64).to_json(), "5");
    assert_eq!(5.2.to_json(), "5.2");
    assert_eq!("asdf\"asdf".to_json(), "\"asdf\\\"asdf\"");
}