# 🌌 Vesper OS v1.0.0

## 🦉 Nox, el búho minimalista de Vesper OS

Vesper OS es un sistema operativo experimental escrito en Rust, diseñado para ser portable, ligero y persistente, con la capacidad de ejecutar aplicaciones en formato WebAssembly (.wasm).

## 🚀 Concepto Principal

- 🦉 Crear un sistema operativo personalizado en Rust
- 🦉 Soporte para aplicaciones WebAssembly ejecutadas como programas nativos
- 🦉 Portabilidad en USB con persistencia de datos (inspirado en NomadBSD, pero más ligero)
- 🦉 Identidad propia centrada en portabilidad y aprendizaje

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

## ✅ Progreso Actual

- ✅ Configuración mínima con Rust estable (sin dependencia de nightly)
- ✅ Multiboot2 válido para arranque con GRUB2
- ✅ Kernel funcional que muestra el banner de Nox en modo texto VGA
- ✅ Makefile para compilar el kernel, crear ISO con GRUB2 y ejecutar en QEMU
- ✅ Identidad visual completa con paleta de colores de Vesper OS
- ✅ Sistema de logging con emojis y branding de Vesper OS
- ✅ Módulo de colores personalizado para VGA y futuras interfaces

## 🎯 Próximos Pasos

- 🦉 Mejorar salida en pantalla (drivers VGA, framebuffer)
- 🦉 Implementar IDT y manejo de interrupciones
- 🦉 Agregar soporte para entrada de teclado
- 🦉 Diseñar el logo inicial en SVG
- 🦉 Probar persistencia en USB (arranque real)
- 🦉 Integrar soporte básico para .wasm
- 🦉 Sistema de empaquetado y gestión de aplicaciones WebAssembly
- 🦉 Logo y branding en ASCII/CLI y gráficos

## ✍️ Meta Personal

Aprender Rust a profundidad, dominando tanto el desarrollo de bajo nivel (OSDev) como el de alto nivel (ejecución de WebAssembly), para crear un sistema operativo único con una identidad propia y sólida centrada en Vesper OS y su búho Nox.

## 🔧 Características Técnicas

### Kernel
- **Versión**: 1.0.0
- **Lenguaje**: Rust (edición 2024)
- **Arquitectura**: x86_64 (con soporte planificado para AArch64 y RISC-V)
- **Tipo**: Microkernel
- **Bootloader**: Multiboot2 compatible

### Identidad Visual
- **ASCII Art**: Nox el búho con diseño minimalista
- **Colores**: Paleta nocturna con acentos violetas
- **Tipografía**: Moderna y geométrica
- **Símbolos**: Búho y estrella vespertina

### Características Planificadas
- **WebAssembly**: Ejecución nativa de aplicaciones .wasm
- **Portabilidad**: USB boot con persistencia
- **Ligereza**: Optimizado para sistemas de recursos limitados
- **Aprendizaje**: Documentación completa y ejemplos educativos

## 🌟 Filosofía

Vesper OS representa la elegancia nocturna de la programación de sistemas, donde cada línea de código es una estrella en el cielo del desarrollo. Nox, nuestro búho minimalista, vigila cada proceso, asegurando que la sabiduría y la eficiencia guíen cada decisión técnica.

*"En la oscuridad de la complejidad, brilla la luz de la simplicidad elegante."* - Filosofía de Vesper OS
