
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
        .sum::<i32>()
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn data_destroy(data: *mut Data) {
    unsafe {
        if !data.is_null() {
            drop(Box::from_raw(data))
        }
    }
}