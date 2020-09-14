
pub trait Address {

    fn new(id: i32, index: usize) -> Self;
    fn get_id(&self) -> &i32;
    fn get_index(&self) -> &usize;

}