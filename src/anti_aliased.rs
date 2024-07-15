/*
functions adapated and taken from:

MIT License

Copyright (c) 2020 zingl

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

pub fn plot_line_aa(
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    mut set_pixel: impl FnMut(i32, i32, u8),
) {
    let mut set_pixel = |x, y, c| set_pixel(x, y, 255 - c);
    let sx: i32 = if x0 < x1 { 1 as i32 } else { -(1 as i32) };
    let sy: i32 = if y0 < y1 { 1 as i32 } else { -(1 as i32) };
    let mut x2: i32;
    let mut dx: i64 = i32::abs(x1 - x0) as i64;
    let mut dy: i64 = i32::abs(y1 - y0) as i64;
    let mut err: i64 = dx * dx + dy * dy;
    let mut e2: i64 = (if err == 0 as i32 as i64 {
        1 as i32 as f64
    } else {
        0xffff7f as i64 as f64 / f64::sqrt(err as f64)
    }) as i64;
    dx *= e2;
    dy *= e2;
    err = dx - dy;
    loop {
        set_pixel(x0, y0, (i32::abs((err - dx + dy) as i32) >> 16) as u8);
        e2 = err;
        x2 = x0;
        if 2 as i32 as i64 * e2 >= -dx {
            if x0 == x1 {
                break;
            }
            if e2 + dy < 0xff0000 as i64 {
                set_pixel(x0, y0 + sy, (e2 + dy >> 16) as u8);
            }
            err -= dy;
            x0 += sx;
        }
        if !(2 as i32 as i64 * e2 <= dy) {
            continue;
        }
        if y0 == y1 {
            break;
        }
        if dx - e2 < 0xff0000 as i64 {
            set_pixel(x2 + sx, y0, (dx - e2 >> 16) as u8);
        }
        err += dx;
        y0 += sy;
    }
}

/// plot an anti-aliased line of width wd
pub fn plot_line_width(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut wd: f32,
    mut set_pixel: impl FnMut(i32, i32, u8),
) {
    let mut set_pixel = |x, y, c| set_pixel(x, y, 255 - c);
    let mut dx: i32 = i32::abs(x1 - x0);
    let mut sx: i32 = if x0 < x1 { 1 as i32 } else { -(1 as i32) };
    let mut dy: i32 = i32::abs(y1 - y0);
    let mut sy: i32 = if y0 < y1 { 1 as i32 } else { -(1 as i32) };
    let mut err: i32 = dx - dy;
    let mut e2: i32 = 0;
    let mut x2: i32 = 0;
    let mut y2: i32 = 0;
    let mut ed: f32 = (if dx + dy == 0 as i32 {
        1 as i32 as f64
    } else {
        f64::sqrt((dx as f32 * dx as f32 + dy as f32 * dy as f32) as f64)
    }) as f32;
    wd = (wd + 1 as i32 as f32) / 2 as i32 as f32;
    loop {
        set_pixel(
            x0,
            y0,
            i32::max(
                0 as i32,
                (255.0 * (i32::abs(err - dx + dy) as f32 / ed - wd + 1 as f32)) as i32,
            ) as u8,
        );
        e2 = err;
        x2 = x0;
        if 2 as i32 * e2 >= -dx {
            e2 += dy;
            y2 = y0;
            while (e2 as f32) < ed * wd && (y1 != y2 || dx > dy) {
                y2 += sy;
                set_pixel(
                    x0,
                    y2,
                    i32::max(
                        0 as i32,
                        (255.0 * (i32::abs(e2) as f32 / ed - wd + 1.0)) as i32,
                    ) as u8,
                );
                e2 += dx;
            }
            if x0 == x1 {
                break;
            }
            e2 = err;
            err -= dy;
            x0 += sx;
        }
        if !(2 as i32 * e2 <= dy) {
            continue;
        }
        e2 = dx - e2;
        while (e2 as f32) < ed * wd && (x1 != x2 || dx < dy) {
            x2 += sx;
            set_pixel(
                x2,
                y0,
                i32::max(
                    0 as i32,
                    (255.0 * (i32::abs(e2) as f32 / ed - wd + 1.0)) as i32,
                ) as u8,
            );
            e2 += dy;
        }
        if y0 == y1 {
            break;
        }
        err += dx;
        y0 += sy;
    }
}

pub fn plot_circle_aa(
    mut xm: i32,
    mut ym: i32,
    mut r: i32,
    mut set_pixel: impl FnMut(i32, i32, u8),
) {
    let mut set_pixel = |x, y, c| set_pixel(x, y, 255 - c);
    let mut x: i32 = -r;
    let mut y: i32 = 0 as i32;
    let mut i: i32 = 0;
    let mut x2: i32 = 0;
    let mut e2: i32 = 0;
    let mut err: i32 = 2 as i32 - 2 as i32 * r;
    r = 1 as i32 - err;
    loop {
        i = 255 * i32::abs(err - 2 as i32 * (x + y) - 2 as i32) / r;
        set_pixel(xm - x, ym + y, i as u8);
        set_pixel(xm - y, ym - x, i as u8);
        set_pixel(xm + x, ym - y, i as u8);
        set_pixel(xm + y, ym + x, i as u8);
        e2 = err;
        x2 = x;
        if err + y > 0 as i32 {
            i = 255 as i32 * (err - 2 as i32 * x - 1 as i32) / r;
            if i < 256 as i32 {
                set_pixel(xm - x, ym + y + 1 as i32, i as u8);
                set_pixel(xm - y - 1 as i32, ym - x, i as u8);
                set_pixel(xm + x, ym - y - 1 as i32, i as u8);
                set_pixel(xm + y + 1 as i32, ym + x, i as u8);
            }
            x += 1;
            err += x * 2 as i32 + 1 as i32;
        }
        if e2 + x2 <= 0 as i32 {
            i = 255 as i32 * (2 as i32 * y + 3 as i32 - e2) / r;
            if i < 256 as i32 {
                set_pixel(xm - x2 - 1 as i32, ym + y, i as u8);
                set_pixel(xm - y, ym - x2 - 1 as i32, i as u8);
                set_pixel(xm + x2 + 1 as i32, ym - y, i as u8);
                set_pixel(xm + y, ym + x2 + 1 as i32, i as u8);
            }
            y += 1;
            err += y * 2 as i32 + 1 as i32;
        }
        if !(x < 0 as i32) {
            break;
        }
    }
}
