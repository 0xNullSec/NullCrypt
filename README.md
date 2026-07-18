<div align="center">

<img src="https://capsule-render.vercel.app/content?type=waving&color=00ffcc&height=220&section=header&text=NullCrypt%20v2.0&fontSize=65&fontColor=111111&animation=twinkling" width="100%" />

### NullCrypt

[![Language](https://img.shields.io/badge/Language-Rust-ea4aaa?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows-0078d7?style=for-the-badge&logo=windows&logoColor=white)](#)
[![Security Focus](https://img.shields.io/badge/Focus-Malware%20Dev%20%2F%20Evasion-red?style=for-the-badge&logo=hack-the-box&logoColor=white)](#)
[![Release](https://img.shields.io/badge/Release-v2.0.0-00ffcc?style=for-the-badge)](https://github.com/0xNullSec/NullCrypt/releases/tag/v2.0.0)

<p align="center">
  <b>Select Language / Seleccione Idioma:</b><br>
  <a href="#english">English</a> • <a href="#español">Español</a>
</p>

---
</div>

<div id="english"></div>

# English

<details open>
<summary>Click to expand/collapse English version</summary>

## About the Project

**NullCrypt** is a crypter developed in **Rust** to understand the evasion techniques used to bypass AVs/EDRs. 

This project serves as an educational resource and proof-of-concept for security researchers and red teamers interested in defensive bypasses.

---

## Why a Crypter?

When I first got into malware development, I became interested in how attackers evade security systems, so I decided to undertake this project early on; naturally, given the nature of malware development, the project seemed both daunting and ambitious.

I began researching evasion techniques, and I believe I now have a solid project—even if I know it has some shortcomings. I decided to publish it because others interested in the field might find a project like this useful, serving as an aid for both researching the techniques and building a similar tool.

---

## Features & Evasion Techniques

NullCrypt implements the following bypass techniques to defeat static and dynamic analysis:

### Evasion & Unhooking
*   **TartarusGate & Indirect Syscalls:** Bypasses EDR user-mode hooks by executing system calls indirectly.
*   **Module Stomping & Trampoline:** Conceals shellcode execution by loading it into the memory space of a trusted DLL.
*   **API Hashing:** Obfuscates Windows API calls to prevent detection via suspicious function imports (IAT obfuscation).

### Anti-Machine Learning (ML) & Heuristics
*   **Junk Code:** Injects non-functional code sequences to alter the binary's structure and signature.
*   **Predicate Opaques:** Uses complex logical structures that resolve statically, breaking heuristic engine pattern-matching.

### Payload Management
*   **ChaCha20Poly1305 Cipher:** Encrypts the shellcode before execution.
*   **Load Shellcode in .rsrc:** Stores the encrypted payload inside the binary resources section rather than the text section.

---

## Usage

> **No Rust Required:** You don't need to have Rust installed; the target binary will be embedded directly into the stub.

1. **Download:** Get the latest .zip file from the [Releases v2.0.0](https://github.com/0xNullSec/NullCrypt/releases/tag/v2.0.0) page.
2. **Exclusion:** Create an antivirus exclusion for the folder containing the releases to prevent signature flags during testing.
3. **Build your Stub:** Run the builder and input the payload.

---

## Screenshots

### Builder Interface
<div align="center">
  <img src="path/to/builder-screenshot.png" alt="NullCrypt Builder Interface" width="80%" />
  <p><i>The compiler interface embedding the payload into the stub.</i></p>
</div>

### Evasion & Execution Demo
<div align="center">
  <img src="path/to/execution-screenshot.png" alt="NullCrypt Evasion Demo" width="80%" />
  <p><i>Execution of the payload bypassing local detection controls.</i></p>
</div>

---

## Documentation & Technical Analysis

### Complete Write-up & Blog Post — Coming Soon

I am developing a personal blog where I will publish the technical documentation regarding NullCrypt.

The upcoming post will cover:
* Breakdown of the Rust implementation.
* Mechanics of TartarusGate and Indirect Syscalls under the hood.
* Analysis on how these techniques circumvent EDR behavior monitoring.

*The link will be attached here as soon as it goes live.*

---

## Disclaimer

Using this software to attack targets without prior written consent is illegal. It is the end user's responsibility to comply with all applicable local, state, and federal laws. The author assumes no liability and is not responsible for any misuse or damage caused by this program.

</details>

---
---

<div id="español"></div>

# Español

<details>
<summary>Clic para expandir/colapsar la versión en Español</summary>

## Acerca del Proyecto

**NullCrypt** es un crypter desarrollado en **Rust** con el objetivo de comprender las técnicas utilizadas para evadir diversos AVs/EDRs.

Este proyecto actúa como un recurso educativo y una prueba de concepto para investigadores de seguridad y red teamers interesados en el funcionamiento de los bypasses defensivos.

---

## ¿Por qué un Crypter?

Cuando me adentré por primera vez en el desarrollo de malware, me interesé en cómo los atacantes evaden los sistemas de seguridad, por lo que decidí emprender este proyecto desde el principio; naturalmente, dada la naturaleza del desarrollo de malware, el proyecto parecía tanto desalentador como ambicioso.

Comencé a investigar técnicas de evasión y creo que ahora tengo un proyecto sólido, incluso si sé que tiene algunas deficiencias. Decidí publicarlo porque a otros interesados en el campo les podría resultar útil un proyecto como este, sirviendo de ayuda tanto para investigar las técnicas como para construir una herramienta similar.

---

## Características y Técnicas de Evasión

NullCrypt integra las siguientes técnicas para evadir el análisis estático y dinámico:

### Evasión y Desenganche (Unhooking)
*   **TartarusGate e Indirect Syscalls:** Evade los hooks en modo usuario de los EDRs ejecutando llamadas al sistema de manera indirecta.
*   **Module Stomping y Trampoline:** Oculta la ejecución del shellcode cargándolo en el espacio de memoria de una DLL legítima.
*   **API Hashing:** Ofusca las llamadas a la API de Windows para evitar la detección de importaciones de funciones sospechosas (ofuscación de la IAT).

### Anti-Machine Learning (ML) y Heurística
*   **Junk Code:** Inyecta secuencias de código no funcionales para alterar la estructura y la firma del binario.
*   **Predicate Opaques:** Utiliza estructuras lógicas complejas que se resuelven de forma estática, rompiendo el emparejamiento de patrones de los motores heurísticos.

### Gestión de Payloads
*   **Cifrado ChaCha20Poly1305:** Cifra el shellcode antes de la ejecución.
*   **Carga de Shellcode en .rsrc:** Almacena el payload cifrado dentro de la sección de recursos del binario en lugar de la sección de texto.

---

## Uso

> **No se requiere Rust:** No necesitas tener Rust instalado; el binario elegido se incrustará directamente en el stub.

1. **Descarga:** Descarga el archivo .zip desde la página de [Releases v2.0.0](https://github.com/0xNullSec/NullCrypt/releases/tag/v2.0.0).
2. **Exclusión:** Crea una exclusión de antivirus para la carpeta que contiene las versiones lanzadas para evitar alertas de firmas durante las pruebas.
3. **Construye tu Stub:** Abre el builder e ingresa el payload que deseas utilizar.

---

## Capturas de Pantalla

### Interfaz del Builder
<div align="center">
  <img src="path/to/builder-screenshot.png" alt="Interfaz del Builder de NullCrypt" width="80%" />
  <p><i>Interfaz del compilador incrustando el payload directamente en el stub.</i></p>
</div>

### Demostración de Evasión y Ejecución
<div align="center">
  <img src="path/to/execution-screenshot.png" alt="Demostración de Evasión de NullCrypt" width="80%" />
  <p><i>Ejecución del payload evadiendo los controles de detección locales.</i></p>
</div>

---

## Documentación y Análisis Técnico

### Análisis Completo y Artículo de Blog — Próximamente

Estoy desarrollando un blog personal donde publicaré la documentación técnica sobre NullCrypt.

La próxima publicación cubrirá:
* Desglose de la implementación en Rust.
* Funcionamiento interno de TartarusGate e Indirect Syscalls.
* Análisis sobre cómo estas técnicas eluden el monitoreo de comportamiento de los EDRs.

*El enlace se adjuntará aquí tan pronto como esté disponible.*

---

## Descargo de Responsabilidad

El uso de este software para atacar objetivos sin el consentimiento previo por escrito es ilegal. Es responsabilidad del usuario final cumplir con todas las leyes locales, estatales y federales aplicables. El autor no asume ninguna responsabilidad y no es responsable de ningún mal uso o daño causado por este programa.

</details>

---

<div align="center">
  <p>Maintained by <a href="https://github.com/0xNullSec">0xNullSec</a></p>
  <sub>"Nullis in verba"</sub>
</div>
