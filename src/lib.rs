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
#![allow(clippy::too_many_arguments)]

#[cfg(test)]
mod tests;

pub mod anti_aliased;
pub mod cubic_bezier;

pub fn plot_line(mut x0: i32, mut y0: i32, x1: i32, y1: i32, mut set_pixel: impl FnMut(i32, i32)) {
    let dx: i32 = i32::abs(x1 - x0);
    let sx: i32 = if x0 < x1 { 1_i32 } else { -1_i32 };
    let dy: i32 = -i32::abs(y1 - y0);
    let sy: i32 = if y0 < y1 { 1_i32 } else { -1_i32 };
    let mut err: i32 = dx + dy;
    let mut e2: i32;
    loop {
        set_pixel(x0, y0);
        e2 = 2_i32 * err;
        if e2 >= dy {
            if x0 == x1 {
                break;
            }
            err += dy;
            x0 += sx;
        }
        if e2 > dx {
            continue;
        }
        if y0 == y1 {
            break;
        }
        err += dx;
        y0 += sy;
    }
}

/// (xm,ym): center of ellipse
/// a: width of ellipse
/// b: height of ellipse
pub fn plot_ellipse(xm: i32, ym: i32, a: i32, b: i32, mut set_pixel: impl FnMut(i32, i32)) {
    let mut x: i32 = -a;
    let mut y: i32 = 0_i32;
    let mut e2: i64 = b as i64 * b as i64;
    let mut err: i64 = x as i64 * (2_i32 as i64 * e2 + x as i64) + e2;
    loop {
        set_pixel(xm - x, ym + y);
        set_pixel(xm + x, ym + y);
        set_pixel(xm + x, ym - y);
        set_pixel(xm - x, ym - y);
        e2 = 2_i32 as i64 * err;
        if e2 >= (x * 2_i32 + 1_i32) as i64 * b as i64 * b as i64 {
            x += 1;
            err += (x * 2_i32 + 1_i32) as i64 * b as i64 * b as i64;
        }
        if e2 <= (y * 2_i32 + 1_i32) as i64 * a as i64 * a as i64 {
            y += 1;
            err += (y * 2_i32 + 1_i32) as i64 * a as i64 * a as i64;
        }
        if x > 0_i32 {
            break;
        }
    }
    loop {
        let fresh1 = y;
        y += 1;
        if fresh1 >= b {
            break;
        }
        set_pixel(xm, ym + y);
        set_pixel(xm, ym - y);
    }
}

pub fn plot_optimized_ellipse(
    xm: i32,
    ym: i32,
    a: i32,
    b: i32,
    mut set_pixel: impl FnMut(i32, i32),
) {
    let mut x: i64 = -a as i64;
    let mut y: i64 = 0_i32 as i64;
    let mut e2: i64 = b as i64;
    let mut dx: i64 = (1_i32 as i64 + 2_i32 as i64 * x) * e2 * e2;
    let mut dy: i64 = x * x;
    let mut err: i64 = dx + dy;
    loop {
        set_pixel(xm - x as i32, ym + y as i32);
        set_pixel(xm + x as i32, ym + y as i32);
        set_pixel(xm + x as i32, ym - y as i32);
        set_pixel(xm - x as i32, ym - y as i32);
        e2 = 2_i32 as i64 * err;
        if e2 >= dx {
            x += 1;
            dx += 2_i32 as i64 * b as i64 * b as i64;
            err += dx;
        }
        if e2 <= dy {
            y += 1;
            dy += 2_i32 as i64 * a as i64 * a as i64;
            err += dy;
        }
        if x > 0_i32 as i64 {
            break;
        }
    }
    loop {
        let fresh2 = y;
        y += 1;
        if fresh2 >= b as i64 {
            break;
        }
        set_pixel(xm, ym + y as i32);
        set_pixel(xm, ym - y as i32);
    }
}

