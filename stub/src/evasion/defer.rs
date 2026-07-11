use crate::tartarusgate;

pub struct DeferExitThread;

impl Drop for DeferExitThread {
    fn drop(&mut self) {
        unsafe {
            println!("\n[DEBUG] [DEFER] Ejecutando NtTerminateThread antes de salir...");

            let current_thread = -2isize as usize;
            let exit_status = 0usize;

            let result = (|| -> Result<(), Box<dyn std::error::Error>> {
                let _ = tartarusgate!("NtTerminateThread", current_thread, exit_status);
                Ok(())
            })();

            if let Err(e) = result {
                println!("[DEBUG] Error silencioso en el Defer: {}", e);
            }
        }
    }
}
