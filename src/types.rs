use std::collections::HashMap;

pub type Headers = HashMap<String, String>;
pub type MultiValueHeaders = HashMap<String, Vec<String>>;

pub type PathParameters = HashMap<String, String>;

pub type QueryStringParameters = HashMap<String, String>;
pub type MultiValueQueryStringParameters = HashMap<String, Vec<String>>;
