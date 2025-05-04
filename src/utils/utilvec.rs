pub fn remove_from_index<T>(xs: &mut Vec<T>, some_x: T) where T: PartialEq {
  let index = xs.iter().position(|x| *x == some_x).unwrap();
  xs.remove(index);
}