// mod.
// pub mod browser;
pub mod configure;
pub mod events;


/// # array.protype.set simulation.
///     can be written beyond the boundary.
/// 
/// ## example
/// ```
/// let mut a = vec![0, 1, 2];
/// let b = vec![3, 4, 5];
/// set_vec(&mut a, &b, 4);
/// ```
pub fn set_vec<T> (data: &mut Vec<T>, insert: &Vec<T>, offset: usize) 
    where T: std::clone::Clone  {
    let mut offset_copy = offset;

    // check boundary.
    if data.len() - 1 < offset {
        let offset_length = offset - (data.len() - 1);
        data.reserve(offset_length + insert.len());
    }

    // insert data.
    for value in insert.iter() {
        data.insert(offset_copy, value.clone());
        offset_copy += 1;
    }
}