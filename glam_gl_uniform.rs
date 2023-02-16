// From https://stackoverflow.com/a/67881267/16854783
// Depends on: gl, glam

fn main() {
  // Where `transform` is a Matrix, and `transform_loc` is a uniform location
  gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, &transform.to_cols_array()[0]);
}
