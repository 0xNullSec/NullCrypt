<div align="center">

<!-- Banner Dinámico Estilo Cyberpunk -->
<img src="https://capsule-render.vercel.app/content?type=waving&color=00ffcc&height=220&section=header&text=NullCrypt%20v2.0&fontSize=65&fontColor=111111&animation=twinkling" width="100%" />

###  NullCrypt

[![Language](https://img.shields.io/badge/Language-Rust-ea4aaa?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows-0078d7?style=for-the-badge&logo=windows&logoColor=white)](#)
[![Security Focus](https://img.shields.io/badge/Focus-Malware%20Dev%20%2F%20Evasion-red?style=for-the-badge&logo=hack-the-box&logoColor=white)](#)
[![Release](https://img.shields.io/badge/Release-v2.0.0-00ffcc?style=for-the-badge)](https://github.com/0xNullSec/NullCrypt/releases/tag/v2.0.0)

<p align="center">
  <a href="#-about-the-project">About</a> •
  <a href="#-why-a-crypter">Why NullCrypt?</a> •
  <a href="#-features--evasion-techniques">Evasion Features</a> •
  <a href="#-usage">Usage</a> •
  <a href="#-disclaimer">Disclaimer</a>
</p>

---
</div>

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

###  Evasion & Unhooking
*   **TartarusGate & Indirect Syscalls:** Bypasses EDR user-mode hooks by executing system calls indirectly, hiding the execution flow.
*   **Module Stomping & Trampoline:** Conceals shellcode execution by loading it into the memory space of a legitimate, trusted DLL.
*   **API Hashing:** Obfuscates Windows API calls to prevent static analysis from detecting suspicious function imports (IAT obfuscation).

###  Anti-Machine Learning (ML) & Heuristics
*   **Junk Code:** Injects non-functional code sequences to alter the binary's structure and signature.
*   **Predicate Opaques:** Uses complex logical structures that look dynamic but resolve statically, breaking heuristic engine pattern-matching.

### Payload Management
*   **ChaCha20Poly1305 Cipher:** Encrypts the shellcode with an authenticated encryption algorithm to ensure high-grade data protection before execution.
*   **Load Shellcode in `.rsrc`:** Stores the encrypted payload cleanly inside the binary resources section rather than the text section.

---

##  Usage

>  **No Rust Required:** You don't need to have Rust installed on your machine; the binary you choose will be embedded directly into the stub.

1. **Download:** Grab the latest `.zip` file from the [Releases v2.0.0](https://github.com/0xNullSec/NullCrypt/releases/tag/v2.0.0) page.
2. **Exclusion (Optional but recommended):** It is best to create an antivirus exclusion for the folder containing the releases to prevent premature signature flags during testing.
3. **Build your Stub:** Open the builder within the folder and input the payload you wish to use.

---

## 🛑 Disclaimer

Using this software to attack targets without prior written consent is illegal. It is the end user's responsibility to comply with all applicable local, state, and federal laws. The author assumes no liability and is not responsible for any misuse or damage caused by this program.

---

<div align="center">
  <p>Maintained by <a href="https://github.com/0xNullSec">0xNullSec</a></p>
  <sub>"Nullis in verba"</sub>
</div>
