pub fn is_validated_method(method: &str) -> bool {
    let methods = vec![
        "GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "TRACE", "CONNECT",
    ];
    methods.contains(&method)
}
