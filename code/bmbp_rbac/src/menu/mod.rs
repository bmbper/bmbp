pub use route::*;
mod dao;
mod route;
mod service;
mod util;
mod vopo;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