pub fn plot_circle(xm: i32, ym: i32, mut r: i32, mut set_pixel: impl FnMut(i32, i32)) {
    let mut x: i32 = -r;
    let mut y: i32 = 0_i32;
    let mut err: i32 = 2_i32 - 2_i32 * r;
    loop {
        set_pixel(xm - x, ym + y);
        set_pixel(xm - y, ym - x);
        set_pixel(xm + x, ym - y);
        set_pixel(xm + y, ym + x);
        r = err;
        if r <= y {
            y += 1;
            err += y * 2_i32 + 1_i32;
        }
        if r > x || err > y {
            x += 1;
            err += x * 2_i32 + 1_i32;
        }
        if x >= 0_i32 {
            break;
        }
    }
}

pub fn plot_ellipse_rect(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut set_pixel: impl FnMut(i32, i32),
) {
    let mut a: i64 = i32::abs(x1 - x0) as i64;
    let b: i64 = i32::abs(y1 - y0) as i64;
    let mut b1: i64 = b & 1_i32 as i64;
    let mut dx: f64 = 4_f64 * (1.0f64 - a as f64) * b as f64 * b as f64;
    let mut dy: f64 = (4_i32 as i64 * (b1 + 1_i32 as i64) * a * a) as f64;
    let mut err: f64 = dx + dy + (b1 * a * a) as f64;
    let mut e2: f64;
    if x0 > x1 {
        x0 = x1;
        x1 = (x1 as i64 + a) as i32;
    }
    if y0 > y1 {
        y0 = y1;
    }
    y0 = (y0 as i64 + (b + 1_i32 as i64) / 2_i32 as i64) as i32;
    y1 = (y0 as i64 - b1) as i32;
    a = 8_i32 as i64 * a * a;
    b1 = 8_i32 as i64 * b * b;
    loop {
        set_pixel(x1, y0);
        set_pixel(x0, y0);
        set_pixel(x0, y1);
        set_pixel(x1, y1);
        e2 = 2_f64 * err;
        if e2 <= dy {
            y0 += 1;
            y1 -= 1;
            dy += a as f64;
            err += dy;
        }
        if e2 >= dx || 2_f64 * err > dy {
            x0 += 1;
            x1 -= 1;
            dx += b1 as f64;
            err += dx;
        }
        if x0 > x1 {
            break;
        }
    }
    while (y0 - y1) as i64 <= b {
        set_pixel(x0 - 1_i32, y0);
        let fresh3 = y0;
        y0 += 1;
        set_pixel(x1 + 1_i32, fresh3);
        set_pixel(x0 - 1_i32, y1);
        let fresh4 = y1;
        y1 -= 1;
        set_pixel(x1 + 1_i32, fresh4);
    }
}

pub fn plot_quad_bezier_seg(
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
    mut set_pixel: impl FnMut(i32, i32),
) {
    let mut sx: i32 = x2 - x1;
    let mut sy: i32 = y2 - y1;
    let mut xx: i64 = (x0 - x1) as i64;
    let mut yy: i64 = (y0 - y1) as i64;
    let mut xy: i64;
    let mut dx: f64;
    let mut dy: f64;
    let mut err: f64;
    let mut cur: f64 = (xx * sy as i64 - yy * sx as i64) as f64;
    assert!((xx * sx as i64 <= 0_i32 as i64 && yy * sy as i64 <= 0_i32 as i64));
    if sx as i64 * sx as i64 + sy as i64 * sy as i64 > xx * xx + yy * yy {
        x2 = x0;
        x0 = sx + x1;
        y2 = y0;
        y0 = sy + y1;
        cur = -cur;
    }
    if cur != 0_i32 as f64 {
        xx += sx as i64;
        sx = if x0 < x2 { 1_i32 } else { -1_i32 };
        xx *= sx as i64;
        yy += sy as i64;
        sy = if y0 < y2 { 1_i32 } else { -1_i32 };
        yy *= sy as i64;
        xy = 2_i32 as i64 * xx * yy;
        xx *= xx;
        yy *= yy;
        if (cur * sx as f64 * sy as f64) < 0_i32 as f64 {
            xx = -xx;
            yy = -yy;
            xy = -xy;
            cur = -cur;
        }
        dx = 4.0f64 * sy as f64 * cur * (x1 - x0) as f64 + xx as f64 - xy as f64;
        dy = 4.0f64 * sx as f64 * cur * (y0 - y1) as f64 + yy as f64 - xy as f64;
        xx += xx;
        yy += yy;
        err = dx + dy + xy as f64;
        loop {
            set_pixel(x0, y0);
            if x0 == x2 && y0 == y2 {
                return;
            }
            y1 = (2_f64 * err < dx) as i32;
            if 2_f64 * err > dy {
                x0 += sx;
                dx -= xy as f64;
                dy += yy as f64;
                err += dy;
            }
            if y1 != 0 {
                y0 += sy;
                dy -= xy as f64;
                dx += xx as f64;
                err += dx;
            }
            if !(dy < 0_i32 as f64 && dx > 0_i32 as f64) {
                break;
            }
        }
    }
    plot_line(x0, y0, x2, y2, set_pixel);
}

