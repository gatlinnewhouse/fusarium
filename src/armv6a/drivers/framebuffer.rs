// Shout-out https://github.com/BRA1L0R/raspberrypi-1-rust

use super::mailbox::{Mail, Mailable};
use crate::armv6a::{interrupts::without_interrupts, memory::AddressTranslation};
use core::{convert::TryInto, fmt};
use embedded_graphics::{
    mono_font::{iso_8859_4::FONT_9X18, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::Rectangle,
};
use embedded_text::{
    alignment::VerticalAlignment,
    plugin::tail::Tail,
    style::{TextBoxStyle, TextBoxStyleBuilder},
    TextBox,
};

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

pub struct FrameBuffer<'a> {
    pitch: u32,
    rows: u32,
    columns: u32,
    buffer: &'a mut [Pixel],
}

pub struct Painter<'a> {
    fb: &'a mut FrameBuffer<'a>,
}

const CHARACTER_STYLE: MonoTextStyle<'static, Rgb888> =
    MonoTextStyle::new(&FONT_9X18, Rgb888::YELLOW);
const TEXTBOX_STYLE: TextBoxStyle = TextBoxStyleBuilder::new()
    .vertical_alignment(VerticalAlignment::Bottom)
    .build();

#[repr(C)]
#[derive(Debug)]
pub struct FrameBufferMail {
    pub width: u32,
    pub height: u32,
    pub virtual_width: u32,
    pub virtual_height: u32,
    pub pitch: u32,
    pub depth: u32,
    pub x_offset: u32,
    pub y_offset: u32,
    pub pointer: u32,
    pub size: u32,
}

impl Pixel {
    pub fn hex(rgb: u32) -> Pixel {
        Pixel {
            blue: rgb as u8,
            green: (rgb >> 8) as u8,
            red: (rgb >> 16) as u8,
        }
    }
    pub fn to_u32(self) -> u32 {
        (self.blue as u32) + ((self.green as u32) << 8) + ((self.red as u32) << 16)
    }
}

impl From<Pixel> for Rgb888 {
    fn from(value: Pixel) -> Self {
        Rgb888::new(value.red, value.green, value.blue)
    }
}

impl From<Rgb888> for Pixel {
    fn from(value: Rgb888) -> Self {
        // embedded_graphics assumes a different endianess than ARM
        // so we have to reverse their orders when converting
        Pixel {
            blue: value.r(),
            green: value.g(),
            red: value.b(),
        }
    }
}

impl<'a> FrameBuffer<'a> {
    pub fn unitialized() -> FrameBuffer<'a> {
        FrameBuffer {
            buffer: &mut [],
            pitch: 0,
            rows: 0,
            columns: 0,
        }
    }
    pub fn new(pitch: u32, rows: u32, columns: u32, buffer: &'a mut [Pixel]) -> FrameBuffer<'a> {
        FrameBuffer {
            pitch,
            rows,
            buffer,
            columns,
        }
    }
    pub fn get_pitch(&self) -> u32 {
        self.pitch
    }
    pub fn get_rows(&self) -> u32 {
        self.rows
    }
    pub fn get_buffer(&mut self) -> &mut [Pixel] {
        self.buffer
    }
    pub fn get_columns(&mut self) -> u32 {
        self.columns
    }
}

impl<'a> Painter<'a> {
    pub fn new(fb: &'a mut FrameBuffer<'a>) -> Painter<'a> {
        Painter { fb }
    }
    fn buffer(&mut self) -> &mut [Pixel] {
        self.fb.get_buffer()
    }
    pub fn pixel(&mut self, x: u32, y: u32, pix: Pixel) {
        let col = self.fb.get_columns();
        self.buffer()[(y * col + x) as usize] = pix
    }
    pub fn pixel_rgb(&mut self, x: u32, y: u32, rgb: u32) {
        self.pixel(x, y, Pixel::hex(rgb))
    }
    pub fn line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Pixel) {
        let (dx, dy) = ((x1 - x0).abs(), -((y1 - y0).abs()));

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx + dy;

        loop {
            self.pixel(x0 as u32, y0 as u32, color);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let err2 = 2 * err;
            if err2 >= dy {
                err += dy;
                x0 += sx;
            }
            if err2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
    pub fn clear_screen(&mut self) {
        self.buffer().iter_mut().for_each(|p| *p = Pixel::hex(0));
    }
    pub fn print_text(&mut self, args: fmt::Arguments) {
        // Textbox is a hero
        without_interrupts(|| {
            TextBox::with_textbox_style(
                args.as_str().unwrap(),
                Rectangle::new(
                    Point::zero(),
                    Size {
                        width: WIDTH,
                        height: HEIGHT + 16,
                    },
                ),
                CHARACTER_STYLE,
                TEXTBOX_STYLE,
            )
            .add_plugin(Tail)
            .draw(self)
            .unwrap();
        });
    }
}

impl<'a> DrawTarget for Painter<'a> {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if let Ok((x @ 0..=WIDTH, y @ 0..=HEIGHT)) = coord.try_into() {
                self.pixel(x, y, Pixel::from(color));
            }
        }
        Ok(())
    }
}

impl<'a> OriginDimensions for Painter<'a> {
    fn size(&self) -> Size {
        Size::new(WIDTH, HEIGHT)
    }
}

impl Mailable for FrameBufferMail {
    fn write_mail(&mut self) -> Mail {
        Mail::new()
            .data()
            .set_noshift((self as *mut Self as u32).phys_to_uncachedbus())
            .chan()
            .set(1)
    }
}
