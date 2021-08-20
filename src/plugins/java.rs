pub mod lombok;
pub mod webflux;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_java_lombok_dto_source_gen() {
        lombok::create_code();
    }

    #[test]
    fn test_java_controller_source_gen() {
        webflux::create_code();
    }
}
