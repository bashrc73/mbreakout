use bevy::prelude::*;
use std::f32::consts::*;

pub const SAVE_FILENAME: &str = ".merino_breakout.txt";

pub const NLEVELS: usize = 8;
pub const NLIVES: u32 = 3;

pub const SCREEN_WIDTH: f32 = 800.;
pub const SCREEN_HEIGHT: f32 = 640.;

pub const GAMEAREA_MINX: f32 = -SCREEN_WIDTH / 2.0;
pub const GAMEAREA_MAXX: f32 = GAMEAREA_MINX + 640.;
pub const GAMEAREA_MINY: f32 = -SCREEN_HEIGHT / 2.0;
pub const GAMEAREA_MAXY: f32 = SCREEN_HEIGHT / 2.0;
pub const GAMEAREA_WIDTH: f32 = GAMEAREA_MAXX - GAMEAREA_MINX;
pub const GAMEAREA_HEIGHT: f32 = GAMEAREA_MAXY - GAMEAREA_MINY;
pub const GAMEAREA_CENTER_X: f32 = GAMEAREA_MINX + GAMEAREA_WIDTH / 2.0;
pub const GAMEAREA_CENTER_Y: f32 = GAMEAREA_MINY + GAMEAREA_HEIGHT / 2.0;
pub const BALLAREA_MINX: f32 = GAMEAREA_MINX + 20.;
pub const BALLAREA_MAXX: f32 = GAMEAREA_MAXX - 20.;
pub const BALLAREA_MAXY: f32 = GAMEAREA_MAXY - 40.;
pub const INFOAREA_CENTER_X: f32 = (SCREEN_WIDTH / 2.0 + GAMEAREA_MAXX) / 2.0;
pub const INFOAREA_LOGO_Y: f32 = 250.;
pub const INFOAREA_HEADER_Y: f32 = 150.;
pub const INFOAREA_MSG_Y: f32 = -250.;
pub const INFOAREA_TIMER_Y: f32 = 75.;
pub const INFOAREA_LIVES_Y: f32 = 0.;
pub const INFOAREA_TIMER: f32 = 3.;

pub const GRID_ROWS: usize = 22;
pub const GRID_COLS: usize = 15;

pub const BRICK_WIDTH: f32 = 40.;
pub const BRICK_HEIGHT: f32 = 20.;
pub const BRICK_SIZE: Vec2 = Vec2::new(BRICK_WIDTH - 2., BRICK_HEIGHT - 2.);
pub const BRICK_TYPES: usize = 15;
pub const BRICK_FRAMES: usize = 4;
pub const IBRICK_FRAMES: usize = 4;
pub const BRICK_FRAMERATE: f32 = 20.0;

pub const PORTAL_FRAMES: usize = 4;
pub const PORTAL_FRAMERATE: f32 = 20.0;

pub const BARREL_TITLES: [&str; BARREL_TYPES] = [
    "Extended!",   // Brown
    "Gun!",        // Red
    "Shrinked!",   // Yellow
    "Magnet!",     // Purple
    "Multiball!",  // Violet
    "Fast!",       // Dark Blue
    "Slow!",       // Light Blue
    "Portal!",     // Light Green
    "Extra Live!", // Green
    "Extra Time!", // Grey
];
pub const BARREL_FRAMERATE: f32 = 9.0;
pub const BARREL_TYPES: usize = 10;
pub const BARREL_FRAMES: usize = 10;
pub const BARREL_SPEED: f32 = 150.;
pub const BARREL_CHANCE: f64 = 0.15;
pub const BARREL_SIZE: Vec2 = Vec2::new(40., 18.);
pub const BARREL_WEIGHTS: [u32; BARREL_TYPES] = [4, 0, 2, 0, 0, 2, 2, 0, 0, 0];

pub const LAYER_BG: f32 = 0.;
pub const LAYER_PORTAL_BG: f32 = 1.;
pub const LAYER_SHADOWS: f32 = 2.;
pub const LAYER_PADDLE: f32 = 3.;
pub const LAYER_BALL: f32 = 3.;
pub const LAYER_BRICKS: f32 = 3.;
pub const LAYER_BARRELS: f32 = 4.;
pub const LAYER_MEANIES: f32 = 4.;
pub const LAYER_GUN: f32 = 5.;
pub const LAYER_EXPLOSIONS: f32 = 6.;
pub const LAYER_PORTAL_FG: f32 = 6.;
pub const LAYER_BANNER: f32 = 7.;

pub const SHADOW_DX: f32 = 10.;
pub const SHADOW_DY: f32 = -10.;

