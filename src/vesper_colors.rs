//! 🌌 Vesper OS Color Palette
//!
//! Este módulo define la paleta de colores oficial de Vesper OS,
//! inspirada en la elegancia nocturna y la sabiduría del búho Nox.

/// Paleta de colores de Vesper OS
pub mod colors {
    /// Negro Profundo - El color base de Vesper OS
    pub const DEEP_BLACK: u32 = 0x0B0B0D;

    /// Morado Oscuro - Color secundario elegante
    pub const DARK_PURPLE: u32 = 0x2C1A47;

    /// Azul Cósmico - Color de acento principal
    pub const COSMIC_BLUE: u32 = 0x1E2A78;

    /// Violeta Brillante - Color de destacado
    pub const BRIGHT_VIOLET: u32 = 0xA259FF;

    /// Blanco Humo - Color de texto principal
    pub const SMOKE_WHITE: u32 = 0xEAEAEA;

    /// Verde Neón - Color de éxito y confirmación
    pub const NEON_GREEN: u32 = 0x3DFFB4;
}

/// Constantes de color para VGA
pub mod vga_colors {
    use crate::vesper_colors::colors::*;

    /// Color de fondo principal (Negro Profundo)
    pub const BACKGROUND: u8 = 0x00; // Negro

    /// Color de texto principal (Blanco Humo)
    pub const TEXT_PRIMARY: u8 = 0x07; // Gris claro

    /// Color de acento (Violeta Brillante)
    pub const ACCENT: u8 = 0x05; // Magenta

    /// Color de éxito (Verde Neón)
    pub const SUCCESS: u8 = 0x0A; // Verde claro

    /// Color de advertencia (Azul Cósmico)
    pub const WARNING: u8 = 0x01; // Azul

    /// Color de error (Morado Oscuro)
    pub const ERROR: u8 = 0x04; // Rojo
}

/// Temas de color para diferentes contextos
pub mod themes {
    use crate::vesper_colors::vga_colors::*;

    /// Tema nocturno - El tema principal de Vesper OS
    pub const NIGHT_THEME: (u8, u8) = (BACKGROUND, TEXT_PRIMARY);

    /// Tema de acento - Para elementos destacados
    pub const ACCENT_THEME: (u8, u8) = (ACCENT, TEXT_PRIMARY);

    /// Tema de éxito - Para mensajes positivos
    pub const SUCCESS_THEME: (u8, u8) = (BACKGROUND, SUCCESS);

    /// Tema de advertencia - Para alertas
    pub const WARNING_THEME: (u8, u8) = (BACKGROUND, WARNING);

    /// Tema de error - Para errores críticos
    pub const ERROR_THEME: (u8, u8) = (BACKGROUND, ERROR);
}

/// Funciones de utilidad para colores
pub mod utils {
    use crate::vesper_colors::colors::*;

    /// Convierte un color RGB a formato VGA
    pub fn rgb_to_vga(rgb: u32) -> u8 {
        let r = ((rgb >> 16) & 0xFF) as u8;
        let g = ((rgb >> 8) & 0xFF) as u8;
        let b = (rgb & 0xFF) as u8;

        // Conversión aproximada a colores VGA de 16 colores
        if r > 128 && g > 128 && b > 128 {
            0x07 // Gris claro
        } else if r > 128 && g < 128 && b > 128 {
            0x05 // Magenta
        } else if r < 128 && g > 128 && b < 128 {
            0x0A // Verde claro
        } else if r < 128 && g < 128 && b > 128 {
            0x01 // Azul
        } else if r > 128 && g < 128 && b < 128 {
            0x04 // Rojo
        } else {
            0x00 // Negro
        }
    }

    /// Obtiene el color de fondo de Vesper OS
    pub fn get_background_color() -> u32 {
        DEEP_BLACK
    }

    /// Obtiene el color de texto principal de Vesper OS
    pub fn get_text_color() -> u32 {
        SMOKE_WHITE
    }

    /// Obtiene el color de acento de Vesper OS
    pub fn get_accent_color() -> u32 {
        BRIGHT_VIOLET
    }
}
