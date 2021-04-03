use std::env::{current_dir, join_paths, set_var, split_paths, var_os, vars_os};
use std::path::PathBuf;
use std::process::Command;

fn main() {
  println!("Installing...");
  let dir = current_dir().unwrap();
  // let path = format!("set PATH=%PATH%;{}", dir.to_str().unwrap());
  println!("The current directory is {}", dir.to_str().unwrap());
  // let vars = vars_os();

  // if let Some(path) = var_os("PATH") {
  //   let mut paths = split_paths(&path).collect::<Vec<_>>();
  //   paths.push(PathBuf::from(dir));
  //   let new_path = join_paths(paths);
  //   set_var("PATH", &new_path.unwrap());
  // }

  for (key, value) in vars_os() {
    println!("{}: {}", key.to_str().unwrap(), value.to_str().unwrap())
  }

  // let output = Command::new("cmd")
  //   .args(&["/C", &path])
  //   .output()
  //   .expect("failed to execute");

  // println!("{} added to PATH", dir);
}
