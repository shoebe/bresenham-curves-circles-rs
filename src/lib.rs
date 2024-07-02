pub fn setPixel(x: i32, y: i32) {}
pub fn setPixel3D(x: i32, y: i32, z: i32) {}


pub fn plot_line(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
) {
    let mut dx: i32 = i32::abs(x1 - x0);
    let mut sx: i32 = if x0 < x1 {
        1 as i32
    } else {
        -(1 as i32)
    };
    let mut dy: i32 = -i32::abs(y1 - y0);
    let mut sy: i32 = if y0 < y1 {
        1 as i32
    } else {
        -(1 as i32)
    };
    let mut err: i32 = dx + dy;
    let mut e2: i32 = 0;
    loop {
        setPixel(x0, y0);
        e2 = 2 as i32 * err;
        if e2 >= dy {
            if x0 == x1 {
                break;
            }
            err += dy;
            x0 += sx;
        }
        if !(e2 <= dx) {
            continue;
        }
        if y0 == y1 {
            break;
        }
        err += dx;
        y0 += sy;
    };
}

pub fn plot_line_3d(
    mut x0: i32,
    mut y0: i32,
    mut z0: i32,
    mut x1: i32,
    mut y1: i32,
    mut z1: i32,
) {
    let mut dx: i32 = i32::abs(x1 - x0);
    let mut sx: i32 = if x0 < x1 {
        1 as i32
    } else {
        -(1 as i32)
    };
    let mut dy: i32 = i32::abs(y1 - y0);
    let mut sy: i32 = if y0 < y1 {
        1 as i32
    } else {
        -(1 as i32)
    };
    let mut dz: i32 = i32::abs(z1 - z0);
    let mut sz: i32 = if z0 < z1 {
        1 as i32
    } else {
        -(1 as i32)
    };
    let mut dm: i32 = if dx > dy && dx > dz {
        dx
    } else if dy > dz {
        dy
    } else {
        dz
    };
    let mut i: i32 = dm;
    z1 = dm / 2 as i32;
    y1 = z1;
    x1 = y1;
    loop {
        setPixel3D(x0, y0, z0);
        let fresh0 = i;
        i = i - 1;
        if fresh0 == 0 as i32 {
            break;
        }
        x1 -= dx;
        if x1 < 0 as i32 {
            x1 += dm;
            x0 += sx;
        }
        y1 -= dy;
        if y1 < 0 as i32 {
            y1 += dm;
            y0 += sy;
        }
        z1 -= dz;
        if z1 < 0 as i32 {
            z1 += dm;
            z0 += sz;
        }
    };
}

pub fn plot_ellipse(
    mut xm: i32,
    mut ym: i32,
    mut a: i32,
    mut b: i32,
) {
    let mut x: i32 = -a;
    let mut y: i32 = 0 as i32;
    let mut e2: i64 = b as i64 * b as i64;
    let mut err: i64 = x as i64
        * (2 as i32 as i64 * e2 + x as i64) + e2;
    loop {
        setPixel(xm - x, ym + y);
        setPixel(xm + x, ym + y);
        setPixel(xm + x, ym - y);
        setPixel(xm - x, ym - y);
        e2 = 2 as i32 as i64 * err;
        if e2
            >= (x * 2 as i32 + 1 as i32) as i64
                * b as i64 * b as i64
        {
            x += 1;
            err
                += (x * 2 as i32 + 1 as i32) as i64
                    * b as i64 * b as i64;
        }
        if e2
            <= (y * 2 as i32 + 1 as i32) as i64
                * a as i64 * a as i64
        {
            y += 1;
            err
                += (y * 2 as i32 + 1 as i32) as i64
                    * a as i64 * a as i64;
        }
        if !(x <= 0 as i32) {
            break;
        }
    }
    loop {
        let fresh1 = y;
        y = y + 1;
        if !(fresh1 < b) {
            break;
        }
        setPixel(xm, ym + y);
        setPixel(xm, ym - y);
    };
}

pub fn plot_optimized_ellipse(
    mut xm: i32,
    mut ym: i32,
    mut a: i32,
    mut b: i32,
) {
    let mut x: i64 = -a as i64;
    let mut y: i64 = 0 as i32 as i64;
    let mut e2: i64 = b as i64;
    let mut dx: i64 = (1 as i32 as i64
        + 2 as i32 as i64 * x) * e2 * e2;
    let mut dy: i64 = x * x;
    let mut err: i64 = dx + dy;
    loop {
        setPixel(xm - x as i32, ym + y as i32);
        setPixel(xm + x as i32, ym + y as i32);
        setPixel(xm + x as i32, ym - y as i32);
        setPixel(xm - x as i32, ym - y as i32);
        e2 = 2 as i32 as i64 * err;
        if e2 >= dx {
            x += 1;
            dx
                += 2 as i32 as i64 * b as i64
                    * b as i64;
            err += dx;
        }
        if e2 <= dy {
            y += 1;
            dy
                += 2 as i32 as i64 * a as i64
                    * a as i64;
            err += dy;
        }
        if !(x <= 0 as i32 as i64) {
            break;
        }
    }
    loop {
        let fresh2 = y;
        y = y + 1;
        if !(fresh2 < b as i64) {
            break;
        }
        setPixel(xm, ym + y as i32);
        setPixel(xm, ym - y as i32);
    };
}

