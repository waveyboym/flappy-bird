//TIMING & VELOCITY stuff
pub const PLAYER_MOVEMENT_SPEED: i32 = 5;
pub const BACKGROUND_WALLPAPER_MOVEMENT_SPEED: i32 = 1;
pub const PERIODSLIMIT: u8 = 5;

//SIZES OF STUFF ON SCREEN
pub const SCREEN_HEIGHT: u32 = 600;
pub const SCREEN_WIDTH: u32 = 800;
pub const PLAYER_SPRITE_SIZE_WL: u32 = 16;
pub const PIPE_WIDTH: u32 = 132;
pub const PIPE_HEIGHT: u32 = SCREEN_HEIGHT;
pub const LANDSCAPE_WIDTH: u32 = 132;

//COLOUR (for background behind wallpaper)
pub const REDVAL: u8 = 255;
pub const GREENVAL: u8 = 255;
pub const BLUEVAL: u8 = 255;

//POSITIONS, CUTOFF POINTS AND SPACING
pub const EDGEOFSCREEN_YAXIS: i32 = (LANDSCAPE_WIDTH as i32 / 4) + 8;
pub const LOWER_NEG_SCREEN_WIDTH: i32 = -800;
pub const TOP_SUBBASE_VAL: u32 = 50;
pub const BTM_ADDBASE_VAL: u32 = 400;
pub const BTM_LANDSCAPE_POS: i32 = (SCREEN_HEIGHT as i32 / 2) - ((LANDSCAPE_WIDTH as i32 / 4) / 2);
//pub const SPACE_BTWN_PIPES: u32 = 450;
pub const X_SEPARATION: u32 = 175;