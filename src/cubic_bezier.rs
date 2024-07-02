use crate::{plot_line, plot_quad_bezier_seg};

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

unsafe fn plot_cubic_bezier_seg(
    mut x0: i32,
    mut y0: i32,
    mut x1: f32,
    mut y1: f32,
    mut x2: f32,
    y2: f32,
    mut x3: i32,
    mut y3: i32,
    mut set_pixel: impl FnMut(i32, i32),
) {
    let mut f: i32;
    let mut fx: i32;
    let mut fy: i32;
    let mut leg: i32 = 1_i32;
    let mut sx: i32 = if x0 < x3 { 1_i32 } else { -1_i32 };
    let mut sy: i32 = if y0 < y3 { 1_i32 } else { -1_i32 };
    let xc: f32 = -f64::abs((x0 as f32 + x1 - x2 - x3 as f32) as f64) as f32;
    let xa: f32 = xc - (4_i32 * sx) as f32 * (x1 - x2);
    let mut xb: f32 = sx as f32 * (x0 as f32 - x1 - x2 + x3 as f32);
    let yc: f32 = -f64::abs((y0 as f32 + y1 - y2 - y3 as f32) as f64) as f32;
    let ya: f32 = yc - (4_i32 * sy) as f32 * (y1 - y2);
    let mut yb: f32 = sy as f32 * (y0 as f32 - y1 - y2 + y3 as f32);
    let mut ab: f64;
    let mut ac: f64;
    let mut bc: f64;
    let mut cb: f64;
    let mut xx: f64;
    let mut xy: f64;
    let mut yy: f64;
    let mut dx: f64;
    let mut dy: f64;
    let mut ex: f64;
    let mut pxy: *mut f64;
    let mut ep: f64 = 0.01f64;
    assert!(
        ((((x1 - x0 as f32) * (x2 - x3 as f32)) as f64) < ep
            && ((((x3 - x0) as f32 * (x1 - x2)) as f64) < ep
                || ((xb * xb) as f64) < (xa * xc) as f64 + ep))
    );
    assert!(
        ((((y1 - y0 as f32) * (y2 - y3 as f32)) as f64) < ep
            && ((((y3 - y0) as f32 * (y1 - y2)) as f64) < ep
                || ((yb * yb) as f64) < (ya * yc) as f64 + ep))
    );
    if xa == 0_i32 as f32 && ya == 0_i32 as f32 {
        sx = f64::floor(((3_f32 * x1 - x0 as f32 + 1_f32) / 2_f32) as f64) as i32;
        sy = f64::floor(((3_f32 * y1 - y0 as f32 + 1_f32) / 2_f32) as f64) as i32;
        return plot_quad_bezier_seg(x0, y0, sx, sy, x3, y3, set_pixel);
    }
    x1 = (x1 - x0 as f32) * (x1 - x0 as f32) + (y1 - y0 as f32) * (y1 - y0 as f32) + 1_f32;
    x2 = (x2 - x3 as f32) * (x2 - x3 as f32) + (y2 - y3 as f32) * (y2 - y3 as f32) + 1_f32;
    loop {
        ab = (xa * yb - xb * ya) as f64;
        ac = (xa * yc - xc * ya) as f64;
        bc = (xb * yc - xc * yb) as f64;
        ex = ab * (ab + ac - 3_f64 * bc) + ac * ac;
        f = (if ex > 0_i32 as f64 {
            1_f64
        } else {
            f64::sqrt((1_f32 + 1024_f32 / x1) as f64)
        }) as i32;
        ab *= f as f64;
        ac *= f as f64;
        bc *= f as f64;
        ex *= (f * f) as f64;
        xy = 9_f64 * (ab + ac + bc) / 8_f64;
        cb = (8_f32 * (xa - ya)) as f64;
        dx = 27_f64
            * (8_f64 * ab * (yb * yb - ya * yc) as f64 + ex * (ya + 2_f32 * yb + yc) as f64)
            / 64_f64
            - (ya * ya) as f64 * (xy - ya as f64);
        dy = 27_f64
            * (8_f64 * ab * (xb * xb - xa * xc) as f64 - ex * (xa + 2_f32 * xb + xc) as f64)
            / 64_f64
            - (xa * xa) as f64 * (xy + xa as f64);
        xx = 3_f64
            * (3_f64 * ab * (3_f32 * yb * yb - ya * ya - 2_f32 * ya * yc) as f64
                - ya as f64 * (3_f64 * ac * (ya + yb) as f64 + ya as f64 * cb))
            / 4_f64;
        yy = 3_f64
            * (3_f64 * ab * (3_f32 * xb * xb - xa * xa - 2_f32 * xa * xc) as f64
                - xa as f64 * (3_f64 * ac * (xa + xb) as f64 + xa as f64 * cb))
            / 4_f64;
        xy = (xa * ya) as f64 * (6_f64 * ab + 6_f64 * ac - 3_f64 * bc + cb);
        ac = (ya * ya) as f64;
        cb = (xa * xa) as f64;
        xy = 3_f64
            * (xy + (9_i32 * f) as f64 * (cb * yb as f64 * yc as f64 - (xb * xc) as f64 * ac)
                - (18_f32 * xb * yb) as f64 * ab)
            / 8_f64;
        if ex < 0_i32 as f64 {
            dx = -dx;
            dy = -dy;
            xx = -xx;
            yy = -yy;
            xy = -xy;
            ac = -ac;
            cb = -cb;
        }
        ab = (6_f32 * ya) as f64 * ac;
        ac *= (-6_f32 * xa) as f64;
        bc = (6_f32 * ya) as f64 * cb;
        cb *= (-6_f32 * xa) as f64;
        dx += xy;
        ex = dx + dy;
        dy += xy;
        pxy = &mut xy;
        fy = f;
        fx = fy;
        's_201: while x0 != x3 && y0 != y3 {
            set_pixel(x0, y0);
            loop {
                if dx > *pxy || dy < *pxy {
                    break 's_201;
                }
                y1 = (2_f64 * ex - dy) as f32;
                if 2_f64 * ex >= dx {
                    fx -= 1;
                    dx += xx;
                    ex += dx;
                    xy += ac;
                    dy += xy;
                    yy += bc;
                    xx += ab;
                }
                if y1 <= 0_i32 as f32 {
                    fy -= 1;
                    dy += yy;
                    ex += dy;
                    xy += bc;
                    dx += xy;
                    xx += ac;
                    yy += cb;
                }
                if !(fx > 0_i32 && fy > 0_i32) {
                    break;
                }
            }
            if 2_i32 * fx <= f {
                x0 += sx;
                fx += f;
            }
            if 2_i32 * fy <= f {
                y0 += sy;
                fy += f;
            }
            if pxy == &mut xy as &mut f64 && dx < 0_i32 as f64 && dy > 0_i32 as f64 {
                pxy = &mut ep;
            }
        }
        xx = x0 as f64;
        x0 = x3;
        x3 = xx as i32;
        sx = -sx;
        xb = -xb;
        yy = y0 as f64;
        y0 = y3;
        y3 = yy as i32;
        sy = -sy;
        yb = -yb;
        x1 = x2;
        let fresh5 = leg;
        leg -= 1;
        if fresh5 == 0 {
            break;
        }
    }
    plot_line(x0, y0, x3, y3, set_pixel);
}