pub const BALL_RADIUS: f32 = 10.;
pub const BALL_INITIAL_SPEED: f32 = 350.;
pub const BALL_INITIAL_ANGLE: f32 = 3. * FRAC_PI_8;
pub const BALL_MIN_SPEED: f32 = 200.;
pub const BALL_MAX_SPEED: f32 = 800.;
pub const BALL_SPEED_DELTA: f32 = 1.15;
pub const BALL_SPEEDUP_IMPACTS: u32 = 50;
pub const BALL_NUDGE_IMPACTS: u32 = 75;
pub const MULTIBALL_MAX: u32 = 27;
pub const MULTIBALL_ANGLE_RANGE: f32 = FRAC_PI_4;
pub const BALL_IMPACT_FRAMES: usize = 4;

// pub const PADDLE_NORMAL: usize = 0;
pub const PADDLE_LARGE: usize = 1;
pub const PADDLE_GUN: usize = 2;
pub const PADDLE_SMALL: usize = 3;
pub const PADDLE_MAGNET: usize = 4;
pub const PADDLE_MAGNET_BORDER: f32 = 5.;
pub const PADDLE_Y: f32 = GAMEAREA_MINY + 30.;
pub const PADDLE_MIN_ANGLE: f32 = 0.5 * FRAC_PI_8;
pub const PADDLE_MAX_ANGLE: f32 = PI - PADDLE_MIN_ANGLE;
pub const PADDLE_MIN_SPEED: f32 = 400.0;
pub const PADDLE_MAX_SPEED: f32 = 800.0;

pub const MEANIES_TYPES: usize = 3;
pub const MEANIES_FRAMERATE: f32 = 8.;
pub const MEANIES_MAX: u32 = 0;
pub const MEANIES_SPEED: f32 = 40.;
pub const MEANIES_NFRAMES: [usize; MEANIES_TYPES] = [8, 8, 10];
pub const MEANIES_PORTAL_X: [f32; 2] = [GAMEAREA_CENTER_X - 165., GAMEAREA_CENTER_X + 165.];
pub const MEANIES_PORTAL_Y: f32 = 300.;
pub const MEANIES_MIN_ANGLE: f32 = -PI - FRAC_PI_8;
pub const MEANIES_MAX_ANGLE: f32 = FRAC_PI_8;
pub const MEANIES_MINY: f32 = GAMEAREA_MINY + 2. * BRICK_HEIGHT;
pub const MEANIES_MAXY: f32 = BALLAREA_MAXY - 2. * BRICK_HEIGHT;
pub const MEANIES_MINX: f32 = BALLAREA_MINX + BRICK_WIDTH;
pub const MEANIES_MAXX: f32 = BALLAREA_MAXX - BRICK_WIDTH;
pub const MEANIES_PER_SECOND: f32 = 1.;
pub const MEANIES_SIZES: [&[Vec2]; MEANIES_TYPES] = [
    &[
        Vec2::new(49., 40.),
        Vec2::new(52., 43.),
        Vec2::new(50., 39.),
        Vec2::new(42., 37.),
        Vec2::new(48., 43.),
        Vec2::new(52., 47.),
        Vec2::new(52., 43.),
        Vec2::new(42., 37.),
    ],
    &[
        Vec2::new(44., 36.),
        Vec2::new(54., 37.),
        Vec2::new(57., 41.),
        Vec2::new(52., 41.),
        Vec2::new(40., 37.),
        Vec2::new(59., 41.),
        Vec2::new(57., 46.),
        Vec2::new(54., 36.),
    ],
    &[
        Vec2::new(36., 36.),
        Vec2::new(51., 45.),
        Vec2::new(61., 48.),
        Vec2::new(57., 52.),
        Vec2::new(36., 36.),
        Vec2::new(57., 49.),
        Vec2::new(61., 48.),
        Vec2::new(51., 45.),
        Vec2::new(36., 36.),
        Vec2::new(51., 47.),
        Vec2::new(61., 50.),
        Vec2::new(57., 47.),
        Vec2::new(37., 36.),
        Vec2::new(57., 47.),
        Vec2::new(61., 48.),
        Vec2::new(51., 47.),
    ],
];

// I made the paddle slignly shorter in the
// vertical direction to improve impact
// dynamics (20 vs 24)
pub const PADDLE_SIZES: [Vec2; 5] = [
    Vec2::new(90., 20.),
    Vec2::new(150., 20.), // Extend paddle
    Vec2::new(92., 20.),  // Gun
    Vec2::new(50., 20.),  // Shrink paddle
    Vec2::new(94., 20.),  // Magnet
];

