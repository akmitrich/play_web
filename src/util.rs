use serde_json::Value;

pub fn get<'a>(collection: &'a Value, path: &[&str]) -> &'a Value {
    path.iter().fold(collection, |result, key| match result {
        Value::Array(a) => match key.parse::<usize>() {
            Ok(index) => &a[index],
            Err(_) => &result[key],
        },
        _ => &result[key],
    })
}

pub fn map<F>(collection: &Value, mut f: F) -> Value
where
    F: FnMut(&Value) -> Value,
{
    match collection {
        Value::Array(a) => Value::Array(a.iter().map(f).collect()),
        Value::Object(m) => Value::Object(
            m.iter()
                .map(|(key, val)| (key.to_owned(), f(val)))
                .collect(),
        ),
        _ => collection.to_owned(),
    }
}

pub fn set(collection: &mut Value, path: &[&str], value: Value) {
    let modify = path.iter().fold(collection, |result, key| match result {
        Value::Array(a) => match key.parse::<usize>() {
            Ok(index) => &mut a[index],
            Err(_) => &mut a[0],
        },
        _ => &mut result[key],
    });
    *modify = value;
}
