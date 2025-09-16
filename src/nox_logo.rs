//! 🦉 Nox Logo - El búho minimalista de Vesper OS
//!
//! Este módulo contiene diferentes variaciones del logo ASCII de Nox,
//! el búho minimalista que representa la sabiduría nocturna de Vesper OS.

/// Logo principal de Nox para el banner de inicio
pub const NOX_MAIN_LOGO: &str = r#"
    🌌 VESPER OS v1.0.0 🌌
    
    🦉 Nox, el búho minimalista 🦉
    
    ╔══════════════════════════════════════╗
    ║                                      ║
    ║        🦉     🦉     🦉     🦉        ║
    ║       /|\\   /|\\   /|\\   /|\\       ║
    ║      /_|_\\ /_|_\\ /_|_\\ /_|_\\      ║
    ║     |_|_|_|_|_|_|_|_|_|_|_|_|_|_|     ║
    ║                                      ║
    ║     🦉 Sabiduría Nocturna 🦉         ║
    ║     🌟 Elegancia Vespertina 🌟       ║
    ║     ⚡ WebAssembly Ready ⚡           ║
    ║                                      ║
    ║  Paleta: #0B0B0D #2C1A47 #1E2A78    ║
    ║         #A259FF #EAEAEA #3DFFB4      ║
    ║                                      ║
    ╚══════════════════════════════════════╝
"#;

/// Logo minimalista de Nox para mensajes cortos
pub const NOX_MINIMAL_LOGO: &str = r#"
    🦉 Nox 🦉
    ╔══════════╗
    ║  🦉 🦉 🦉  ║
    ║ /|\\ /|\\ ║
    ║/_|_\\/_|_\\║
    ╚══════════╝
"#;

/// Logo de Nox para errores
pub const NOX_ERROR_LOGO: &str = r#"
    🦉 Nox Error 🦉
    ╔═══════════════╗
    ║  🦉     🦉     ║
    ║ /|\\   /|\\   ║
    ║/_|_\\ /_|_\\  ║
    ║               ║
    ║  ⚠️  Error!   ║
    ╚═══════════════╝
"#;

/// Logo de Nox para éxito
pub const NOX_SUCCESS_LOGO: &str = r#"
    🦉 Nox Success 🦉
    ╔════════════════╗
    ║  🦉     🦉     ║
    ║ /|\\   /|\\   ║
    ║/_|_\\ /_|_\\  ║
    ║                ║
    ║  ✅ Success!   ║
    ╚════════════════╝
"#;

/// Logo de Nox para advertencias
pub const NOX_WARNING_LOGO: &str = r#"
    🦉 Nox Warning 🦉
    ╔═════════════════╗
    ║  🦉     🦉     ║
    ║ /|\\   /|\\   ║
    ║/_|_\\ /_|_\\  ║
    ║                 ║
    ║  ⚠️  Warning!  ║
    ╚═════════════════╝
"#;

/// Logo de Nox para información
pub const NOX_INFO_LOGO: &str = r#"
    🦉 Nox Info 🦉
    ╔══════════════╗
    ║  🦉     🦉    ║
    ║ /|\\   /|\\  ║
    ║/_|_\\ /_|_\\ ║
    ║              ║
    ║  ℹ️  Info    ║
    ╚══════════════╝
"#;

/// Función para obtener el logo apropiado según el contexto
pub fn get_nox_logo(context: NoxContext) -> &'static str {
    match context {
        NoxContext::Main => NOX_MAIN_LOGO,
        NoxContext::Minimal => NOX_MINIMAL_LOGO,
        NoxContext::Error => NOX_ERROR_LOGO,
        NoxContext::Success => NOX_SUCCESS_LOGO,
        NoxContext::Warning => NOX_WARNING_LOGO,
        NoxContext::Info => NOX_INFO_LOGO,
    }
}

/// Contextos para diferentes logos de Nox
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoxContext {
    /// Logo principal para el banner de inicio
    Main,
    /// Logo minimalista para mensajes cortos
    Minimal,
    /// Logo para errores
    Error,
    /// Logo para mensajes de éxito
    Success,
    /// Logo para advertencias
    Warning,
    /// Logo para información
    Info,
}

/// Frases inspiradoras de Nox
pub const NOX_QUOTES: &[&str] = &[
    "🦉 En la oscuridad de la complejidad, brilla la luz de la simplicidad elegante.",
    "🦉 Cada línea de código es una estrella en el cielo del desarrollo.",
    "🦉 La sabiduría nocturna guía cada decisión técnica.",
    "🦉 La elegancia vespertina se refleja en cada algoritmo.",
    "🦉 Nox vigila, Vesper brilla, el código fluye.",
    "🦉 En el silencio de la noche, el código habla con claridad.",
    "🦉 La portabilidad es la llave de la libertad digital.",
    "🦉 WebAssembly: donde la web se encuentra con el sistema.",
];

/// Obtiene una frase inspiradora aleatoria de Nox
pub fn get_nox_quote() -> &'static str {
    // En un sistema real, esto usaría un generador de números aleatorios
    // Por ahora, usamos una selección simple basada en el tiempo
    let index = 0; // Placeholder para selección aleatoria
    NOX_QUOTES[index % NOX_QUOTES.len()]
}
