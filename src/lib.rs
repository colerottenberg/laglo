use databento::dbn::{self};
use polars::prelude::*;
use std::path::Path;

pub fn add(p: &str) {
    let path = Path::new(p);
    let mut data: DataFrame;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
