fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let a = [1, 2, 3, 4, 5];

        // let nice_slice: [i32; 3] = a[1..4].try_into().unwrap(); // Creating new array
        let nice_slice: &[i32] = &a[1..4]; // Borrowing the data without copying

        assert_eq!([2, 3, 4], nice_slice);
    }
}
