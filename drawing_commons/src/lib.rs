pub mod array;
pub mod classifiers;
pub mod data;
pub mod models;
pub mod sketch_pad;
pub mod ui;
pub mod utils;
pub mod draw_images;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