pub fn plotCircle(
    mut xm: i32,
    mut ym: i32,
    mut r: i32,
) {
    let mut x: i32 = -r;
    let mut y: i32 = 0 as i32;
    let mut err: i32 = 2 as i32 - 2 as i32 * r;
    loop {
        setPixel(xm - x, ym + y);
        setPixel(xm - y, ym - x);
        setPixel(xm + x, ym - y);
        setPixel(xm + y, ym + x);
        r = err;
        if r <= y {
            y += 1;
            err += y * 2 as i32 + 1 as i32;
        }
        if r > x || err > y {
            x += 1;
            err += x * 2 as i32 + 1 as i32;
        }
        if !(x < 0 as i32) {
            break;
        }
    };
}

pub fn plot_ellipse_rect(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
) {
    let mut a: i64 = i32::abs(x1 - x0) as i64;
    let mut b: i64 = i32::abs(y1 - y0) as i64;
    let mut b1: i64 = b & 1 as i32 as i64;
    let mut dx: f64 = 4 as i32 as f64
        * (1.0f64 - a as f64) * b as f64 * b as f64;
    let mut dy: f64 = (4 as i32 as i64
        * (b1 + 1 as i32 as i64) * a * a) as f64;
    let mut err: f64 = dx + dy + (b1 * a * a) as f64;
    let mut e2: f64 = 0.;
    if x0 > x1 {
        x0 = x1;
        x1 = (x1 as i64 + a) as i32;
    }
    if y0 > y1 {
        y0 = y1;
    }
    y0 = (y0 as i64
        + (b + 1 as i32 as i64) / 2 as i32 as i64)
        as i32;
    y1 = (y0 as i64 - b1) as i32;
    a = 8 as i32 as i64 * a * a;
    b1 = 8 as i32 as i64 * b * b;
    loop {
        setPixel(x1, y0);
        setPixel(x0, y0);
        setPixel(x0, y1);
        setPixel(x1, y1);
        e2 = 2 as i32 as f64 * err;
        if e2 <= dy {
            y0 += 1;
            y1 -= 1;
            dy += a as f64;
            err += dy;
        }
        if e2 >= dx || 2 as i32 as f64 * err > dy {
            x0 += 1;
            x1 -= 1;
            dx += b1 as f64;
            err += dx;
        }
        if !(x0 <= x1) {
            break;
        }
    }
    while (y0 - y1) as i64 <= b {
        setPixel(x0 - 1 as i32, y0);
        let fresh3 = y0;
        y0 = y0 + 1;
        setPixel(x1 + 1 as i32, fresh3);
        setPixel(x0 - 1 as i32, y1);
        let fresh4 = y1;
        y1 = y1 - 1;
        setPixel(x1 + 1 as i32, fresh4);
    }
}

