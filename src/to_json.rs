extern crate serde_json;

use block::{Block, Blockchain, Transaction};

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

impl<'a, T> ToJSON for &'a Vec<T>
where
    T: ToJSON,
{
    fn to_json(&self) -> String {
        let mut acc = "[".to_string();
        for elem in self.iter() {
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
        format!(
            r#"{{"id":{},"timestamp":{},"payload":{}}}"#,
            self.id.as_str().to_json(),
            self.timestamp.to_json(),
            self.payload.as_str().to_json()
        )
    }
}

impl ToJSON for Blockchain {
    fn to_json(&self) -> String {
        format!(
            r#"{{"blocks":{},"blockHeight":{}}}"#,
            (&self.blocks).to_json(),
            self.block_height.to_json()
        )
    }
}

impl ToJSON for Block {
    fn to_json(&self) -> String {
        let mut json: String = String::from("{");
        add_property(&mut json, "index", self.index, false);
        add_property(&mut json, "timestamp", self.timestamp, false);
        add_property(&mut json, "proof", self.proof, false);
        add_property(&mut json, "transactions", &self.transactions, false);
        add_property(&mut json, "previousBlockHash", self.previous_block_hash.as_str(), true);
        json.push_str("}");
        json
    }
}

fn add_property<V>(json: &mut String, name: &str, value: V, last: bool)
where
    V: ToJSON,
{
    json.push_str("\"");
    json.push_str(name);
    json.push_str("\":");
    json.push_str(value.to_json().as_str());
    if !last {
        json.push_str(",");
    }
}

#[test]
fn it_works() {
    assert_eq!((5 as u64).to_json(), "5");
    assert_eq!((5 as i64).to_json(), "5");
    assert_eq!(5.2.to_json(), "5.2");
    assert_eq!("asdf\"asdf".to_json(), r#""asdf\"asdf""#);
    assert_eq!(
        Transaction {
            id: "\"".to_string(),
            payload: "a".to_string(),
            timestamp: 1,
        }.to_json(),
        r#"{"id":"\"","timestamp":1,"payload":"a"}"#
    );
    assert_eq!((&vec![1 as u64, 2, 3]).to_json(), "[1,2,3]");
}

#[test]
fn genesis() {
    let chain = Blockchain::new();
    assert_eq!(
        chain.to_json(),
        r#"{"blocks":[{"index":1,"timestamp":0,"proof":1917336,"transactions":[{"id":"b3c973e2-db05-4eb5-9668-3e81c7389a6d","timestamp":0,"payload":"I am Heribert Innoq"}],"previousBlockHash":"0"}],"blockHeight":1}"#);
}
