# 🌌 Vesper OS Kernel v1.0.0

**Vesper OS** - Un sistema operativo experimental escrito en Rust, diseñado para ser portable, ligero y persistente, con la capacidad de ejecutar aplicaciones en formato WebAssembly (.wasm).

## 🦉 Nox, el búho minimalista de Vesper OS

Vesper OS está centrado en la portabilidad y el aprendizaje, con una identidad visual única que combina elegancia nocturna y sabiduría tecnológica.

## 🎨 Identidad Visual

- **Nombre**: Vesper (estrella vespertina, elegancia nocturna)
- **Mascota**: Nox, el búho minimalista 🦉
- **Paleta de Colores**:
  - Negro Profundo: `#0B0B0D`
  - Morado Oscuro: `#2C1A47`
  - Azul Cósmico: `#1E2A78`
  - Violeta Brillante: `#A259FF`
  - Blanco Humo: `#EAEAEA`
  - Verde Neón: `#3DFFB4`

## 🚀 Concepto Principal

- 🦉 Crear un sistema operativo personalizado en Rust
- 🦉 Soporte para aplicaciones WebAssembly ejecutadas como programas nativos
- 🦉 Portabilidad en USB con persistencia de datos (inspirado en NomadBSD, pero más ligero)
- 🦉 Identidad propia centrada en portabilidad y aprendizaje

[![docs](https://img.shields.io/badge/docs-master-blue.svg)](https://docs.rs/redox_syscall/latest/syscall/)
[![SLOCs counter](https://tokei.rs/b1/github/DevMaubry/vesper-os-kernel?category=code)](https://github.com/XAMPPRocky/tokei)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## Requirements

* [`nasm`](https://nasm.us/) needs to be available on the PATH at build time.

## Building The Documentation

Use this command:

```sh
cargo doc --open --target x86_64-unknown-none
```

## Debugging

### QEMU

Running [QEMU](https://www.qemu.org) with the `-s` flag will set up QEMU to listen on port `1234` for a GDB client to connect to it. To debug the VerperOS kernel run.

```sh
make qemu gdb=yes
```

This will start a virtual machine with and listen on port `1234` for a GDB or LLDB client.

### GDB

If you are going to use [GDB](https://www.gnu.org/software/gdb/), run these commands to load debug symbols and connect to your running kernel:

```
(gdb) symbol-file build/kernel.sym
(gdb) target remote localhost:1234
```

### LLDB

If you are going to use [LLDB](https://lldb.llvm.org/), run these commands to start debugging:

```
(lldb) target create -s build/kernel.sym build/kernel
(lldb) gdb-remote localhost:1234
```

After connecting to your kernel you can set some interesting breakpoints and `continue`
the process. See your debuggers man page for more information on useful commands to run.

## Notes

- Always use `foo.get(n)` instead of `foo[n]` and try to cover for the possibility of `Option::None`. Doing the regular way may work fine for applications, but never in the kernel. No possible panics should ever exist in kernel space, because then the whole OS would just stop working.

- If you receive a kernel panic in QEMU, use `pkill qemu-system` to kill the frozen QEMU process.

## How To Contribute

To learn how to contribute to this system component you need to read the following document:

- [CONTRIBUTING.md](https://github.com/DevMaubry/verperos-kernel/blob/main/CONTRIBUTING.md)

## Development

## 🔧 Desarrollo

Para aprender cómo desarrollar con este componente del sistema dentro del ecosistema de Vesper OS, necesitas leer la documentación de [Build System](https://doc.vesperos.org/book/build-system-reference.html) y [Coding and Building](https://doc.vesperos.org/book/coding-and-building.html).

### 🏗️ Cómo Compilar

Para compilar este componente del sistema necesitas descargar el sistema de build de Vesper OS. Puedes aprender cómo hacerlo en la página [Building Vesper OS](https://doc.vesperos.org/book/podman-build.html).

Esto es necesario porque solo funcionan con cross-compilación a una máquina virtual de Vesper OS, pero puedes hacer algunas pruebas desde Linux.

### 🦉 Progreso Actual

- ✅ Configuración mínima con Rust estable (sin dependencia de nightly)
- ✅ Multiboot2 válido para arranque con GRUB2
- ✅ Kernel funcional que muestra el banner de Nox en modo texto VGA
- ✅ Makefile para compilar el kernel, crear ISO con GRUB2 y ejecutar en QEMU
- ✅ Identidad visual completa con paleta de colores de Vesper OS

## Funding - _Unix-style Signals and Process Management_

This project is funded through [NGI Zero Core](https://nlnet.nl/core), a fund established by [NLnet](https://nlnet.nl) with financial support from the European Commission's [Next Generation Internet](https://ngi.eu) program. Learn more at the [NLnet project page](https://nlnet.nl/project/VesperOS-Signals).

## 🎯 Próximos Pasos

- 🦉 Mejorar salida en pantalla (drivers VGA, framebuffer)
- 🦉 Implementar IDT y manejo de interrupciones
- 🦉 Agregar soporte para entrada de teclado
- 🦉 Diseñar el logo inicial en SVG
- 🦉 Probar persistencia en USB (arranque real)
- 🦉 Integrar soporte básico para .wasm

## ✍️ Meta Personal

Aprender Rust a profundidad, dominando tanto el desarrollo de bajo nivel (OSDev) como el de alto nivel (ejecución de WebAssembly), para crear un sistema operativo único con una identidad propia y sólida centrada en Vesper OS y su búho Nox.

[<img src="https://nlnet.nl/logo/banner.png" alt="NLnet foundation logo" width="20%" />](https://nlnet.nl)
[<img src="https://nlnet.nl/image/logos/NGI0_tag.svg" alt="NGI Zero Logo" width="20%" />](https://nlnet.nl/core)