pub fn plot_quad_bezier(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
    mut set_pixel: impl FnMut(i32, i32),
) {
    let mut x: i32 = x0 - x1;
    let mut y: i32 = y0 - y1;
    let mut t: f64 = (x0 - 2_i32 * x1 + x2) as f64;
    let mut r: f64;
    if x as i64 * (x2 - x1) as i64 > 0_i32 as i64 {
        if y as i64 * (y2 - y1) as i64 > 0_i32 as i64
            && f64::abs((y0 - 2_i32 * y1 + y2) as f64 / t * x as f64) > i32::abs(y) as f64
        {
            x0 = x2;
            x2 = x + x1;
            y0 = y2;
            y2 = y + y1;
        }
        t = (x0 - x1) as f64 / t;
        r = (1_f64 - t) * ((1_f64 - t) * y0 as f64 + 2.0f64 * t * y1 as f64) + t * t * y2 as f64;
        t = (x0 * x2 - x1 * x1) as f64 * t / (x0 - x1) as f64;
        x = f64::floor(t + 0.5f64) as i32;
        y = f64::floor(r + 0.5f64) as i32;
        r = (y1 - y0) as f64 * (t - x0 as f64) / (x1 - x0) as f64 + y0 as f64;
        plot_quad_bezier_seg(
            x0,
            y0,
            x,
            f64::floor(r + 0.5f64) as i32,
            x,
            y,
            &mut set_pixel,
        );
        r = (y1 - y2) as f64 * (t - x2 as f64) / (x1 - x2) as f64 + y2 as f64;
        x1 = x;
        x0 = x1;
        y0 = y;
        y1 = f64::floor(r + 0.5f64) as i32;
    }
    if (y0 - y1) as i64 * (y2 - y1) as i64 > 0_i32 as i64 {
        t = (y0 - 2_i32 * y1 + y2) as f64;
        t = (y0 - y1) as f64 / t;
        r = (1_f64 - t) * ((1_f64 - t) * x0 as f64 + 2.0f64 * t * x1 as f64) + t * t * x2 as f64;
        t = (y0 * y2 - y1 * y1) as f64 * t / (y0 - y1) as f64;
        x = f64::floor(r + 0.5f64) as i32;
        y = f64::floor(t + 0.5f64) as i32;
        r = (x1 - x0) as f64 * (t - y0 as f64) / (y1 - y0) as f64 + x0 as f64;
        plot_quad_bezier_seg(
            x0,
            y0,
            f64::floor(r + 0.5f64) as i32,
            y,
            x,
            y,
            &mut set_pixel,
        );
        r = (x1 - x2) as f64 * (t - y2 as f64) / (y1 - y2) as f64 + x2 as f64;
        x0 = x;
        x1 = f64::floor(r + 0.5f64) as i32;
        y1 = y;
        y0 = y1;
    }
    plot_quad_bezier_seg(x0, y0, x1, y1, x2, y2, set_pixel);
}

