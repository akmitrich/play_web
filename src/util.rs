use serde_json::{json, Value};

pub fn get<'a>(collection: &'a Value, path: &[&str]) -> &'a Value {
    path.iter().fold(collection, |result, key| match result {
        Value::Array(a) => match key.parse::<usize>() {
            Ok(index) => &a[index],
            Err(_) => &result[key],
        },
        _ => &result[key],
    })
}

pub fn get_mut<'a>(collection: &'a mut Value, path: &[&str]) -> &'a mut Value {
    path.iter().fold(collection, |result, key| match result {
        Value::Array(a) => match key.parse::<usize>() {
            Ok(index) => &mut a[index],
            Err(_) => &mut a[0],
        },
        _ => &mut result[key],
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
    let modify = get_mut(collection, path);
    *modify = value;
}

pub fn get_num(collection: &Value, path: &[&str]) -> Option<f64> {
    if let Value::Number(num) = get(collection, path) {
        num.as_f64()
    } else {
        None
    }
}

pub fn set_num(collection: &mut Value, path: &[&str], value: f64) {
    let num_val = json!(value);
    set(collection, path, num_val);
}

pub fn iterate<F>(collection: &Value, f: F)
where
    F: Fn(&Value),
{
    match collection {
        Value::Array(a) => {
            for v in a {
                f(v)
            }
        }
        Value::Object(o) => {
            for v in o.values() {
                f(v)
            }
        }
        _ => {}
    }
}

pub fn iterate_mut<F>(collection: &mut Value, f: F)
where
    F: Fn(&mut Value),
{
    match collection {
        Value::Array(a) => {
            for v in a {
                f(v)
            }
        }
        Value::Object(o) => {
            for v in o.values_mut() {
                f(v)
            }
        }
        _ => {}
    }
}