/// # Safety
///
/// C implementation uses some weird pointer magic
/// Javascript version doesn't though so probably wouldn't be too hard
/// to make this safe
pub unsafe fn plot_cubic_bezier(
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    mut x3: i32,
    mut y3: i32,
    mut set_pixel: impl FnMut(i32, i32),
) {
    let mut n: i32 = 0_i32;
    let mut i: i32;
    let xc: i64 = (x0 + x1 - x2 - x3) as i64;
    let xa: i64 = xc - (4_i32 * (x1 - x2)) as i64;
    let xb: i64 = (x0 - x1 - x2 + x3) as i64;
    let xd: i64 = xb + (4_i32 * (x1 + x2)) as i64;
    let yc: i64 = (y0 + y1 - y2 - y3) as i64;
    let ya: i64 = yc - (4_i32 * (y1 - y2)) as i64;
    let yb: i64 = (y0 - y1 - y2 + y3) as i64;
    let yd: i64 = yb + (4_i32 * (y1 + y2)) as i64;
    let mut fx0: f32 = x0 as f32;
    let mut fx1: f32;
    let mut fx2: f32;
    let mut fx3: f32;
    let mut fy0: f32 = y0 as f32;
    let mut fy1: f32;
    let mut fy2: f32;
    let mut fy3: f32;
    let mut t1: f64 = (xb * xb - xa * xc) as f64;
    let mut t2: f64;
    let mut t: [f64; 5] = [0.; 5];
    if xa == 0_i32 as i64 {
        if i32::abs(xc as i32) < 2_i32 * i32::abs(xb as i32) {
            let fresh6 = n;
            n += 1;
            t[fresh6 as usize] = xc as f64 / (2.0f64 * xb as f64);
        }
    } else if t1 > 0.0f64 {
        t2 = f64::sqrt(t1);
        t1 = (xb as f64 - t2) / xa as f64;
        if f64::abs(t1) < 1.0f64 {
            let fresh7 = n;
            n += 1;
            t[fresh7 as usize] = t1;
        }
        t1 = (xb as f64 + t2) / xa as f64;
        if f64::abs(t1) < 1.0f64 {
            let fresh8 = n;
            n += 1;
            t[fresh8 as usize] = t1;
        }
    }
    t1 = (yb * yb - ya * yc) as f64;
    if ya == 0_i32 as i64 {
        if i32::abs(yc as i32) < 2_i32 * i32::abs(yb as i32) {
            let fresh9 = n;
            n += 1;
            t[fresh9 as usize] = yc as f64 / (2.0f64 * yb as f64);
        }
    } else if t1 > 0.0f64 {
        t2 = f64::sqrt(t1);
        t1 = (yb as f64 - t2) / ya as f64;
        if f64::abs(t1) < 1.0f64 {
            let fresh10 = n;
            n += 1;
            t[fresh10 as usize] = t1;
        }
        t1 = (yb as f64 + t2) / ya as f64;
        if f64::abs(t1) < 1.0f64 {
            let fresh11 = n;
            n += 1;
            t[fresh11 as usize] = t1;
        }
    }
    i = 1_i32;
    while i < n {
        t1 = t[(i - 1_i32) as usize];
        if t1 > t[i as usize] {
            t[(i - 1_i32) as usize] = t[i as usize];
            t[i as usize] = t1;
            i = 0_i32;
        }
        i += 1;
    }
    t1 = -1.0f64;
    t[n as usize] = 1.0f64;
    i = 0_i32;
    while i <= n {
        t2 = t[i as usize];
        fx1 = ((t1 * (t1 * xb as f64 - (2_i32 as i64 * xc) as f64)
            - t2 * (t1 * (t1 * xa as f64 - (2_i32 as i64 * xb) as f64) + xc as f64)
            + xd as f64)
            / 8_f64
            - fx0 as f64) as f32;
        fy1 = ((t1 * (t1 * yb as f64 - (2_i32 as i64 * yc) as f64)
            - t2 * (t1 * (t1 * ya as f64 - (2_i32 as i64 * yb) as f64) + yc as f64)
            + yd as f64)
            / 8_f64
            - fy0 as f64) as f32;
        fx2 = ((t2 * (t2 * xb as f64 - (2_i32 as i64 * xc) as f64)
            - t1 * (t2 * (t2 * xa as f64 - (2_i32 as i64 * xb) as f64) + xc as f64)
            + xd as f64)
            / 8_f64
            - fx0 as f64) as f32;
        fy2 = ((t2 * (t2 * yb as f64 - (2_i32 as i64 * yc) as f64)
            - t1 * (t2 * (t2 * ya as f64 - (2_i32 as i64 * yb) as f64) + yc as f64)
            + yd as f64)
            / 8_f64
            - fy0 as f64) as f32;
        fx3 = ((t2
            * (t2 * ((3_i32 as i64 * xb) as f64 - t2 * xa as f64) - (3_i32 as i64 * xc) as f64)
            + xd as f64)
            / 8_f64) as f32;
        fx0 -= fx3;
        fy3 = ((t2
            * (t2 * ((3_i32 as i64 * yb) as f64 - t2 * ya as f64) - (3_i32 as i64 * yc) as f64)
            + yd as f64)
            / 8_f64) as f32;
        fy0 -= fy3;
        x3 = f64::floor(fx3 as f64 + 0.5f64) as i32;
        y3 = f64::floor(fy3 as f64 + 0.5f64) as i32;
        if fx0 as f64 != 0.0f64 {
            fx0 = (x0 - x3) as f32 / fx0;
            fx1 *= fx0;
            fx2 *= fx0;
        }
        if fy0 as f64 != 0.0f64 {
            fy0 = (y0 - y3) as f32 / fy0;
            fy1 *= fy0;
            fy2 *= fy0;
        }
        if x0 != x3 || y0 != y3 {
            plot_cubic_bezier_seg(
                x0,
                y0,
                x0 as f32 + fx1,
                y0 as f32 + fy1,
                x0 as f32 + fx2,
                y0 as f32 + fy2,
                x3,
                y3,
                &mut set_pixel,
            );
        }
        x0 = x3;
        y0 = y3;
        fx0 = fx3;
        fy0 = fy3;
        t1 = t2;
        i += 1;
    }
}

