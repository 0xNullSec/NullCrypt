// build.rs
use std::env;
use std::path::PathBuf;
use std::process::Command;
fn main() {
    let _target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Ruta del archivo .asm
    let asm_path = PathBuf::from("src/syscall/hellgates.asm");
    let obj_path = out_dir.join("hellgates.obj");

    // 1. Ensamblamos hellgates.asm → hellgates.obj con NASM (formato win64)
    let status = Command::new("nasm")
        .args(&[
            "-f", "win64",
            asm_path.to_str().unwrap(),
            "-o", obj_path.to_str().unwrap(),
        ])
        .status()
        .expect("nasm no está instalado → sudo apt install nasm");

    if !status.success() {
        panic!("Falló el ensamblado de hellgates.asm");
    }

    // 2. Convertimos el .obj en una librería estática .a que Rust entienda
    // ar es el que viene con mingw (binutils)
    let lib_path = out_dir.join("libhellgates.a");
    let ar_status = Command::new("x86_64-w64-mingw32-ar")
        .args(&["crs", lib_path.to_str().unwrap(), obj_path.to_str().unwrap()])
        .status()
        .expect("x86_64-w64-mingw32-ar no encontrado (instala mingw-w64)");

    if !ar_status.success() {
        panic!("Falló la creación de libhellgates.a");
    }

    // 3. Le decimos a Cargo dónde está la librería y que la enlace estáticamente
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=hellgates");

    // Para que se recompile si cambias el .asm
    println!("cargo:rerun-if-changed=src/syscall/hellgates.asm");
}

/*
fn main() {
    let target = env::var("TARGET").unwrap();

    // Solo Windows x64 MSVC
    if target == "x86_64-pc-windows-msvc" {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        let asm = "src/hellgates.asm";
        let obj = out_dir.join("hellgates.obj");

        // 1️⃣ Compilar ASM con NASM
        let status = Command::new("nasm")
            .args([
                "-f", "win64",
                asm,
                "-o",
                obj.to_str().unwrap(),
            ])
            .status()
            .expect("NASM no está instalado");

        if !status.success() {
            panic!("Falló el ensamblado NASM");
        }

        // 2️⃣ Enlazar el OBJ directamente (MSVC linker)
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-arg={}", obj.display());

        // Rebuild si cambia el ASM
        println!("cargo:rerun-if-changed={}", asm);
    }
}
*/