/// same as plot_quad_bezier but the control point is a thru point
pub fn plot_quad_bezier_passthrough(
    x0: i32,
    y0: i32,
    mut x1: i32,
    mut y1: i32,
    x2: i32,
    y2: i32,
    set_pixel: impl FnMut(i32, i32),
) {
    x1 = 2 * x1 - (x0 + x2) / 2;
    y1 = 2 * y1 - (y0 + y2) / 2;
    plot_quad_bezier(x0, y0, x1, y1, x2, y2, set_pixel);
}

fn plot_quad_rational_bezier_seg(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
    mut w: f32,
    mut set_pixel: impl FnMut(i32, i32),
) {
    let mut sx: i32 = x2 - x1;
    let mut sy: i32 = y2 - y1;
    let mut dx: f64 = (x0 - x2) as f64;
    let mut dy: f64 = (y0 - y2) as f64;
    let mut xx: f64 = (x0 - x1) as f64;
    let mut yy: f64 = (y0 - y1) as f64;
    let mut xy: f64 = xx * sy as f64 + yy * sx as f64;
    let mut cur: f64 = xx * sy as f64 - yy * sx as f64;
    let mut err: f64;
    assert!(xx * sx as f64 <= 0.0f64 && yy * sy as f64 <= 0.0f64);
    if cur != 0.0f64 && w as f64 > 0.0f64 {
        if (sx as i64 * sx as i64 + sy as i64 * sy as i64) as f64 > xx * xx + yy * yy {
            x2 = x0;
            x0 = (x0 as f64 - dx) as i32;
            y2 = y0;
            y0 = (y0 as f64 - dy) as i32;
            cur = -cur;
        }
        xx = 2.0f64 * (4.0f64 * w as f64 * sx as f64 * xx + dx * dx);
        yy = 2.0f64 * (4.0f64 * w as f64 * sy as f64 * yy + dy * dy);
        sx = if x0 < x2 { 1_i32 } else { -1_i32 };
        sy = if y0 < y2 { 1_i32 } else { -1_i32 };
        xy = -2.0f64 * sx as f64 * sy as f64 * (2.0f64 * w as f64 * xy + dx * dy);
        if (cur * sx as f64 * sy as f64) < 0.0f64 {
            xx = -xx;
            yy = -yy;
            xy = -xy;
            cur = -cur;
        }
        dx = 4.0f64 * w as f64 * (x1 - x0) as f64 * sy as f64 * cur + xx / 2.0f64 + xy;
        dy = 4.0f64 * w as f64 * (y0 - y1) as f64 * sx as f64 * cur + yy / 2.0f64 + xy;
        if (w as f64) < 0.5f64 && (dy > xy || dx < xy) {
            cur = (w as f64 + 1.0f64) / 2.0f64;
            w = (w as f64).sqrt() as f32;
            xy = 1.0f64 / (w as f64 + 1.0f64);
            sx = f64::floor(
                (x0 as f64 + 2.0f64 * w as f64 * x1 as f64 + x2 as f64) * xy / 2.0f64 + 0.5f64,
            ) as i32;
            sy = f64::floor(
                (y0 as f64 + 2.0f64 * w as f64 * y1 as f64 + y2 as f64) * xy / 2.0f64 + 0.5f64,
            ) as i32;
            dx = f64::floor((w * x1 as f32 + x0 as f32) as f64 * xy + 0.5f64);
            dy = f64::floor((y1 as f32 * w + y0 as f32) as f64 * xy + 0.5f64);
            plot_quad_rational_bezier_seg(
                x0,
                y0,
                dx as i32,
                dy as i32,
                sx,
                sy,
                cur as f32,
                &mut set_pixel,
            );
            dx = f64::floor((w * x1 as f32 + x2 as f32) as f64 * xy + 0.5f64);
            dy = f64::floor((y1 as f32 * w + y2 as f32) as f64 * xy + 0.5f64);
            plot_quad_rational_bezier_seg(
                sx, sy, dx as i32, dy as i32, x2, y2, cur as f32, set_pixel,
            );
            return;
        }
        err = dx + dy - xy;
        loop {
            set_pixel(x0, y0);
            if x0 == x2 && y0 == y2 {
                return;
            }
            x1 = (2_f64 * err > dy) as i32;
            y1 = (2_f64 * (err + yy) < -dy) as i32;
            if 2_f64 * err < dx || y1 != 0 {
                y0 += sy;
                dy += xy;
                dx += xx;
                err += dx;
            }
            if 2_f64 * err > dx || x1 != 0 {
                x0 += sx;
                dx += xy;
                dy += yy;
                err += dy;
            }
            if !(dy <= xy && dx >= xy) {
                break;
            }
        }
    }
    plot_line(x0, y0, x2, y2, set_pixel);
}