/// # Safety
///
/// see plot_cubic_bezier()
pub unsafe fn plot_cubic_spline(
    n: i32,
    x: *mut i32,
    y: *mut i32,
    mut set_pixel: impl FnMut(i32, i32),
) {
    let mut mi: f32 = 0.25f64 as f32;
    let mut m: [f32; 6] = [0.; 6];
    let mut x3: i32 = *x.offset((n - 1_i32) as isize);
    let mut y3: i32 = *y.offset((n - 1_i32) as isize);
    let mut x4: i32 = *x.offset(n as isize);
    let mut y4: i32 = *y.offset(n as isize);
    let mut i: i32;
    let mut x0: i32;
    let mut y0: i32;
    let mut x1: i32;
    let mut y1: i32;
    let mut x2: i32;
    let mut y2: i32;
    assert!(n > 2_i32);
    x0 = 12_i32 * *x.offset(1_i32 as isize) - 3_i32 * *x.offset(0_i32 as isize);
    *x.offset(1_i32 as isize) = x0;
    y0 = 12_i32 * *y.offset(1_i32 as isize) - 3_i32 * *y.offset(0_i32 as isize);
    *y.offset(1_i32 as isize) = y0;
    i = 2_i32;
    while i < n {
        if (i - 2_i32) < 6_i32 {
            mi = (0.25f64 / (2.0f64 - mi as f64)) as f32;
            m[(i - 2_i32) as usize] = mi;
        }
        x0 = f64::floor(
            ((12_i32 * *x.offset(i as isize)) as f32 - (2_i32 * x0) as f32 * mi) as f64 + 0.5f64,
        ) as i32;
        *x.offset(i as isize) = x0;
        y0 = f64::floor(
            ((12_i32 * *y.offset(i as isize)) as f32 - (2_i32 * y0) as f32 * mi) as f64 + 0.5f64,
        ) as i32;
        *y.offset(i as isize) = y0;
        i += 1;
    }
    x2 = f64::floor(((x0 - 3_i32 * x4) as f32 / (7_f32 - 4_f32 * mi)) as f64 + 0.5f64) as i32;
    y2 = f64::floor(((y0 - 3_i32 * y4) as f32 / (7_f32 - 4_f32 * mi)) as f64 + 0.5f64) as i32;
    plot_cubic_bezier(
        x3,
        y3,
        (x2 + x4) / 2_i32,
        (y2 + y4) / 2_i32,
        x4,
        y4,
        x4,
        y4,
        &mut set_pixel,
    );
    if (n - 3_i32) < 6_i32 {
        mi = m[(n - 3_i32) as usize];
    }
    x1 = f64::floor(((*x.offset((n - 2_i32) as isize) - 2_i32 * x2) as f32 * mi) as f64 + 0.5f64)
        as i32;
    y1 = f64::floor(((*y.offset((n - 2_i32) as isize) - 2_i32 * y2) as f32 * mi) as f64 + 0.5f64)
        as i32;
    i = n - 3_i32;
    while i > 0_i32 {
        if i <= 6_i32 {
            mi = m[(i - 1_i32) as usize];
        }
        x0 = f64::floor(((*x.offset(i as isize) - 2_i32 * x1) as f32 * mi) as f64 + 0.5f64) as i32;
        y0 = f64::floor(((*y.offset(i as isize) - 2_i32 * y1) as f32 * mi) as f64 + 0.5f64) as i32;
        x4 = f64::floor((x0 + 4_i32 * x1 + x2 + 3_i32) as f64 / 6.0f64) as i32;
        y4 = f64::floor((y0 + 4_i32 * y1 + y2 + 3_i32) as f64 / 6.0f64) as i32;
        plot_cubic_bezier(
            x4,
            y4,
            f64::floor(((2_i32 * x1 + x2) / 3_i32) as f64 + 0.5f64) as i32,
            f64::floor(((2_i32 * y1 + y2) / 3_i32) as f64 + 0.5f64) as i32,
            f64::floor(((x1 + 2_i32 * x2) / 3_i32) as f64 + 0.5f64) as i32,
            f64::floor(((y1 + 2_i32 * y2) / 3_i32) as f64 + 0.5f64) as i32,
            x3,
            y3,
            &mut set_pixel,
        );
        x3 = x4;
        y3 = y4;
        x2 = x1;
        y2 = y1;
        x1 = x0;
        y1 = y0;
        i -= 1;
    }
    x0 = *x.offset(0_i32 as isize);
    x4 = f64::floor((3_i32 * x0 + 7_i32 * x1 + 2_i32 * x2 + 6_i32) as f64 / 12.0f64) as i32;
    y0 = *y.offset(0_i32 as isize);
    y4 = f64::floor((3_i32 * y0 + 7_i32 * y1 + 2_i32 * y2 + 6_i32) as f64 / 12.0f64) as i32;
    plot_cubic_bezier(
        x4,
        y4,
        f64::floor(((2_i32 * x1 + x2) / 3_i32) as f64 + 0.5f64) as i32,
        f64::floor(((2_i32 * y1 + y2) / 3_i32) as f64 + 0.5f64) as i32,
        f64::floor(((x1 + 2_i32 * x2) / 3_i32) as f64 + 0.5f64) as i32,
        f64::floor(((y1 + 2_i32 * y2) / 3_i32) as f64 + 0.5f64) as i32,
        x3,
        y3,
        &mut set_pixel,
    );
    plot_cubic_bezier(
        x0,
        y0,
        x0,
        y0,
        (x0 + x1) / 2_i32,
        (y0 + y1) / 2_i32,
        x4,
        y4,
        set_pixel,
    );
}