pub const GUN_TIMER_MS: u64 = 500;
pub const GUN_LEFT_X: f32 = -21.;
pub const GUN_RIGHT_X: f32 = 19.;
pub const GUN_Y: f32 = 25.;

pub const BULLET_IMPACT_FRAMES: usize = 4;
pub const BULLET_SPEED: f32 = 400.;
pub const BULLET_SIZE: Vec2 = Vec2::new(7., 20.);

pub const TRANSITION_BANNER_SECS: u64 = 4;

pub const CIPHER_KEY: &str = "RUMPLESTILTSKIN";
pub const USERNAME_LEN: usize = 6;
pub const CODE_LEN: usize = USERNAME_LEN + 3;
pub const SECRETS: [&str; NLEVELS] = [
    "Multiball",
    "The Magnet",
    "Portal to Hell",
    "The Gun",
    "Arkanoid Tribute",
    "Meanies",
    "X Ray",
    "Credits",
];
pub enum Secret {
    Multiball,
    Magnet,
    Hell,
    Gun,
    Arkanoid,
    Meanies,
    XRay,
    Credits,
}

pub const HINTS: [&str; 8] = [
    "Press left shift to move faster",
    "When you unlock a level, the portal\nwill remain open in following games",
    "Unlock all levels to claim a phonetool icon",
    "To finish the game you need to\ninsert codes from other players\nin the shop",
    "Grey barrels will give you 30 extra seconds",
    "Game progress is autosaved",    
    "Beat the countdown a second time\nto permanently unlock the portal",
    "You get an extra life when\nyou destroy all bricks in a level",
];

pub const LEVEL_TITLES: [&str; NLEVELS] = [
    "Angry Peccy",
    "Free Bananas",
    "Mochinuts",
    "Hell\n(crank sound up to 11 for this one)",
    "Lipu",
    "The Toxic Cloud of Capitalism",
    "Tensors... so complex",
    "PRISM-S Invisible Bug",
];

pub const LEVEL_COLORS: [Srgba; NLEVELS] = [
    Srgba::new(1.0, 0.1, 0.1, 1.0), // Evil Peccy
    Srgba::new(1.0, 0.9, 0.2, 1.0), // Banana
    Srgba::new(1.0, 0.0, 0.5, 1.0), // Mochinuts
    Srgba::new(0.0, 0.5, 0.5, 1.0), // Hell
    Srgba::new(0.9, 0.6, 0.1, 1.0), // Lipu
    Srgba::new(1.0, 1.0, 1.0, 1.0), // Toxic Cloud
    Srgba::new(1.0, 0.0, 0.0, 1.0), // Tensors...
    Srgba::new(0.2, 0.2, 0.2, 1.0), // Invisible Bug
];

pub const LEVEL_TIMERS: [f32; NLEVELS] = [210., 150., 150., 150., 150., 150., 150., 150.];
pub const UNLOCKED_PORTAL_TIMER: f32 = 10.;

// 0: Yellow
// 1: Green
// 2: Blue
// 3: Red
// 4: Magenta
// 5: Dark Yellow
// 6: Dark Green
// 7: Dark Blue
// 8: Dark Red
// 9: Dark Purple
// a: Grey
// b: Dark Grey (1 hit)
// c: Dark Grey (2 hits)
// d: Gold (indestructible)
// e: Invisible