pub fn plot_quad_rational_bezier(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
    mut w: f32,
    mut set_pixel: impl FnMut(i32, i32),
) {
    let mut x: i32 = x0 - 2_i32 * x1 + x2;
    let mut y: i32 = y0 - 2_i32 * y1 + y2;
    let mut xx: f64 = (x0 - x1) as f64;
    let mut yy: f64 = (y0 - y1) as f64;
    let mut ww: f64;
    let mut t: f64;
    let mut q: f64;
    assert!(w as f64 >= 0.0f64);
    if xx * (x2 - x1) as f64 > 0_i32 as f64 {
        if yy * (y2 - y1) as f64 > 0_i32 as f64 && f64::abs(xx * y as f64) > f64::abs(yy * x as f64)
        {
            x0 = x2;
            x2 = (xx + x1 as f64) as i32;
            y0 = y2;
            y2 = (yy + y1 as f64) as i32;
        }
        if x0 == x2 || w as f64 == 1.0f64 {
            t = (x0 - x1) as f64 / x as f64;
        } else {
            q = f64::sqrt(
                4.0f64 * w as f64 * w as f64 * (x0 - x1) as f64 * (x2 - x1) as f64
                    + ((x2 - x0) as i64 * (x2 - x0) as i64) as f64,
            );
            if x1 < x0 {
                q = -q;
            }
            t = (2.0f64 * w as f64 * (x0 - x1) as f64 - x0 as f64 + x2 as f64 + q)
                / (2.0f64 * (1.0f64 - w as f64) * (x2 - x0) as f64);
        }
        q = 1.0f64 / (2.0f64 * t * (1.0f64 - t) * (w as f64 - 1.0f64) + 1.0f64);
        xx = (t * t * (x0 as f64 - 2.0f64 * w as f64 * x1 as f64 + x2 as f64)
            + 2.0f64 * t * (w * x1 as f32 - x0 as f32) as f64
            + x0 as f64)
            * q;
        yy = (t * t * (y0 as f64 - 2.0f64 * w as f64 * y1 as f64 + y2 as f64)
            + 2.0f64 * t * (w * y1 as f32 - y0 as f32) as f64
            + y0 as f64)
            * q;
        ww = t * (w as f64 - 1.0f64) + 1.0f64;
        ww = ww * ww * q;
        w = (((1.0f64 - t) * (w as f64 - 1.0f64) + 1.0f64) * q.sqrt()) as f32;
        x = f64::floor(xx + 0.5f64) as i32;
        y = f64::floor(yy + 0.5f64) as i32;
        yy = (xx - x0 as f64) * (y1 - y0) as f64 / (x1 - x0) as f64 + y0 as f64;
        plot_quad_rational_bezier_seg(
            x0,
            y0,
            x,
            f64::floor(yy + 0.5f64) as i32,
            x,
            y,
            ww as f32,
            &mut set_pixel,
        );
        yy = (xx - x2 as f64) * (y1 - y2) as f64 / (x1 - x2) as f64 + y2 as f64;
        y1 = f64::floor(yy + 0.5f64) as i32;
        x1 = x;
        x0 = x1;
        y0 = y;
    }
    if (y0 - y1) as i64 * (y2 - y1) as i64 > 0_i32 as i64 {
        if y0 == y2 || w as f64 == 1.0f64 {
            t = (y0 - y1) as f64 / (y0 as f64 - 2.0f64 * y1 as f64 + y2 as f64);
        } else {
            q = f64::sqrt(
                4.0f64 * w as f64 * w as f64 * (y0 - y1) as f64 * (y2 - y1) as f64
                    + ((y2 - y0) as i64 * (y2 - y0) as i64) as f64,
            );
            if y1 < y0 {
                q = -q;
            }
            t = (2.0f64 * w as f64 * (y0 - y1) as f64 - y0 as f64 + y2 as f64 + q)
                / (2.0f64 * (1.0f64 - w as f64) * (y2 - y0) as f64);
        }
        q = 1.0f64 / (2.0f64 * t * (1.0f64 - t) * (w as f64 - 1.0f64) + 1.0f64);
        xx = (t * t * (x0 as f64 - 2.0f64 * w as f64 * x1 as f64 + x2 as f64)
            + 2.0f64 * t * (w * x1 as f32 - x0 as f32) as f64
            + x0 as f64)
            * q;
        yy = (t * t * (y0 as f64 - 2.0f64 * w as f64 * y1 as f64 + y2 as f64)
            + 2.0f64 * t * (w * y1 as f32 - y0 as f32) as f64
            + y0 as f64)
            * q;
        ww = t * (w as f64 - 1.0f64) + 1.0f64;
        ww = ww * ww * q;
        w = (((1.0f64 - t) * (w as f64 - 1.0f64) + 1.0f64) * f64::sqrt(q)) as f32;
        x = f64::floor(xx + 0.5f64) as i32;
        y = f64::floor(yy + 0.5f64) as i32;
        xx = (x1 - x0) as f64 * (yy - y0 as f64) / (y1 - y0) as f64 + x0 as f64;
        plot_quad_rational_bezier_seg(
            x0,
            y0,
            f64::floor(xx + 0.5f64) as i32,
            y,
            x,
            y,
            ww as f32,
            &mut set_pixel,
        );
        xx = (x1 - x2) as f64 * (yy - y2 as f64) / (y1 - y2) as f64 + x2 as f64;
        x1 = f64::floor(xx + 0.5f64) as i32;
        x0 = x;
        y1 = y;
        y0 = y1;
    }
    plot_quad_rational_bezier_seg(x0, y0, x1, y1, x2, y2, w * w, set_pixel);
}

