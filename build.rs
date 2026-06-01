fn main() {
  let devkitpro = std::env::var("DEVKITPRO").unwrap_or_else(|_| "/opt/devkitpro".to_string());

  // Pass the search path for libctru binary to cargo
  println!("cargo:rustc-link-search=native={}/libctru/lib", devkitpro);
  println!("cargo:rustc-link-lib=static=ctru");
}
