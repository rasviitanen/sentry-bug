#[cfg(test)]
mod tests {
    #[test]
    fn test_a() {
        let _guard = sentry::init(sentry::ClientOptions {
            ..Default::default()
        });
    }
    
    #[test]
    fn test_b() {
        let _guard = sentry::init(sentry::ClientOptions {
            ..Default::default()
        });
    }
}
