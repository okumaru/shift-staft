fn main() {
    println!("cargo:rustc-env=YEAR=2024");
    println!("cargo:rustc-env=MONTH=6");
    println!("cargo:rustc-env=SAT_DATE=25");
    println!("cargo:rustc-env=SUN_DATE=26");
    println!("cargo:rustc-env=PREV_YEAR=false");
    println!("cargo:rustc-env=PREV_MONTH=true");
}
