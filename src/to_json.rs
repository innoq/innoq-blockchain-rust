extern crate serde_json;

use block::{Block, Transaction};

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

impl<T> ToJSON for Vec<T>
where
    T: ToJSON,
{
    fn to_json(&self) -> String {
        let mut acc = "[".to_string();
        for elem in self {
            acc += &elem.to_json();
            acc += ",";
        }
        acc.pop();
        acc += "]";
        acc
    }
}

impl ToJSON for Transaction {
    fn to_json(&self) -> String {
        let id: &str = &self.id;
        let payload: &str = &self.payload;
        format!(r#"{{"id":{},"timestamp":{},"payload":{}}}"#,
                id.to_json(),
                self.timestamp.to_json(),
                payload.to_json())
    }
}

impl ToJSON for Block {
    fn to_json(&self) -> String {
        let mut json: String = String::from("{");
        json.push_str("}");
        json
    }
}

#[test]
fn it_works() {
    assert_eq!((5 as u64).to_json(), "5");
    assert_eq!((5 as i64).to_json(), "5");
    assert_eq!(5.2.to_json(), "5.2");
    assert_eq!("asdf\"asdf".to_json(), "\"asdf\\\"asdf\"");
    assert_eq!(
        Transaction{
            id: "\"".to_string(),
            payload: "a".to_string(),
            timestamp: 1
        }.to_json(),
        r#"{"id":"\"","timestamp":1,"payload":"a"}"#);
    assert_eq!(vec![1 as u64, 2, 3].to_json(), "[1,2,3]");
}
