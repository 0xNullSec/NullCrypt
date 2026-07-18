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

**NullCrypt** is a crypter developed in **Rust** with the aim of understanding certain techniques used by attackers to evade various AVs/EDRs. 

By implementing modern evasion mechanics at a native level, this project acts as an educational resource and a proof-of-concept for security researchers, red teamers, and anyone interested in the inner workings of defensive bypasses.

---

## Why a Crypter?

When I first got into malware development, I became interested in how attackers evade security systems, so I decided to undertake this project early on; naturally, given the nature of malware development, the project seemed both daunting and ambitious.

I began researching evasion techniques, and I believe I now have a solid project—even if I know it has some shortcomings. I decided to publish it because others interested in the field might find a project like this useful, serving as an aid for both researching the techniques and building a similar tool.

---

## Features & Evasion Techniques

NullCrypt integrates several industry-standard and advanced bypass techniques to defeat both static and dynamic analysis:

### Evasion & Unhooking
*   **TartarusGate & Indirect Syscalls:** Bypasses EDR user-mode hooks by executing system calls indirectly, hiding the execution flow.
*   **Module Stomping & Trampoline:** Conceals shellcode execution by loading it into the memory space of a legitimate, trusted DLL.
*   **API Hashing:** Obfuscates Windows API calls to prevent static analysis from detecting suspicious function imports (IAT obfuscation).

### Anti-Machine Learning (ML) & Heuristics
*   **Junk Code:** Injects non-functional code sequences to alter the binary's structure and signature.
*   **Predicate Opaques:** Uses complex logical structures that look dynamic but resolve statically, breaking heuristic engine pattern-matching.

### Payload Management
*   **ChaCha20Poly1305 Cipher:** Encrypts the shellcode with an authenticated encryption algorithm to ensure high-grade data protection before execution.
*   **Load Shellcode in .rsrc:** Stores the encrypted payload cleanly inside the binary resources section rather than the text section.

---

## Usage

> **No Rust Required:** You don't need to have Rust installed on your machine; the binary you choose will be embedded directly into the stub.

1. **Download:** Grab the latest .zip file from the [Releases v2.0.0](https://github.com/0xNullSec/NullCrypt/releases/tag/v2.0.0) page.
2. **Exclusion (Optional but recommended):** It is best to create an antivirus exclusion for the folder containing the releases to prevent premature signature flags during testing.
3. **Build your Stub:** Open the builder within the folder and input the payload you wish to use.

---

## Documentation & Technical Analysis

### Complete Write-up & Blog Post — Coming Soon

I am currently developing a personal blog/website where I will publish a comprehensive technical documentation and a deep-dive write-up about NullCrypt. 

The upcoming post will cover:
* A step-by-step breakdown of the Rust implementation.
* Deeper technical insights into the mechanics of TartarusGate and Indirect Syscalls under the hood.
* Advanced analysis on how these techniques circumvent modern EDR behavior monitoring.

*The link to the blog post will be attached here as soon as it goes live!*

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

**NullCrypt** es un crypter desarrollado en **Rust** con el objetivo de comprender ciertas técnicas utilizadas por los atacantes para evadir diversos AVs/EDRs.

Al implementar mecánicas modernas de evasión a nivel nativo, este proyecto actúa como un recurso educativo y una prueba de concepto para investigadores de seguridad, red teamers y cualquier persona interesada en el funcionamiento interno de los bypasses defensivos.

---

## ¿Por qué un Crypter?

Cuando me adentré por primera vez en el desarrollo de malware, me interesé en cómo los atacantes evaden los sistemas de seguridad, por lo que decidí emprender este proyecto desde el principio; naturalmente, dada la naturaleza del desarrollo de malware, el proyecto parecía tanto desalentador como ambicioso.

Comencé a investigar técnicas de evasión y creo que ahora tengo un proyecto sólido, incluso si sé que tiene algunas deficiencias. Decidí publicarlo porque a otros interesados en el campo les podría resultar útil un proyecto como este, sirviendo de ayuda tanto para investigar las técnicas como para construir una herramienta similar.

---

## Características y Técnicas de Evasión

NullCrypt integra varias técnicas de bypass avanzadas y estándar de la industria para derrotar tanto el análisis estático como el dinámico:

### Evasión y Desenganche (Unhooking)
*   **TartarusGate e Indirect Syscalls:** Evade los hooks en modo usuario de los EDRs ejecutando llamadas al sistema de manera indirecta, ocultando el flujo de ejecución.
*   **Module Stomping y Trampoline:** Oculta la ejecución del shellcode cargándolo en el espacio de memoria de una DLL legítima y confiable.
*   **API Hashing:** Ofusca las llamadas a la API de Windows para evitar que el análisis estático detecte importaciones de funciones sospechosas (ofuscación de la IAT).

### Anti-Machine Learning (ML) y Heurística
*   **Junk Code (Código Basura):** Inyecta secuencias de código no funcionales para alterar la estructura y la firma del binario.
*   **Predicate Opaques (Predicados Opacos):** Utiliza estructuras lógicas complejas que parecen dinámicas pero se resuelven de forma estática, rompiendo el emparejamiento de patrones de los motores heurísticos.

### Gestión de Payloads
*   **Cifrado ChaCha20Poly1305:** Cifra el shellcode con un algoritmo de cifrado autenticado para garantizar una protección de datos de alto nivel antes de la ejecución.
*   **Carga de Shellcode en .rsrc:** Almacena el payload cifrado de forma limpia dentro de la sección de recursos del binario en lugar de la sección de texto.

---

## Uso

> **No se requiere Rust:** No necesitas tener Rust instalado en tu máquina; el binario que elijas se incrustará directamente en el stub.

1. **Descarga:** Descarga el archivo .zip más reciente desde la página de [Releases v2.0.0](https://github.com/0xNullSec/NullCrypt/releases/tag/v2.0.0).
2. **Exclusión (Opcional pero recomendado):** Es mejor crear una exclusión de antivirus para la carpeta que contiene las versiones lanzadas para evitar alertas prematuras de firmas durante las pruebas.
3. **Construye tu Stub:** Abre el builder dentro de la carpeta e ingresa el payload que deseas utilizar.

---

## Documentación y Análisis Técnico

### Análisis Completo y Artículo de Blog — Próximamente

Actualmente estoy desarrollando un sitio web/blog personal donde publicaré una documentación técnica completa y un análisis profundo sobre NullCrypt.

La próxima publicación cubrirá:
* Un desglose paso a paso de la implementación en Rust.
* Perspectivas técnicas más profundas sobre el funcionamiento interno de TartarusGate e Indirect Syscalls.
* Análisis avanzado sobre cómo estas técnicas eluden el monitoreo de comportamiento de los EDRs modernos.

*¡El enlace al artículo del blog se adjuntará aquí tan pronto como esté disponible!*

---

## Descargo de Responsabilidad

El uso de este software para atacar objetivos sin el consentimiento previo por escrito es ilegal. Es responsabilidad del usuario final cumplir con todas las leyes locales, estatales y federales aplicables. El autor no asume ninguna responsabilidad y no es responsable de ningún mal uso o daño causado por este programa.

</details>

---

<div align="center">
  <p>Maintained by <a href="https://github.com/0xNullSec">0xNullSec</a></p>
  <sub>"Nullis in verba"</sub>
</div>