pub const LEVELS: [[&[u8; GRID_COLS]; GRID_ROWS]; NLEVELS] = [
    [
        b"               ",
        b"               ",
        b"               ",
        b"    0000000    ",
        b"   000000000   ",
        b"   0bbb0bbb0   ",
        b"   003000300   ",
        b"   000000000   ",
        b"   000000000   ",
        b"   00     00   ",
        b"   0 00000 0   ",
        b"  00000000000  ",
        b"  00000000000  ",
        b"  00000000000  ",
        b"               ",
        b"  99999999999  ",
        b"               ",
        b"   000   000   ",
        b"   000   000   ",
        b"   000   000   ",
        b"   000   000   ",
        b"               ",
    ],
    [
        b"               ",
        b"               ",
        b"               ",
        b"  5            ",
        b"  0        00  ",
        b"  000     000  ",
        b"   000000000   ",
        b"    0000000    ",
        b"  5   0000     ",
        b"  0        00  ",
        b"  000     000  ",
        b"   000000000   ",
        b"    0000000    ",
        b"  5   0000     ",
        b"  0        00  ",
        b"  000     000  ",
        b"   000000000   ",
        b"    0000000    ",
        b"      0000     ",
        b"               ",
        b"               ",
        b"               ",
    ],
    [
        b"               ",
        b"               ",
        b"  222     333  ",
        b" 22 22   33 33 ",
        b"2     2 3     3",
        b"2     2 3     3",
        b"2     2 3     3",
        b" 22 22   33 33 ",
        b"  222     333  ",
        b"               ",
        b"               ",
        b"               ",
        b"  444     111  ",
        b" 44 44   11 11 ",
        b"4     4 1     1",
        b"d     d d     d",
        b"d     d d     d",
        b" dd dd   dd dd ",
        b"  ddd     ddd  ",
        b"               ",
        b"               ",
        b"               ",
    ],
    [
        b"               ",
        b"               ",
        b"        11     ",
        b"        11     ",
        b"       71      ",
        b"  71   71      ",
        b"  771  71  771 ",
        b"   771 71 771  ",
        b"    77171771   ",
        b" 1   77 777    ",
        b" 11111   7777  ",
        b" 11111   11111 ",
        b"  77777 711111 ",
        b"     17177   1 ",
        b"    1771777    ",
        b"   177 17177   ",
        b"  177  17 177  ",
        b"       17  17  ",
        b"      11       ",
        b"      11       ",
        b"               ",
        b"               ",
    ],
    [
        b"               ",
        b"               ",
        b"   b bbbbb b   ",
        b"  b b     b b  ",
        b" b           b ",
        b" b0         0b ",
        b" b0 000 000 0b ",
        b" b000b0 0b000b ",
        b" b0b0b0 0b0b0b ",
        b" b0b       b0b ",
        b" b0b       b0b ",
        b" bbb  bbb  bbb ",
        b"   b  bbb  b   ",
        b"    b  b  b    ",
        b"    b  b  b    ",
        b"     bbbbb     ",
        b"      3 3      ",
        b"      3 3      ",
        b"      3 3      ",
        b"       3       ",
        b"               ",
        b"               ",
    ],
    [
        b"               ",
        b"               ",
        b"               ",
        b"               ",
        b"    ccccccc    ",
        b"  ccbbbbbbbcc  ",
        b" cbbbbbbbbbbbc ",
        b" c   bbbbb   c ",
        b" c   bbbbb   c ",
        b" cbbbbbbbbbbbc ",
        b" cbbbb   bbbbc ",
        b"  cbb     bbc  ",
        b"  ccbbbbbbbcc  ",
        b"    ccccccc    ",
        b"               ",
        b"   d       d   ",
        b"   ddddddddd   ",
        b"    ddddddd    ",
        b"               ",
        b"               ",
        b"               ",
        b"               ",
    ],
    [
        b"               ",
        b" 77  1         ",
        b"  77 4         ",
        b" 17714         ",
        b"3  77 40       ",
        b"  477  2       ",
        b"   177 4       ",
        b"  4 77 4       ",
        b"    077  1     ",
        b"  24 77 4      ",
        b"   4 1770      ",
        b"    4 77 12    ",
        b"   3  077 4    ",
        b"     1 77 2    ",
        b"      4 7731   ",
        b"     2  77 4   ",
        b"       4 7701  ",
        b"        177 4  ",
        b"      2  0772  ",
        b"        4 77 1 ",
        b"         3 77  ",
        b"        4 1770 ",
    ],
    [
        b"               ",
        b"               ",
        b"               ",
        b"   e       e   ",
        b"  e e     e e  ",
        b"  e e     e e  ",
        b"  e  d   d  e  ",
        b"  e   d d   e  ",
        b"   eed   dee   ",
        b"  e  d e d  e  ",
        b"  e         e  ",
        b"   e  ddd  e   ",
        b"  e  d   d  e  ",
        b" e  e     e  e ",
        b" e  e     e  e ",
        b" e  e     e  e ",
        b" e  e     e  e ",
        b"  e  e   e  e  ",
        b"   e e   e e   ",
        b"    ee   ee    ",
        b"               ",
        b"               ",
    ],
];

pub const CREDITS: &str = "CREDITS\n\
			   \n\
			   Code and levels in the public domain\n\
			   by felipb@amazon.com\n\
			   \n\
			   Arkanoid tunes by Hisayoshi Ogura\n\
			   included as a tribute to his genius.\n\
			   \n\
			   I don't know the author of the Chime ringtone,\n\
			   It may be best that he remains anonymous.\n\
			   \n\
			   All other assets are from Code The Classics Vol 2.\n\
			   Graphics by Dan Malone, Sound by Allister Brimble.\n\
			   Licensed under Creative Commons Attribution-\n\
			   NonCommercial-ShareAlike 3.0 Unported\n\
			   (CC BY-NC-SA 3.0). Free for non-commercial use by\n\
			   properly citing the book and creators. Thank you Sirs.";