pub fn plot_quad_bezier_seg(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) {
    let mut sx: i32 = x2 - x1;
    let mut sy: i32 = y2 - y1;
    let mut xx: i64 = (x0 - x1) as i64;
    let mut yy: i64 = (y0 - y1) as i64;
    let mut xy: i64 = 0;
    let mut dx: f64 = 0.;
    let mut dy: f64 = 0.;
    let mut err: f64 = 0.;
    let mut cur: f64 = (xx * sy as i64 - yy * sx as i64)
        as f64;
    assert!(
        (xx * sx as i64 <= 0 as i32 as i64
            && yy * sy as i64 <= 0 as i32 as i64)
    );
    if sx as i64 * sx as i64 + sy as i64 * sy as i64
        > xx * xx + yy * yy
    {
        x2 = x0;
        x0 = sx + x1;
        y2 = y0;
        y0 = sy + y1;
        cur = -cur;
    }
    if cur != 0 as i32 as f64 {
        xx += sx as i64;
        sx = if x0 < x2 { 1 as i32 } else { -(1 as i32) };
        xx *= sx as i64;
        yy += sy as i64;
        sy = if y0 < y2 { 1 as i32 } else { -(1 as i32) };
        yy *= sy as i64;
        xy = 2 as i32 as i64 * xx * yy;
        xx *= xx;
        yy *= yy;
        if (cur * sx as f64 * sy as f64)
            < 0 as i32 as f64
        {
            xx = -xx;
            yy = -yy;
            xy = -xy;
            cur = -cur;
        }
        dx = 4.0f64 * sy as f64 * cur * (x1 - x0) as f64
            + xx as f64 - xy as f64;
        dy = 4.0f64 * sx as f64 * cur * (y0 - y1) as f64
            + yy as f64 - xy as f64;
        xx += xx;
        yy += yy;
        err = dx + dy + xy as f64;
        loop {
            setPixel(x0, y0);
            if x0 == x2 && y0 == y2 {
                return;
            }
            y1 = (2 as i32 as f64 * err < dx) as i32;
            if 2 as i32 as f64 * err > dy {
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
            if !(dy < 0 as i32 as f64
                && dx > 0 as i32 as f64)
            {
                break;
            }
        }
    }
    plot_line(x0, y0, x2, y2);
}

pub fn plot_quad_bezier(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
) {
    let mut x: i32 = x0 - x1;
    let mut y: i32 = y0 - y1;
    let mut t: f64 = (x0 - 2 as i32 * x1 + x2) as f64;
    let mut r: f64 = 0.;
    if x as i64 * (x2 - x1) as i64 > 0 as i32 as i64 {
        if y as i64 * (y2 - y1) as i64
            > 0 as i32 as i64
        {
            if f64::abs(
                (y0 - 2 as i32 * y1 + y2) as f64 / t
                    * x as f64,
            ) > i32::abs(y) as f64
            {
                x0 = x2;
                x2 = x + x1;
                y0 = y2;
                y2 = y + y1;
            }
        }
        t = (x0 - x1) as f64 / t;
        r = (1 as i32 as f64 - t)
            * ((1 as i32 as f64 - t) * y0 as f64
                + 2.0f64 * t * y1 as f64) + t * t * y2 as f64;
        t = (x0 * x2 - x1 * x1) as f64 * t / (x0 - x1) as f64;
        x = f64::floor(t + 0.5f64) as i32;
        y = f64::floor(r + 0.5f64) as i32;
        r = (y1 - y0) as f64 * (t - x0 as f64)
            / (x1 - x0) as f64 + y0 as f64;
        plot_quad_bezier_seg(x0, y0, x, f64::floor(r + 0.5f64) as i32, x, y);
        r = (y1 - y2) as f64 * (t - x2 as f64)
            / (x1 - x2) as f64 + y2 as f64;
        x1 = x;
        x0 = x1;
        y0 = y;
        y1 = f64::floor(r + 0.5f64) as i32;
    }
    if (y0 - y1) as i64 * (y2 - y1) as i64
        > 0 as i32 as i64
    {
        t = (y0 - 2 as i32 * y1 + y2) as f64;
        t = (y0 - y1) as f64 / t;
        r = (1 as i32 as f64 - t)
            * ((1 as i32 as f64 - t) * x0 as f64
                + 2.0f64 * t * x1 as f64) + t * t * x2 as f64;
        t = (y0 * y2 - y1 * y1) as f64 * t / (y0 - y1) as f64;
        x = f64::floor(r + 0.5f64) as i32;
        y = f64::floor(t + 0.5f64) as i32;
        r = (x1 - x0) as f64 * (t - y0 as f64)
            / (y1 - y0) as f64 + x0 as f64;
        plot_quad_bezier_seg(x0, y0, f64::floor(r + 0.5f64) as i32, y, x, y);
        r = (x1 - x2) as f64 * (t - y2 as f64)
            / (y1 - y2) as f64 + x2 as f64;
        x0 = x;
        x1 = f64::floor(r + 0.5f64) as i32;
        y1 = y;
        y0 = y1;
    }
    plot_quad_bezier_seg(x0, y0, x1, y1, x2, y2);
}

pub fn plot_quad_rational_bezier_seg(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
    mut w: f32,
) {
    let mut sx: i32 = x2 - x1;
    let mut sy: i32 = y2 - y1;
    let mut dx: f64 = (x0 - x2) as f64;
    let mut dy: f64 = (y0 - y2) as f64;
    let mut xx: f64 = (x0 - x1) as f64;
    let mut yy: f64 = (y0 - y1) as f64;
    let mut xy: f64 = xx * sy as f64 + yy * sx as f64;
    let mut cur: f64 = xx * sy as f64 - yy * sx as f64;
    let mut err: f64 = 0.;
    assert!(
        xx * sx as f64 <= 0.0f64 && yy * sy as f64 <= 0.0f64
    );
    if cur != 0.0f64 && w as f64 > 0.0f64 {
        if (sx as i64 * sx as i64
            + sy as i64 * sy as i64) as f64
            > xx * xx + yy * yy
        {
            x2 = x0;
            x0 = (x0 as f64 - dx) as i32;
            y2 = y0;
            y0 = (y0 as f64 - dy) as i32;
            cur = -cur;
        }
        xx = 2.0f64
            * (4.0f64 * w as f64 * sx as f64 * xx + dx * dx);
        yy = 2.0f64
            * (4.0f64 * w as f64 * sy as f64 * yy + dy * dy);
        sx = if x0 < x2 { 1 as i32 } else { -(1 as i32) };
        sy = if y0 < y2 { 1 as i32 } else { -(1 as i32) };
        xy = -2.0f64 * sx as f64 * sy as f64
            * (2.0f64 * w as f64 * xy + dx * dy);
        if (cur * sx as f64 * sy as f64) < 0.0f64 {
            xx = -xx;
            yy = -yy;
            xy = -xy;
            cur = -cur;
        }
        dx = 4.0f64 * w as f64 * (x1 - x0) as f64
            * sy as f64 * cur + xx / 2.0f64 + xy;
        dy = 4.0f64 * w as f64 * (y0 - y1) as f64
            * sx as f64 * cur + yy / 2.0f64 + xy;
        if (w as f64) < 0.5f64 && (dy > xy || dx < xy) {
            cur = (w as f64 + 1.0f64) / 2.0f64;
            w = (w as f64).sqrt() as f32;
            xy = 1.0f64 / (w as f64 + 1.0f64);
            sx = f64::floor(
                (x0 as f64
                    + 2.0f64 * w as f64 * x1 as f64
                    + x2 as f64) * xy / 2.0f64 + 0.5f64,
            ) as i32;
            sy = f64::floor(
                (y0 as f64
                    + 2.0f64 * w as f64 * y1 as f64
                    + y2 as f64) * xy / 2.0f64 + 0.5f64,
            ) as i32;
            dx = f64::floor(
                (w * x1 as f32 + x0 as f32) as f64 * xy
                    + 0.5f64,
            );
            dy = f64::floor(
                (y1 as f32 * w + y0 as f32) as f64 * xy
                    + 0.5f64,
            );
            plot_quad_rational_bezier_seg(
                x0,
                y0,
                dx as i32,
                dy as i32,
                sx,
                sy,
                cur as f32,
            );
            dx = f64::floor(
                (w * x1 as f32 + x2 as f32) as f64 * xy
                    + 0.5f64,
            );
            dy = f64::floor(
                (y1 as f32 * w + y2 as f32) as f64 * xy
                    + 0.5f64,
            );
            plot_quad_rational_bezier_seg(
                sx,
                sy,
                dx as i32,
                dy as i32,
                x2,
                y2,
                cur as f32,
            );
            return;
        }
        err = dx + dy - xy;
        loop {
            setPixel(x0, y0);
            if x0 == x2 && y0 == y2 {
                return;
            }
            x1 = (2 as i32 as f64 * err > dy) as i32;
            y1 = (2 as i32 as f64 * (err + yy) < -dy) as i32;
            if 2 as i32 as f64 * err < dx || y1 != 0 {
                y0 += sy;
                dy += xy;
                dx += xx;
                err += dx;
            }
            if 2 as i32 as f64 * err > dx || x1 != 0 {
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
    plot_line(x0, y0, x2, y2);
}

pub fn plot_quad_rational_bezier(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
    mut w: f32,
) {
    let mut x: i32 = x0 - 2 as i32 * x1 + x2;
    let mut y: i32 = y0 - 2 as i32 * y1 + y2;
    let mut xx: f64 = (x0 - x1) as f64;
    let mut yy: f64 = (y0 - y1) as f64;
    let mut ww: f64 = 0.;
    let mut t: f64 = 0.;
    let mut q: f64 = 0.;
    assert!(w as f64 >= 0.0f64);
    if xx * (x2 - x1) as f64 > 0 as i32 as f64 {
        if yy * (y2 - y1) as f64 > 0 as i32 as f64 {
            if f64::abs(xx * y as f64) > f64::abs(yy * x as f64) {
                x0 = x2;
                x2 = (xx + x1 as f64) as i32;
                y0 = y2;
                y2 = (yy + y1 as f64) as i32;
            }
        }
        if x0 == x2 || w as f64 == 1.0f64 {
            t = (x0 - x1) as f64 / x as f64;
        } else {
            q = f64::sqrt(
                4.0f64 * w as f64 * w as f64
                    * (x0 - x1) as f64 * (x2 - x1) as f64
                    + ((x2 - x0) as i64 * (x2 - x0) as i64)
                        as f64,
            );
            if x1 < x0 {
                q = -q;
            }
            t = (2.0f64 * w as f64 * (x0 - x1) as f64
                - x0 as f64 + x2 as f64 + q)
                / (2.0f64 * (1.0f64 - w as f64)
                    * (x2 - x0) as f64);
        }
        q = 1.0f64
            / (2.0f64 * t * (1.0f64 - t) * (w as f64 - 1.0f64) + 1.0f64);
        xx = (t * t
            * (x0 as f64 - 2.0f64 * w as f64 * x1 as f64
                + x2 as f64)
            + 2.0f64 * t
                * (w * x1 as f32 - x0 as f32) as f64
            + x0 as f64) * q;
        yy = (t * t
            * (y0 as f64 - 2.0f64 * w as f64 * y1 as f64
                + y2 as f64)
            + 2.0f64 * t
                * (w * y1 as f32 - y0 as f32) as f64
            + y0 as f64) * q;
        ww = t * (w as f64 - 1.0f64) + 1.0f64;
        ww *= ww * q;
        w = (((1.0f64 - t) * (w as f64 - 1.0f64) + 1.0f64) * q.sqrt())
            as f32;
        x = f64::floor(xx + 0.5f64) as i32;
        y = f64::floor(yy + 0.5f64) as i32;
        yy = (xx - x0 as f64) * (y1 - y0) as f64
            / (x1 - x0) as f64 + y0 as f64;
        plot_quad_rational_bezier_seg(
            x0,
            y0,
            x,
            f64::floor(yy + 0.5f64) as i32,
            x,
            y,
            ww as f32,
        );
        yy = (xx - x2 as f64) * (y1 - y2) as f64
            / (x1 - x2) as f64 + y2 as f64;
        y1 = f64::floor(yy + 0.5f64) as i32;
        x1 = x;
        x0 = x1;
        y0 = y;
    }
    if (y0 - y1) as i64 * (y2 - y1) as i64
        > 0 as i32 as i64
    {
        if y0 == y2 || w as f64 == 1.0f64 {
            t = (y0 - y1) as f64
                / (y0 as f64 - 2.0f64 * y1 as f64
                    + y2 as f64);
        } else {
            q = f64::sqrt(
                4.0f64 * w as f64 * w as f64
                    * (y0 - y1) as f64 * (y2 - y1) as f64
                    + ((y2 - y0) as i64 * (y2 - y0) as i64)
                        as f64,
            );
            if y1 < y0 {
                q = -q;
            }
            t = (2.0f64 * w as f64 * (y0 - y1) as f64
                - y0 as f64 + y2 as f64 + q)
                / (2.0f64 * (1.0f64 - w as f64)
                    * (y2 - y0) as f64);
        }
        q = 1.0f64
            / (2.0f64 * t * (1.0f64 - t) * (w as f64 - 1.0f64) + 1.0f64);
        xx = (t * t
            * (x0 as f64 - 2.0f64 * w as f64 * x1 as f64
                + x2 as f64)
            + 2.0f64 * t
                * (w * x1 as f32 - x0 as f32) as f64
            + x0 as f64) * q;
        yy = (t * t
            * (y0 as f64 - 2.0f64 * w as f64 * y1 as f64
                + y2 as f64)
            + 2.0f64 * t
                * (w * y1 as f32 - y0 as f32) as f64
            + y0 as f64) * q;
        ww = t * (w as f64 - 1.0f64) + 1.0f64;
        ww *= ww * q;
        w = (((1.0f64 - t) * (w as f64 - 1.0f64) + 1.0f64) * f64::sqrt(q))
            as f32;
        x = f64::floor(xx + 0.5f64) as i32;
        y = f64::floor(yy + 0.5f64) as i32;
        xx = (x1 - x0) as f64 * (yy - y0 as f64)
            / (y1 - y0) as f64 + x0 as f64;
        plot_quad_rational_bezier_seg(
            x0,
            y0,
            f64::floor(xx + 0.5f64) as i32,
            y,
            x,
            y,
            ww as f32,
        );
        xx = (x1 - x2) as f64 * (yy - y2 as f64)
            / (y1 - y2) as f64 + x2 as f64;
        x1 = f64::floor(xx + 0.5f64) as i32;
        x0 = x;
        y1 = y;
        y0 = y1;
    }
    plot_quad_rational_bezier_seg(x0, y0, x1, y1, x2, y2, w * w);
}

pub fn plotRotatedEllipse(
    mut x: i32,
    mut y: i32,
    mut a: i32,
    mut b: i32,
    mut angle: f32,
) {
    let mut xd: f32 = (a as i64 * a as i64) as f32;
    let mut yd: f32 = (b as i64 * b as i64) as f32;
    let mut s: f32 = f64::sin(angle as f64) as f32;
    let mut zd: f32 = (xd - yd) * s;
    xd = f64::sqrt((xd - zd * s) as f64) as f32;
    yd = f64::sqrt((yd + zd * s) as f64) as f32;
    a = (xd as f64 + 0.5f64) as i32;
    b = (yd as f64 + 0.5f64) as i32;
    zd = zd * a as f32 * b as f32 / (xd * yd);
    plotRotatedEllipseRect(
        x - a,
        y - b,
        x + a,
        y + b,
        ((4 as i32 as f32 * zd) as f64
            * f64::cos(angle as f64)) as i64,
    );
}

pub fn plotRotatedEllipseRect(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut zd: i64,
) {
    let mut xd: i32 = x1 - x0;
    let mut yd: i32 = y1 - y0;
    let mut w: f32 = (xd as i64 * yd as i64)
        as f32;
    if zd == 0 as i32 as i64 {
        return plot_ellipse_rect(x0, y0, x1, y1);
    }
    if w as f64 != 0.0f64 {
        w = (w - zd as f32) / (w + w);
    }
    assert!(
        w as f64 <= 1.0f64 && w as f64 >= 0.0f64,
    );
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
    );
    plot_quad_rational_bezier_seg(x0, y0 + yd, x0, y1, x1 - xd, y1, w);
    plot_quad_rational_bezier_seg(
        x1,
        y1 - yd,
        x1,
        y1,
        x1 - xd,
        y1,
        (1.0f64 - w as f64) as f32,
    );
    plot_quad_rational_bezier_seg(x1, y1 - yd, x1, y0, x0 + xd, y0, w);
}

// requires unsafe
/* pub fn plotCubicBezierSeg(
    mut x0: i32,
    mut y0: i32,
    mut x1: f32,
    mut y1: f32,
    mut x2: f32,
    mut y2: f32,
    mut x3: i32,
    mut y3: i32,
) {
    let mut f: i32 = 0;
    let mut fx: i32 = 0;
    let mut fy: i32 = 0;
    let mut leg: i32 = 1 as i32;
    let mut sx: i32 = if x0 < x3 {
        1 as i32
    } else {
        -(1 as i32)
    };
    let mut sy: i32 = if y0 < y3 {
        1 as i32
    } else {
        -(1 as i32)
    };
    let mut xc: f32 = -f64::abs(
        (x0 as f32 + x1 - x2 - x3 as f32) as f64,
    ) as f32;
    let mut xa: f32 = xc
        - (4 as i32 * sx) as f32 * (x1 - x2);
    let mut xb: f32 = sx as f32
        * (x0 as f32 - x1 - x2 + x3 as f32);
    let mut yc: f32 = -f64::abs(
        (y0 as f32 + y1 - y2 - y3 as f32) as f64,
    ) as f32;
    let mut ya: f32 = yc
        - (4 as i32 * sy) as f32 * (y1 - y2);
    let mut yb: f32 = sy as f32
        * (y0 as f32 - y1 - y2 + y3 as f32);
    let mut ab: f64 = 0.;
    let mut ac: f64 = 0.;
    let mut bc: f64 = 0.;
    let mut cb: f64 = 0.;
    let mut xx: f64 = 0.;
    let mut xy: f64 = 0.;
    let mut yy: f64 = 0.;
    let mut dx: f64 = 0.;
    let mut dy: f64 = 0.;
    let mut ex: f64 = 0.;
    let mut pxy: &mut f64 = &mut 0.;
    let mut EP: f64 = 0.01f64;
    assert!(
        ((((x1 - x0 as f32) * (x2 - x3 as f32)) as f64)
            < EP
            && ((((x3 - x0) as f32 * (x1 - x2)) as f64) < EP
                || ((xb * xb) as f64) < (xa * xc) as f64 + EP))
    );
    assert!(
        ((((y1 - y0 as f32) * (y2 - y3 as f32)) as f64)
            < EP
            && ((((y3 - y0) as f32 * (y1 - y2)) as f64) < EP
                || ((yb * yb) as f64) < (ya * yc) as f64 + EP))
    );
    if xa == 0 as i32 as f32 && ya == 0 as i32 as f32
    {
        sx = f64::floor(
            ((3 as i32 as f32 * x1 - x0 as f32
                + 1 as i32 as f32) / 2 as i32 as f32)
                as f64,
        ) as i32;
        sy = f64::floor(
            ((3 as i32 as f32 * y1 - y0 as f32
                + 1 as i32 as f32) / 2 as i32 as f32)
                as f64,
        ) as i32;
        return plot_quad_bezier_seg(x0, y0, sx, sy, x3, y3);
    }
    x1 = (x1 - x0 as f32) * (x1 - x0 as f32)
        + (y1 - y0 as f32) * (y1 - y0 as f32)
        + 1 as i32 as f32;
    x2 = (x2 - x3 as f32) * (x2 - x3 as f32)
        + (y2 - y3 as f32) * (y2 - y3 as f32)
        + 1 as i32 as f32;
    loop {
        ab = (xa * yb - xb * ya) as f64;
        ac = (xa * yc - xc * ya) as f64;
        bc = (xb * yc - xc * yb) as f64;
        ex = ab * (ab + ac - 3 as i32 as f64 * bc) + ac * ac;
        f = (if ex > 0 as i32 as f64 {
            1 as i32 as f64
        } else {
            f64::sqrt(
                (1 as i32 as f32
                    + 1024 as i32 as f32 / x1) as f64,
            )
        }) as i32;
        ab *= f as f64;
        ac *= f as f64;
        bc *= f as f64;
        ex *= (f * f) as f64;
        xy = 9 as i32 as f64 * (ab + ac + bc)
            / 8 as i32 as f64;
        cb = (8 as i32 as f32 * (xa - ya)) as f64;
        dx = 27 as i32 as f64
            * (8 as i32 as f64 * ab
                * (yb * yb - ya * yc) as f64
                + ex
                    * (ya + 2 as i32 as f32 * yb + yc)
                        as f64) / 64 as i32 as f64
            - (ya * ya) as f64 * (xy - ya as f64);
        dy = 27 as i32 as f64
            * (8 as i32 as f64 * ab
                * (xb * xb - xa * xc) as f64
                - ex
                    * (xa + 2 as i32 as f32 * xb + xc)
                        as f64) / 64 as i32 as f64
            - (xa * xa) as f64 * (xy + xa as f64);
        xx = 3 as i32 as f64
            * (3 as i32 as f64 * ab
                * (3 as i32 as f32 * yb * yb - ya * ya
                    - 2 as i32 as f32 * ya * yc) as f64
                - ya as f64
                    * (3 as i32 as f64 * ac
                        * (ya + yb) as f64 + ya as f64 * cb))
            / 4 as i32 as f64;
        yy = 3 as i32 as f64
            * (3 as i32 as f64 * ab
                * (3 as i32 as f32 * xb * xb - xa * xa
                    - 2 as i32 as f32 * xa * xc) as f64
                - xa as f64
                    * (3 as i32 as f64 * ac
                        * (xa + xb) as f64 + xa as f64 * cb))
            / 4 as i32 as f64;
        xy = (xa * ya) as f64
            * (6 as i32 as f64 * ab
                + 6 as i32 as f64 * ac
                - 3 as i32 as f64 * bc + cb);
        ac = (ya * ya) as f64;
        cb = (xa * xa) as f64;
        xy = 3 as i32 as f64
            * (xy
                + (9 as i32 * f) as f64
                    * (cb * yb as f64 * yc as f64
                        - (xb * xc) as f64 * ac)
                - (18 as i32 as f32 * xb * yb) as f64 * ab)
            / 8 as i32 as f64;
        if ex < 0 as i32 as f64 {
            dx = -dx;
            dy = -dy;
            xx = -xx;
            yy = -yy;
            xy = -xy;
            ac = -ac;
            cb = -cb;
        }
        ab = (6 as i32 as f32 * ya) as f64 * ac;
        ac = (-(6 as i32) as f32 * xa) as f64 * ac;
        bc = (6 as i32 as f32 * ya) as f64 * cb;
        cb = (-(6 as i32) as f32 * xa) as f64 * cb;
        dx += xy;
        ex = dx + dy;
        dy += xy;
        pxy = &mut xy;
        fy = f;
        fx = fy;
        's_201: while x0 != x3 && y0 != y3 {
            setPixel(x0, y0);
            loop {
                if dx > *pxy || dy < *pxy {
                    break 's_201;
                }
                y1 = (2 as i32 as f64 * ex - dy) as f32;
                if 2 as i32 as f64 * ex >= dx {
                    fx -= 1;
                    dx += xx;
                    ex += dx;
                    xy += ac;
                    dy += xy;
                    yy += bc;
                    xx += ab;
                }
                if y1 <= 0 as i32 as f32 {
                    fy -= 1;
                    dy += yy;
                    ex += dy;
                    xy += bc;
                    dx += xy;
                    xx += ac;
                    yy += cb;
                }
                if !(fx > 0 as i32 && fy > 0 as i32) {
                    break;
                }
            }
            if 2 as i32 * fx <= f {
                x0 += sx;
                fx += f;
            }
            if 2 as i32 * fy <= f {
                y0 += sy;
                fy += f;
            }
            if pxy == &mut xy as &mut f64
                && dx < 0 as i32 as f64
                && dy > 0 as i32 as f64
            {
                pxy = &mut EP;
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
        leg = leg - 1;
        if !(fresh5 != 0) {
            break;
        }
    }
    plot_line(x0, y0, x3, y3);
}

pub fn plotCubicBezier(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
    mut x3: i32,
    mut y3: i32,
) {
    let mut n: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    let mut xc: i64 = (x0 + x1 - x2 - x3) as i64;
    let mut xa: i64 = xc - (4 as i32 * (x1 - x2)) as i64;
    let mut xb: i64 = (x0 - x1 - x2 + x3) as i64;
    let mut xd: i64 = xb + (4 as i32 * (x1 + x2)) as i64;
    let mut yc: i64 = (y0 + y1 - y2 - y3) as i64;
    let mut ya: i64 = yc - (4 as i32 * (y1 - y2)) as i64;
    let mut yb: i64 = (y0 - y1 - y2 + y3) as i64;
    let mut yd: i64 = yb + (4 as i32 * (y1 + y2)) as i64;
    let mut fx0: f32 = x0 as f32;
    let mut fx1: f32 = 0.;
    let mut fx2: f32 = 0.;
    let mut fx3: f32 = 0.;
    let mut fy0: f32 = y0 as f32;
    let mut fy1: f32 = 0.;
    let mut fy2: f32 = 0.;
    let mut fy3: f32 = 0.;
    let mut t1: f64 = (xb * xb - xa * xc) as f64;
    let mut t2: f64 = 0.;
    let mut t: [f64; 5] = [0.; 5];
    if xa == 0 as i32 as i64 {
        if i32::abs(xc as i32) < 2 as i32 * i32::abs(xb as i32) {
            let fresh6 = n;
            n = n + 1;
            t[fresh6 as usize] = xc as f64 / (2.0f64 * xb as f64);
        }
    } else if t1 > 0.0f64 {
        t2 = f64::sqrt(t1);
        t1 = (xb as f64 - t2) / xa as f64;
        if f64::abs(t1) < 1.0f64 {
            let fresh7 = n;
            n = n + 1;
            t[fresh7 as usize] = t1;
        }
        t1 = (xb as f64 + t2) / xa as f64;
        if f64::abs(t1) < 1.0f64 {
            let fresh8 = n;
            n = n + 1;
            t[fresh8 as usize] = t1;
        }
    }
    t1 = (yb * yb - ya * yc) as f64;
    if ya == 0 as i32 as i64 {
        if i32::abs(yc as i32) < 2 as i32 * i32::abs(yb as i32) {
            let fresh9 = n;
            n = n + 1;
            t[fresh9 as usize] = yc as f64 / (2.0f64 * yb as f64);
        }
    } else if t1 > 0.0f64 {
        t2 = f64::sqrt(t1);
        t1 = (yb as f64 - t2) / ya as f64;
        if f64::abs(t1) < 1.0f64 {
            let fresh10 = n;
            n = n + 1;
            t[fresh10 as usize] = t1;
        }
        t1 = (yb as f64 + t2) / ya as f64;
        if f64::abs(t1) < 1.0f64 {
            let fresh11 = n;
            n = n + 1;
            t[fresh11 as usize] = t1;
        }
    }
    i = 1 as i32;
    while i < n {
        t1 = t[(i - 1 as i32) as usize];
        if t1 > t[i as usize] {
            t[(i - 1 as i32) as usize] = t[i as usize];
            t[i as usize] = t1;
            i = 0 as i32;
        }
        i += 1;
    }
    t1 = -1.0f64;
    t[n as usize] = 1.0f64;
    i = 0 as i32;
    while i <= n {
        t2 = t[i as usize];
        fx1 = ((t1
            * (t1 * xb as f64
                - (2 as i32 as i64 * xc) as f64)
            - t2
                * (t1
                    * (t1 * xa as f64
                        - (2 as i32 as i64 * xb) as f64)
                    + xc as f64) + xd as f64)
            / 8 as i32 as f64 - fx0 as f64)
            as f32;
        fy1 = ((t1
            * (t1 * yb as f64
                - (2 as i32 as i64 * yc) as f64)
            - t2
                * (t1
                    * (t1 * ya as f64
                        - (2 as i32 as i64 * yb) as f64)
                    + yc as f64) + yd as f64)
            / 8 as i32 as f64 - fy0 as f64)
            as f32;
        fx2 = ((t2
            * (t2 * xb as f64
                - (2 as i32 as i64 * xc) as f64)
            - t1
                * (t2
                    * (t2 * xa as f64
                        - (2 as i32 as i64 * xb) as f64)
                    + xc as f64) + xd as f64)
            / 8 as i32 as f64 - fx0 as f64)
            as f32;
        fy2 = ((t2
            * (t2 * yb as f64
                - (2 as i32 as i64 * yc) as f64)
            - t1
                * (t2
                    * (t2 * ya as f64
                        - (2 as i32 as i64 * yb) as f64)
                    + yc as f64) + yd as f64)
            / 8 as i32 as f64 - fy0 as f64)
            as f32;
        fx3 = ((t2
            * (t2
                * ((3 as i32 as i64 * xb) as f64
                    - t2 * xa as f64)
                - (3 as i32 as i64 * xc) as f64)
            + xd as f64) / 8 as i32 as f64)
            as f32;
        fx0 -= fx3;
        fy3 = ((t2
            * (t2
                * ((3 as i32 as i64 * yb) as f64
                    - t2 * ya as f64)
                - (3 as i32 as i64 * yc) as f64)
            + yd as f64) / 8 as i32 as f64)
            as f32;
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
            plotCubicBezierSeg(
                x0,
                y0,
                x0 as f32 + fx1,
                y0 as f32 + fy1,
                x0 as f32 + fx2,
                y0 as f32 + fy2,
                x3,
                y3,
            );
        }
        x0 = x3;
        y0 = y3;
        fx0 = fx3;
        fy0 = fy3;
        t1 = t2;
        i += 1;
    }
} */

fn setPixelAA(x: i32, y: i32, z: i32) {}

pub fn plot_line_AA(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
) {
    let mut sx: i32 = if x0 < x1 {
        1 as i32
    } else {
        -(1 as i32)
    };
    let mut sy: i32 = if y0 < y1 {
        1 as i32
    } else {
        -(1 as i32)
    };
    let mut x2: i32 = 0;
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
        setPixelAA(x0, y0, i32::abs((err - dx + dy) as i32) >> 16 as i32);
        e2 = err;
        x2 = x0;
        if 2 as i32 as i64 * e2 >= -dx {
            if x0 == x1 {
                break;
            }
            if e2 + dy < 0xff0000 as i64 {
                setPixelAA(x0, y0 + sy, ((e2 + dy) >> 16) as i32);
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
            setPixelAA(x2 + sx, y0, ((dx - e2) >> 16) as i32);
        }
        err += dx;
        y0 += sy;
    };
}
