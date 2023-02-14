
#[derive(Default)]
pub struct Data {
    data: Vec<i32>
}

pub fn data_new() -> Data {
    Default::default()
}

pub fn data_push(data: &mut Data, value: i32) {
    data
        .data
        .push(value)
}

pub fn data_sum(data: &Data) -> i32 {
    data
        .data
        .iter()
        .fold(0, |acc, n| acc + n)
}

pub fn data_destroy(data: *mut Data) {
    unsafe {
        if !data.is_null() {
            drop(Box::from_raw(data))
        }
    }
}