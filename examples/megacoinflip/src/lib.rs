#![no_std]
use core::ptr::{read_volatile, write_volatile};

use megadrive_input::{Controllers, Button};
use megadrive_util::rng::PseudoRng;
use megadrive_sys::vdp::{TileFlags, VDP};
use megadrive_graphics::default_ascii::DEFAULT_FONT_1X1;

static mut NEW_FRAME: u16 = 0;
const TEXT_START: u16 = 4 * 32 + 8;

fn draw_text(vdp: &mut VDP, text: &str) {
    let mut tiles = [TileFlags::default(); 16];
    for (tile, byte) in tiles.iter_mut().zip(text.bytes()) {
        *tile = TileFlags::for_tile(byte as u16 + 1, 0);
    }

    vdp.set_plane_a_tiles_iter(TEXT_START, tiles.iter());
}

#[unsafe(no_mangle)]
pub fn main() -> ! {
    let mut controllers = Controllers::new();
    let mut vdp = VDP::new();

    let mut rng = PseudoRng::from_seed(42);

    vdp.enable_interrupts(false, true, false);

    DEFAULT_FONT_1X1.load(&mut vdp);
    draw_text(&mut vdp, "Press A to flip");
    vdp.enable_display(true);

    let mut button_was_down = false;

    loop {
        controllers.update();

        let button_is_down = controllers
            .controller_state(0)
            .is_some_and(|controller| controller.down(Button::A));
        if button_is_down && !button_was_down {
            let result = if rng.random() & 1 == 0 {
                "Heads!"
            } else {
                "Tails!"
            };
            draw_text(&mut vdp, result);
        }
        button_was_down = button_is_down;

        wait_for_vblank();
    }
}

unsafe extern "C" { fn wait_for_interrupt(); }

fn wait_for_vblank() {
    unsafe {
        while read_volatile(&raw const NEW_FRAME) == 0 {
            wait_for_interrupt();
        }
        NEW_FRAME = 0;
    }
}

#[unsafe(no_mangle)]
fn vblank() {
    unsafe { write_volatile(&raw mut NEW_FRAME, 1) };
}