pub fn plot_rotated_ellipse(
    x: i32,
    y: i32,
    mut a: i32,
    mut b: i32,
    angle: f32,
    set_pixel: impl FnMut(i32, i32),
) {
    let mut xd: f32 = (a as i64 * a as i64) as f32;
    let mut yd: f32 = (b as i64 * b as i64) as f32;
    let s: f32 = f64::sin(angle as f64) as f32;
    let mut zd: f32 = (xd - yd) * s;
    xd = f64::sqrt((xd - zd * s) as f64) as f32;
    yd = f64::sqrt((yd + zd * s) as f64) as f32;
    a = (xd as f64 + 0.5f64) as i32;
    b = (yd as f64 + 0.5f64) as i32;
    zd = zd * a as f32 * b as f32 / (xd * yd);
    plot_rotated_ellipse_rect(
        x - a,
        y - b,
        x + a,
        y + b,
        ((4_f32 * zd) as f64 * f64::cos(angle as f64)) as i64,
        set_pixel,
    );
}

pub fn plot_rotated_ellipse_rect(
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    zd: i64,
    mut set_pixel: impl FnMut(i32, i32),
) {
    let mut xd: i32 = x1 - x0;
    let mut yd: i32 = y1 - y0;
    let mut w: f32 = (xd as i64 * yd as i64) as f32;
    if zd == 0_i32 as i64 {
        return plot_ellipse_rect(x0, y0, x1, y1, &mut set_pixel);
    }
    if w as f64 != 0.0f64 {
        w = (w - zd as f32) / (w + w);
    }
    assert!(w as f64 <= 1.0f64 && w as f64 >= 0.0f64,);
    xd = f64::floor((xd as f32 * w) as f64 + 0.5f64) as i32;
    yd = f64::floor((yd as f32 * w) as f64 + 0.5f64) as i32;
    plot_quad_rational_bezier_seg(
        x0,
        y0 + yd,
        x0,
        y0,
        x0 + xd,
        y0,
        (1.0f64 - w as f64) as f32,
        &mut set_pixel,
    );
    plot_quad_rational_bezier_seg(x0, y0 + yd, x0, y1, x1 - xd, y1, w, &mut set_pixel);
    plot_quad_rational_bezier_seg(
        x1,
        y1 - yd,
        x1,
        y1,
        x1 - xd,
        y1,
        (1.0f64 - w as f64) as f32,
        &mut set_pixel,
    );
    plot_quad_rational_bezier_seg(x1, y1 - yd, x1, y0, x0 + xd, y0, w, &mut set_pixel);
}

