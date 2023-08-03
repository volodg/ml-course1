pub mod array;
mod canvas_ext;
pub mod classifiers;
pub mod data;
#[cfg(not(target_arch = "wasm32"))]
pub mod draw_images;
pub mod models;
pub mod sketch_pad;
pub mod ui;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
