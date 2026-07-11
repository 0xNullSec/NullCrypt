using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Builder
{
    internal class Utils
    {

        public static bool IsValidPE(string filePath)
        {
            try
            {
                using (FileStream fs = new FileStream(filePath, FileMode.Open, FileAccess.Read))
                {
                    if (fs.Length < 64) return false; // Demasiado pequeño para ser un PE

                    // 1. Leer la firma DOS "MZ" (0x4D, 0x5A)
                    byte[] dosMagic = new byte[2];
                    fs.Read(dosMagic, 0, 2);
                    if (dosMagic[0] != 0x4D || dosMagic[1] != 0x5A) return false;

                    // 2. Leer la dirección e_lfanew (indica dónde empieza el encabezado PE)
                    fs.Position = 0x3C;
                    byte[] e_lfanew = new byte[4];
                    fs.Read(e_lfanew, 0, 4);
                    int peOffset = BitConverter.ToInt32(e_lfanew, 0);

                    // Validar que el offset tenga sentido dentro del tamaño del archivo
                    if (peOffset < 0 || peOffset + 4 > fs.Length) return false;

                    // 3. Leer la firma NT "PE\0\0" (0x50, 0x45, 0x00, 0x00)
                    fs.Position = peOffset;
                    byte[] peMagic = new byte[4];
                    fs.Read(peMagic, 0, 4);

                    return (peMagic[0] == 0x50 && peMagic[1] == 0x45 && peMagic[2] == 0x00 && peMagic[3] == 0x00);
                }
            }
            catch
            {
                // Si el archivo está bloqueado o hay error de lectura, asumimos falso
                return false;
            }
        }

        public static int GetPeArchitecture(string exePath)
        {
            try
            {
                using (var fs = new FileStream(exePath, FileMode.Open, FileAccess.Read))
                using (var br = new BinaryReader(fs))
                {
                    // 1. Leer offset del PE Header (e_lfanew está en 0x3C)
                    fs.Seek(0x3C, SeekOrigin.Begin);
                    int peOffset = br.ReadInt32();

                    // 2. Saltar al PE Header + 4 bytes (Signature es 4 bytes) -> Inicio de FileHeader
                    // El campo "Machine" son los primeros 2 bytes del FileHeader
                    fs.Seek(peOffset + 4, SeekOrigin.Begin);
                    ushort machine = br.ReadUInt16();

                    // 0x8664 = AMD64 (x64)
                    // 0x014C = I386 (x86)
                    if (machine == 0x8664) return 2; // x64
                    if (machine == 0x014c) return 1; // x86
                }
            }
            catch { }

            return 2; // Ante la duda, default a x64 (o lo que prefieras)
        }

    }
}