/// needs at least 3 points
/// all points will be on the bezier (passthrough)
pub fn plot_quad_spline(points: &mut [(i32, i32)], mut set_pixel: impl FnMut(i32, i32)) {
    const N_MAX: usize = 6;

    let mut mi: f32 = 1_f32;
    let mut m: [f32; N_MAX] = [0.; N_MAX];
    let mut x0: i32;
    let mut y0: i32;
    let mut x1: i32;
    let mut y1: i32;
    let n = points.len() - 1;
    assert!(n > 1);
    let mut x2: i32 = points[n].0;
    let mut y2: i32 = points[n].1;
    x0 = 8 * points[1].0 - 2 * points[0].0;
    points[1].0 = x0;
    y0 = 8 * points[1].1 - 2 * points[0].1;
    points[1].1 = y0;

    for i in 2..n {
        if (i - 2) < N_MAX {
            mi = (1.0 / (6.0 - mi as f64)) as f32;
            m[i - 2] = mi;
        }
        x0 = f64::floor(((8 * points[i].0) as f32 - x0 as f32 * mi) as f64 + 0.5) as i32;
        points[i].0 = x0;
        y0 = f64::floor(((8 * points[i].1) as f32 - y0 as f32 * mi) as f64 + 0.5) as i32;
        points[i].1 = y0;
    }
    x1 = f64::floor((x0 - 2 * x2) as f64 / (5.0 - mi as f64) + 0.5) as i32;
    y1 = f64::floor((y0 - 2 * y2) as f64 / (5.0 - mi as f64) + 0.5) as i32;

    for i in (1..=(n - 2)).rev() {
        if i <= N_MAX {
            mi = m[i - 1];
        }
        x0 = f64::floor(((points[i].0 - x1) as f32 * mi) as f64 + 0.5f64) as i32;
        y0 = f64::floor(((points[i].1 - y1) as f32 * mi) as f64 + 0.5f64) as i32;
        plot_quad_bezier(
            (x0 + x1) / 2_i32,
            (y0 + y1) / 2_i32,
            x1,
            y1,
            x2,
            y2,
            &mut set_pixel,
        );
        x2 = (x0 + x1) / 2_i32;
        x1 = x0;
        y2 = (y0 + y1) / 2_i32;
        y1 = y0;
    }
    plot_quad_bezier(points[0].0, points[0].1, x1, y1, x2, y2, set_pixel);
}
