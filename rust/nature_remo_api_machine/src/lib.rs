#[cfg(test)]
mod tests {
    use nature_remo_api_raw::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
