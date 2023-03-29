use ouroboros::self_referencing;

#[self_referencing]
struct MyStruct {
    int_data: i32,
    float_data: f32,
    #[borrows(int_data)]
    int_reference: &'this i32,
    #[borrows(mut float_data)]
    float_reference: &'this mut f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run() {
        let mut my_value = MyStructBuilder {
            int_data: 42,
            float_data: 3.14,
            int_reference_builder: |int_data: &i32| int_data,
            float_reference_builder: |float_data: &mut f32| float_data,
        }
        .build();
        assert_eq!(42, my_value.borrow_int_data().clone());
        assert_eq!(42, my_value.borrow_int_data().clone());
        assert_eq!(42, my_value.borrow_int_data().clone());
        assert_eq!(42, my_value.borrow_int_data().clone());
        assert_eq!(42, my_value.borrow_int_data().clone());

        assert_eq!(
            3.14.to_string(),
            my_value.borrow_float_reference().to_string()
        );

        // sets the value of float_data to 84.0
        my_value.with_mut(|fields| {
            **fields.float_reference = (**fields.int_reference as f32) * 2.0;
        });

        let int_ref = *my_value.borrow_int_reference();
        assert_eq!(42, (*int_ref).clone());
        drop(my_value);
    }
}
