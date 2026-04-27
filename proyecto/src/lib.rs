use anchor_lang::prelude::*;

declare_id!("HNAN9Sq9WUPAG3UUWZhUVWsaoqSjohjLh4dcFDC6WneK");

#[program]
mod modulo {
    use super::*;

    // ---  Inicializar la Base de Datos ---
    pub fn crear_database(ctx: Context<CrearVideojuegoDB>, nombre_db: String) -> Result<()> {
        let db = &mut ctx.accounts.videojuego_db;
        require!(nombre_db.len() <= 30, ErrorCode::NombreMuyLargo);

        db.nombre_db = nombre_db;
        db.juegos = Vec::new();
        Ok(())
    }

    // --- CREATE: Agregar un nuevo Videojuego ---
    pub fn agregar_videojuego(
        ctx: Context<CrearJuego>, 
        juego_nombre: String, 
        genero: String, 
        estudio: String, 
        dificultad: u8, 
        calificacion: u8
    ) -> Result<()> {
        let videojuego_db = &mut ctx.accounts.videojuego_db;
        let juego = &mut ctx.accounts.juego;

        require!(juego_nombre.len() <= 30, ErrorCode::NombreMuyLargo);
        
        juego.nombre = juego_nombre;
        juego.genero = genero;
        juego.estudio = estudio;
        juego.dificultad = dificultad;
        juego.calificacion = calificacion;
        
        videojuego_db.juegos.push(juego.key());
        Ok(())
    }

    // --- UPDATE: Actualizar datos de un juego existente ---
    pub fn actualizar_videojuego(
        ctx: Context<ActualizarJuego>,
        _juego_nombre: String, // Se usa para las seeds en el contexto
        nuevo_genero: Option<String>,
        nueva_calificacion: Option<u8>
    ) -> Result<()> {
        let juego = &mut ctx.accounts.juego;

        if let Some(g) = nuevo_genero {
            require!(g.len() <= 30, ErrorCode::NombreMuyLargo);
            juego.genero = g;
        }
        if let Some(c) = nueva_calificacion {
            juego.calificacion = c;
        }

        Ok(())
    }

    // --- DELETE: Eliminar un juego y cerrar la cuenta ---
    pub fn eliminar_videojuego(ctx: Context<EliminarJuego>, _juego_nombre: String) -> Result<()> {
        let videojuego_db = &mut ctx.accounts.videojuego_db;
        let juego_key = ctx.accounts.juego.key();

        // Eliminar la referencia de la llave del vector en la DB
        videojuego_db.juegos.retain(|&x| x != juego_key);
        
        // La cuenta 'juego' se cierra automáticamente por el atributo 'close' en el contexto
        Ok(())
    }
}

// --- ESTRUCTURAS DE DATOS ---

#[account]
#[derive(InitSpace)]
pub struct VideojuegoDB {
    #[max_len(30)]
    pub nombre_db: String,
    #[max_len(10)]
    pub juegos: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct Juego {
    #[max_len(30)]
    pub nombre: String,
    #[max_len(30)]
    pub genero: String,
    #[max_len(30)]
    pub estudio: String,
    pub dificultad: u8,
    pub calificacion: u8,
}

// --- CONTEXTOS (VALIDACIÓN DE CUENTAS) ---

#[derive(Accounts)]
#[instruction(nombre_db: String)]
pub struct CrearVideojuegoDB<'info> {
    #[account(
        init,
        payer = usuario, 
        space = 8 + VideojuegoDB::INIT_SPACE,
        seeds = [b"database", usuario.key().as_ref()],
        bump
    )]
    pub videojuego_db: Account<'info, VideojuegoDB>,
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(juego_nombre: String)]
pub struct CrearJuego<'info> {
    #[account(mut)]
    pub videojuego_db: Account<'info, VideojuegoDB>,

    #[account(
        init,
        payer = usuario,
        space = 8 + Juego::INIT_SPACE,
        seeds = [b"videojuego", usuario.key().as_ref(), juego_nombre.as_bytes()],
        bump
    )]
    pub juego: Account<'info, Juego>,
    
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(juego_nombre: String)]
pub struct ActualizarJuego<'info> {
    #[account(
        mut,
        seeds = [b"videojuego", usuario.key().as_ref(), juego_nombre.as_bytes()],
        bump
    )]
    pub juego: Account<'info, Juego>,
    pub usuario: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(juego_nombre: String)]
pub struct EliminarJuego<'info> {
    #[account(mut)]
    pub videojuego_db: Account<'info, VideojuegoDB>,

    #[account(
        mut,
        close = usuario, // Cierra la cuenta y devuelve los lamports al usuario
        seeds = [b"videojuego", usuario.key().as_ref(), juego_nombre.as_bytes()],
        bump
    )]
    pub juego: Account<'info, Juego>,
    
    #[account(mut)]
    pub usuario: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("El nombre es demasiado largo (máximo 30 caracteres).")]
    NombreMuyLargo,
}
