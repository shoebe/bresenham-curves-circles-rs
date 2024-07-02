#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn plotLine(
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
) {
    let mut dx: libc::c_int = abs(x1 - x0);
    let mut sx: libc::c_int = if x0 < x1 {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    let mut dy: libc::c_int = -abs(y1 - y0);
    let mut sy: libc::c_int = if y0 < y1 {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    let mut err: libc::c_int = dx + dy;
    let mut e2: libc::c_int = 0;
    loop {
        setPixel(x0, y0);
        e2 = 2 as libc::c_int * err;
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
#[no_mangle]
pub unsafe extern "C" fn plotLine3d(
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut z0: libc::c_int,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut z1: libc::c_int,
) {
    let mut dx: libc::c_int = abs(x1 - x0);
    let mut sx: libc::c_int = if x0 < x1 {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    let mut dy: libc::c_int = abs(y1 - y0);
    let mut sy: libc::c_int = if y0 < y1 {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    let mut dz: libc::c_int = abs(z1 - z0);
    let mut sz: libc::c_int = if z0 < z1 {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    let mut dm: libc::c_int = if dx > dy && dx > dz {
        dx
    } else if dy > dz {
        dy
    } else {
        dz
    };
    let mut i: libc::c_int = dm;
    z1 = dm / 2 as libc::c_int;
    y1 = z1;
    x1 = y1;
    loop {
        setPixel(x0, y0, z0);
        let fresh0 = i;
        i = i - 1;
        if fresh0 == 0 as libc::c_int {
            break;
        }
        x1 -= dx;
        if x1 < 0 as libc::c_int {
            x1 += dm;
            x0 += sx;
        }
        y1 -= dy;
        if y1 < 0 as libc::c_int {
            y1 += dm;
            y0 += sy;
        }
        z1 -= dz;
        if z1 < 0 as libc::c_int {
            z1 += dm;
            z0 += sz;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn plotEllipse(
    mut xm: libc::c_int,
    mut ym: libc::c_int,
    mut a: libc::c_int,
    mut b: libc::c_int,
) {
    let mut x: libc::c_int = -a;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut e2: libc::c_long = b as libc::c_long * b as libc::c_long;
    let mut err: libc::c_long = x as libc::c_long
        * (2 as libc::c_int as libc::c_long * e2 + x as libc::c_long) + e2;
    loop {
        setPixel(xm - x, ym + y);
        setPixel(xm + x, ym + y);
        setPixel(xm + x, ym - y);
        setPixel(xm - x, ym - y);
        e2 = 2 as libc::c_int as libc::c_long * err;
        if e2
            >= (x * 2 as libc::c_int + 1 as libc::c_int) as libc::c_long
                * b as libc::c_long * b as libc::c_long
        {
            x += 1;
            err
                += (x * 2 as libc::c_int + 1 as libc::c_int) as libc::c_long
                    * b as libc::c_long * b as libc::c_long;
        }
        if e2
            <= (y * 2 as libc::c_int + 1 as libc::c_int) as libc::c_long
                * a as libc::c_long * a as libc::c_long
        {
            y += 1;
            err
                += (y * 2 as libc::c_int + 1 as libc::c_int) as libc::c_long
                    * a as libc::c_long * a as libc::c_long;
        }
        if !(x <= 0 as libc::c_int) {
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
#[no_mangle]
pub unsafe extern "C" fn plotOptimizedEllipse(
    mut xm: libc::c_int,
    mut ym: libc::c_int,
    mut a: libc::c_int,
    mut b: libc::c_int,
) {
    let mut x: libc::c_long = -a as libc::c_long;
    let mut y: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut e2: libc::c_long = b as libc::c_long;
    let mut dx: libc::c_long = (1 as libc::c_int as libc::c_long
        + 2 as libc::c_int as libc::c_long * x) * e2 * e2;
    let mut dy: libc::c_long = x * x;
    let mut err: libc::c_long = dx + dy;
    loop {
        setPixel(xm as libc::c_long - x, ym as libc::c_long + y);
        setPixel(xm as libc::c_long + x, ym as libc::c_long + y);
        setPixel(xm as libc::c_long + x, ym as libc::c_long - y);
        setPixel(xm as libc::c_long - x, ym as libc::c_long - y);
        e2 = 2 as libc::c_int as libc::c_long * err;
        if e2 >= dx {
            x += 1;
            dx
                += 2 as libc::c_int as libc::c_long * b as libc::c_long
                    * b as libc::c_long;
            err += dx;
        }
        if e2 <= dy {
            y += 1;
            dy
                += 2 as libc::c_int as libc::c_long * a as libc::c_long
                    * a as libc::c_long;
            err += dy;
        }
        if !(x <= 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    loop {
        let fresh2 = y;
        y = y + 1;
        if !(fresh2 < b as libc::c_long) {
            break;
        }
        setPixel(xm, ym as libc::c_long + y);
        setPixel(xm, ym as libc::c_long - y);
    };
}
#[no_mangle]
pub unsafe extern "C" fn plotCircle(
    mut xm: libc::c_int,
    mut ym: libc::c_int,
    mut r: libc::c_int,
) {
    let mut x: libc::c_int = -r;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut err: libc::c_int = 2 as libc::c_int - 2 as libc::c_int * r;
    loop {
        setPixel(xm - x, ym + y);
        setPixel(xm - y, ym - x);
        setPixel(xm + x, ym - y);
        setPixel(xm + y, ym + x);
        r = err;
        if r <= y {
            y += 1;
            err += y * 2 as libc::c_int + 1 as libc::c_int;
        }
        if r > x || err > y {
            x += 1;
            err += x * 2 as libc::c_int + 1 as libc::c_int;
        }
        if !(x < 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn plotEllipseRect(
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
) {
    let mut a: libc::c_long = abs(x1 - x0) as libc::c_long;
    let mut b: libc::c_long = abs(y1 - y0) as libc::c_long;
    let mut b1: libc::c_long = b & 1 as libc::c_int as libc::c_long;
    let mut dx: libc::c_double = 4 as libc::c_int as libc::c_double
        * (1.0f64 - a as libc::c_double) * b as libc::c_double * b as libc::c_double;
    let mut dy: libc::c_double = (4 as libc::c_int as libc::c_long
        * (b1 + 1 as libc::c_int as libc::c_long) * a * a) as libc::c_double;
    let mut err: libc::c_double = dx + dy + (b1 * a * a) as libc::c_double;
    let mut e2: libc::c_double = 0.;
    if x0 > x1 {
        x0 = x1;
        x1 = (x1 as libc::c_long + a) as libc::c_int;
    }
    if y0 > y1 {
        y0 = y1;
    }
    y0 = (y0 as libc::c_long
        + (b + 1 as libc::c_int as libc::c_long) / 2 as libc::c_int as libc::c_long)
        as libc::c_int;
    y1 = (y0 as libc::c_long - b1) as libc::c_int;
    a = 8 as libc::c_int as libc::c_long * a * a;
    b1 = 8 as libc::c_int as libc::c_long * b * b;
    loop {
        setPixel(x1, y0);
        setPixel(x0, y0);
        setPixel(x0, y1);
        setPixel(x1, y1);
        e2 = 2 as libc::c_int as libc::c_double * err;
        if e2 <= dy {
            y0 += 1;
            y1 -= 1;
            dy += a as libc::c_double;
            err += dy;
        }
        if e2 >= dx || 2 as libc::c_int as libc::c_double * err > dy {
            x0 += 1;
            x1 -= 1;
            dx += b1 as libc::c_double;
            err += dx;
        }
        if !(x0 <= x1) {
            break;
        }
    }
    while (y0 - y1) as libc::c_long <= b {
        setPixel(x0 - 1 as libc::c_int, y0);
        let fresh3 = y0;
        y0 = y0 + 1;
        setPixel(x1 + 1 as libc::c_int, fresh3);
        setPixel(x0 - 1 as libc::c_int, y1);
        let fresh4 = y1;
        y1 = y1 - 1;
        setPixel(x1 + 1 as libc::c_int, fresh4);
    }
}
#[no_mangle]
pub unsafe extern "C" fn plotQuadBezierSeg(
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut x2: libc::c_int,
    mut y2: libc::c_int,
) {
    let mut sx: libc::c_int = x2 - x1;
    let mut sy: libc::c_int = y2 - y1;
    let mut xx: libc::c_long = (x0 - x1) as libc::c_long;
    let mut yy: libc::c_long = (y0 - y1) as libc::c_long;
    let mut xy: libc::c_long = 0;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut err: libc::c_double = 0.;
    let mut cur: libc::c_double = (xx * sy as libc::c_long - yy * sx as libc::c_long)
        as libc::c_double;
    assert(
        (xx * sx as libc::c_long <= 0 as libc::c_int as libc::c_long
            && yy * sy as libc::c_long <= 0 as libc::c_int as libc::c_long)
            as libc::c_int,
    );
    if sx as libc::c_long * sx as libc::c_long + sy as libc::c_long * sy as libc::c_long
        > xx * xx + yy * yy
    {
        x2 = x0;
        x0 = sx + x1;
        y2 = y0;
        y0 = sy + y1;
        cur = -cur;
    }
    if cur != 0 as libc::c_int as libc::c_double {
        xx += sx as libc::c_long;
        sx = if x0 < x2 { 1 as libc::c_int } else { -(1 as libc::c_int) };
        xx *= sx as libc::c_long;
        yy += sy as libc::c_long;
        sy = if y0 < y2 { 1 as libc::c_int } else { -(1 as libc::c_int) };
        yy *= sy as libc::c_long;
        xy = 2 as libc::c_int as libc::c_long * xx * yy;
        xx *= xx;
        yy *= yy;
        if (cur * sx as libc::c_double * sy as libc::c_double)
            < 0 as libc::c_int as libc::c_double
        {
            xx = -xx;
            yy = -yy;
            xy = -xy;
            cur = -cur;
        }
        dx = 4.0f64 * sy as libc::c_double * cur * (x1 - x0) as libc::c_double
            + xx as libc::c_double - xy as libc::c_double;
        dy = 4.0f64 * sx as libc::c_double * cur * (y0 - y1) as libc::c_double
            + yy as libc::c_double - xy as libc::c_double;
        xx += xx;
        yy += yy;
        err = dx + dy + xy as libc::c_double;
        loop {
            setPixel(x0, y0);
            if x0 == x2 && y0 == y2 {
                return;
            }
            y1 = (2 as libc::c_int as libc::c_double * err < dx) as libc::c_int;
            if 2 as libc::c_int as libc::c_double * err > dy {
                x0 += sx;
                dx -= xy as libc::c_double;
                dy += yy as libc::c_double;
                err += dy;
            }
            if y1 != 0 {
                y0 += sy;
                dy -= xy as libc::c_double;
                dx += xx as libc::c_double;
                err += dx;
            }
            if !(dy < 0 as libc::c_int as libc::c_double
                && dx > 0 as libc::c_int as libc::c_double)
            {
                break;
            }
        }
    }
    plotLine(x0, y0, x2, y2);
}
#[no_mangle]
pub unsafe extern "C" fn plotQuadBezier(
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut x2: libc::c_int,
    mut y2: libc::c_int,
) {
    let mut x: libc::c_int = x0 - x1;
    let mut y: libc::c_int = y0 - y1;
    let mut t: libc::c_double = (x0 - 2 as libc::c_int * x1 + x2) as libc::c_double;
    let mut r: libc::c_double = 0.;
    if x as libc::c_long * (x2 - x1) as libc::c_long > 0 as libc::c_int as libc::c_long {
        if y as libc::c_long * (y2 - y1) as libc::c_long
            > 0 as libc::c_int as libc::c_long
        {
            if fabs(
                (y0 - 2 as libc::c_int * y1 + y2) as libc::c_double / t
                    * x as libc::c_double,
            ) > abs(y) as libc::c_double
            {
                x0 = x2;
                x2 = x + x1;
                y0 = y2;
                y2 = y + y1;
            }
        }
        t = (x0 - x1) as libc::c_double / t;
        r = (1 as libc::c_int as libc::c_double - t)
            * ((1 as libc::c_int as libc::c_double - t) * y0 as libc::c_double
                + 2.0f64 * t * y1 as libc::c_double) + t * t * y2 as libc::c_double;
        t = (x0 * x2 - x1 * x1) as libc::c_double * t / (x0 - x1) as libc::c_double;
        x = floor(t + 0.5f64) as libc::c_int;
        y = floor(r + 0.5f64) as libc::c_int;
        r = (y1 - y0) as libc::c_double * (t - x0 as libc::c_double)
            / (x1 - x0) as libc::c_double + y0 as libc::c_double;
        plotQuadBezierSeg(x0, y0, x, floor(r + 0.5f64) as libc::c_int, x, y);
        r = (y1 - y2) as libc::c_double * (t - x2 as libc::c_double)
            / (x1 - x2) as libc::c_double + y2 as libc::c_double;
        x1 = x;
        x0 = x1;
        y0 = y;
        y1 = floor(r + 0.5f64) as libc::c_int;
    }
    if (y0 - y1) as libc::c_long * (y2 - y1) as libc::c_long
        > 0 as libc::c_int as libc::c_long
    {
        t = (y0 - 2 as libc::c_int * y1 + y2) as libc::c_double;
        t = (y0 - y1) as libc::c_double / t;
        r = (1 as libc::c_int as libc::c_double - t)
            * ((1 as libc::c_int as libc::c_double - t) * x0 as libc::c_double
                + 2.0f64 * t * x1 as libc::c_double) + t * t * x2 as libc::c_double;
        t = (y0 * y2 - y1 * y1) as libc::c_double * t / (y0 - y1) as libc::c_double;
        x = floor(r + 0.5f64) as libc::c_int;
        y = floor(t + 0.5f64) as libc::c_int;
        r = (x1 - x0) as libc::c_double * (t - y0 as libc::c_double)
            / (y1 - y0) as libc::c_double + x0 as libc::c_double;
        plotQuadBezierSeg(x0, y0, floor(r + 0.5f64) as libc::c_int, y, x, y);
        r = (x1 - x2) as libc::c_double * (t - y2 as libc::c_double)
            / (y1 - y2) as libc::c_double + x2 as libc::c_double;
        x0 = x;
        x1 = floor(r + 0.5f64) as libc::c_int;
        y1 = y;
        y0 = y1;
    }
    plotQuadBezierSeg(x0, y0, x1, y1, x2, y2);
}
#[no_mangle]
pub unsafe extern "C" fn plotQuadRationalBezierSeg(
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut x2: libc::c_int,
    mut y2: libc::c_int,
    mut w: libc::c_float,
) {
    let mut sx: libc::c_int = x2 - x1;
    let mut sy: libc::c_int = y2 - y1;
    let mut dx: libc::c_double = (x0 - x2) as libc::c_double;
    let mut dy: libc::c_double = (y0 - y2) as libc::c_double;
    let mut xx: libc::c_double = (x0 - x1) as libc::c_double;
    let mut yy: libc::c_double = (y0 - y1) as libc::c_double;
    let mut xy: libc::c_double = xx * sy as libc::c_double + yy * sx as libc::c_double;
    let mut cur: libc::c_double = xx * sy as libc::c_double - yy * sx as libc::c_double;
    let mut err: libc::c_double = 0.;
    assert(
        (xx * sx as libc::c_double <= 0.0f64 && yy * sy as libc::c_double <= 0.0f64)
            as libc::c_int,
    );
    if cur != 0.0f64 && w as libc::c_double > 0.0f64 {
        if (sx as libc::c_long * sx as libc::c_long
            + sy as libc::c_long * sy as libc::c_long) as libc::c_double
            > xx * xx + yy * yy
        {
            x2 = x0;
            x0 = (x0 as libc::c_double - dx) as libc::c_int;
            y2 = y0;
            y0 = (y0 as libc::c_double - dy) as libc::c_int;
            cur = -cur;
        }
        xx = 2.0f64
            * (4.0f64 * w as libc::c_double * sx as libc::c_double * xx + dx * dx);
        yy = 2.0f64
            * (4.0f64 * w as libc::c_double * sy as libc::c_double * yy + dy * dy);
        sx = if x0 < x2 { 1 as libc::c_int } else { -(1 as libc::c_int) };
        sy = if y0 < y2 { 1 as libc::c_int } else { -(1 as libc::c_int) };
        xy = -2.0f64 * sx as libc::c_double * sy as libc::c_double
            * (2.0f64 * w as libc::c_double * xy + dx * dy);
        if (cur * sx as libc::c_double * sy as libc::c_double) < 0.0f64 {
            xx = -xx;
            yy = -yy;
            xy = -xy;
            cur = -cur;
        }
        dx = 4.0f64 * w as libc::c_double * (x1 - x0) as libc::c_double
            * sy as libc::c_double * cur + xx / 2.0f64 + xy;
        dy = 4.0f64 * w as libc::c_double * (y0 - y1) as libc::c_double
            * sx as libc::c_double * cur + yy / 2.0f64 + xy;
        if (w as libc::c_double) < 0.5f64 && (dy > xy || dx < xy) {
            cur = (w as libc::c_double + 1.0f64) / 2.0f64;
            w = sqrt(w as libc::c_double) as libc::c_float;
            xy = 1.0f64 / (w as libc::c_double + 1.0f64);
            sx = floor(
                (x0 as libc::c_double
                    + 2.0f64 * w as libc::c_double * x1 as libc::c_double
                    + x2 as libc::c_double) * xy / 2.0f64 + 0.5f64,
            ) as libc::c_int;
            sy = floor(
                (y0 as libc::c_double
                    + 2.0f64 * w as libc::c_double * y1 as libc::c_double
                    + y2 as libc::c_double) * xy / 2.0f64 + 0.5f64,
            ) as libc::c_int;
            dx = floor(
                (w * x1 as libc::c_float + x0 as libc::c_float) as libc::c_double * xy
                    + 0.5f64,
            );
            dy = floor(
                (y1 as libc::c_float * w + y0 as libc::c_float) as libc::c_double * xy
                    + 0.5f64,
            );
            plotQuadRationalBezierSeg(
                x0,
                y0,
                dx as libc::c_int,
                dy as libc::c_int,
                sx,
                sy,
                cur as libc::c_float,
            );
            dx = floor(
                (w * x1 as libc::c_float + x2 as libc::c_float) as libc::c_double * xy
                    + 0.5f64,
            );
            dy = floor(
                (y1 as libc::c_float * w + y2 as libc::c_float) as libc::c_double * xy
                    + 0.5f64,
            );
            plotQuadRationalBezierSeg(
                sx,
                sy,
                dx as libc::c_int,
                dy as libc::c_int,
                x2,
                y2,
                cur as libc::c_float,
            );
            return;
        }
        err = dx + dy - xy;
        loop {
            setPixel(x0, y0);
            if x0 == x2 && y0 == y2 {
                return;
            }
            x1 = (2 as libc::c_int as libc::c_double * err > dy) as libc::c_int;
            y1 = (2 as libc::c_int as libc::c_double * (err + yy) < -dy) as libc::c_int;
            if 2 as libc::c_int as libc::c_double * err < dx || y1 != 0 {
                y0 += sy;
                dy += xy;
                dx += xx;
                err += dx;
            }
            if 2 as libc::c_int as libc::c_double * err > dx || x1 != 0 {
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
    plotLine(x0, y0, x2, y2);
}
#[no_mangle]
pub unsafe extern "C" fn plotQuadRationalBezier(
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut x2: libc::c_int,
    mut y2: libc::c_int,
    mut w: libc::c_float,
) {
    let mut x: libc::c_int = x0 - 2 as libc::c_int * x1 + x2;
    let mut y: libc::c_int = y0 - 2 as libc::c_int * y1 + y2;
    let mut xx: libc::c_double = (x0 - x1) as libc::c_double;
    let mut yy: libc::c_double = (y0 - y1) as libc::c_double;
    let mut ww: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    assert((w as libc::c_double >= 0.0f64) as libc::c_int);
    if xx * (x2 - x1) as libc::c_double > 0 as libc::c_int as libc::c_double {
        if yy * (y2 - y1) as libc::c_double > 0 as libc::c_int as libc::c_double {
            if fabs(xx * y as libc::c_double) > fabs(yy * x as libc::c_double) {
                x0 = x2;
                x2 = (xx + x1 as libc::c_double) as libc::c_int;
                y0 = y2;
                y2 = (yy + y1 as libc::c_double) as libc::c_int;
            }
        }
        if x0 == x2 || w as libc::c_double == 1.0f64 {
            t = (x0 - x1) as libc::c_double / x as libc::c_double;
        } else {
            q = sqrt(
                4.0f64 * w as libc::c_double * w as libc::c_double
                    * (x0 - x1) as libc::c_double * (x2 - x1) as libc::c_double
                    + ((x2 - x0) as libc::c_long * (x2 - x0) as libc::c_long)
                        as libc::c_double,
            );
            if x1 < x0 {
                q = -q;
            }
            t = (2.0f64 * w as libc::c_double * (x0 - x1) as libc::c_double
                - x0 as libc::c_double + x2 as libc::c_double + q)
                / (2.0f64 * (1.0f64 - w as libc::c_double)
                    * (x2 - x0) as libc::c_double);
        }
        q = 1.0f64
            / (2.0f64 * t * (1.0f64 - t) * (w as libc::c_double - 1.0f64) + 1.0f64);
        xx = (t * t
            * (x0 as libc::c_double - 2.0f64 * w as libc::c_double * x1 as libc::c_double
                + x2 as libc::c_double)
            + 2.0f64 * t
                * (w * x1 as libc::c_float - x0 as libc::c_float) as libc::c_double
            + x0 as libc::c_double) * q;
        yy = (t * t
            * (y0 as libc::c_double - 2.0f64 * w as libc::c_double * y1 as libc::c_double
                + y2 as libc::c_double)
            + 2.0f64 * t
                * (w * y1 as libc::c_float - y0 as libc::c_float) as libc::c_double
            + y0 as libc::c_double) * q;
        ww = t * (w as libc::c_double - 1.0f64) + 1.0f64;
        ww *= ww * q;
        w = (((1.0f64 - t) * (w as libc::c_double - 1.0f64) + 1.0f64) * sqrt(q))
            as libc::c_float;
        x = floor(xx + 0.5f64) as libc::c_int;
        y = floor(yy + 0.5f64) as libc::c_int;
        yy = (xx - x0 as libc::c_double) * (y1 - y0) as libc::c_double
            / (x1 - x0) as libc::c_double + y0 as libc::c_double;
        plotQuadRationalBezierSeg(
            x0,
            y0,
            x,
            floor(yy + 0.5f64) as libc::c_int,
            x,
            y,
            ww as libc::c_float,
        );
        yy = (xx - x2 as libc::c_double) * (y1 - y2) as libc::c_double
            / (x1 - x2) as libc::c_double + y2 as libc::c_double;
        y1 = floor(yy + 0.5f64) as libc::c_int;
        x1 = x;
        x0 = x1;
        y0 = y;
    }
    if (y0 - y1) as libc::c_long * (y2 - y1) as libc::c_long
        > 0 as libc::c_int as libc::c_long
    {
        if y0 == y2 || w as libc::c_double == 1.0f64 {
            t = (y0 - y1) as libc::c_double
                / (y0 as libc::c_double - 2.0f64 * y1 as libc::c_double
                    + y2 as libc::c_double);
        } else {
            q = sqrt(
                4.0f64 * w as libc::c_double * w as libc::c_double
                    * (y0 - y1) as libc::c_double * (y2 - y1) as libc::c_double
                    + ((y2 - y0) as libc::c_long * (y2 - y0) as libc::c_long)
                        as libc::c_double,
            );
            if y1 < y0 {
                q = -q;
            }
            t = (2.0f64 * w as libc::c_double * (y0 - y1) as libc::c_double
                - y0 as libc::c_double + y2 as libc::c_double + q)
                / (2.0f64 * (1.0f64 - w as libc::c_double)
                    * (y2 - y0) as libc::c_double);
        }
        q = 1.0f64
            / (2.0f64 * t * (1.0f64 - t) * (w as libc::c_double - 1.0f64) + 1.0f64);
        xx = (t * t
            * (x0 as libc::c_double - 2.0f64 * w as libc::c_double * x1 as libc::c_double
                + x2 as libc::c_double)
            + 2.0f64 * t
                * (w * x1 as libc::c_float - x0 as libc::c_float) as libc::c_double
            + x0 as libc::c_double) * q;
        yy = (t * t
            * (y0 as libc::c_double - 2.0f64 * w as libc::c_double * y1 as libc::c_double
                + y2 as libc::c_double)
            + 2.0f64 * t
                * (w * y1 as libc::c_float - y0 as libc::c_float) as libc::c_double
            + y0 as libc::c_double) * q;
        ww = t * (w as libc::c_double - 1.0f64) + 1.0f64;
        ww *= ww * q;
        w = (((1.0f64 - t) * (w as libc::c_double - 1.0f64) + 1.0f64) * sqrt(q))
            as libc::c_float;
        x = floor(xx + 0.5f64) as libc::c_int;
        y = floor(yy + 0.5f64) as libc::c_int;
        xx = (x1 - x0) as libc::c_double * (yy - y0 as libc::c_double)
            / (y1 - y0) as libc::c_double + x0 as libc::c_double;
        plotQuadRationalBezierSeg(
            x0,
            y0,
            floor(xx + 0.5f64) as libc::c_int,
            y,
            x,
            y,
            ww as libc::c_float,
        );
        xx = (x1 - x2) as libc::c_double * (yy - y2 as libc::c_double)
            / (y1 - y2) as libc::c_double + x2 as libc::c_double;
        x1 = floor(xx + 0.5f64) as libc::c_int;
        x0 = x;
        y1 = y;
        y0 = y1;
    }
    plotQuadRationalBezierSeg(x0, y0, x1, y1, x2, y2, w * w);
}
#[no_mangle]
pub unsafe extern "C" fn plotRotatedEllipse(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut angle: libc::c_float,
) {
    let mut xd: libc::c_float = (a as libc::c_long * a as libc::c_long) as libc::c_float;
    let mut yd: libc::c_float = (b as libc::c_long * b as libc::c_long) as libc::c_float;
    let mut s: libc::c_float = sin(angle as libc::c_double) as libc::c_float;
    let mut zd: libc::c_float = (xd - yd) * s;
    xd = sqrt((xd - zd * s) as libc::c_double) as libc::c_float;
    yd = sqrt((yd + zd * s) as libc::c_double) as libc::c_float;
    a = (xd as libc::c_double + 0.5f64) as libc::c_int;
    b = (yd as libc::c_double + 0.5f64) as libc::c_int;
    zd = zd * a as libc::c_float * b as libc::c_float / (xd * yd);
    plotRotatedEllipseRect(
        x - a,
        y - b,
        x + a,
        y + b,
        ((4 as libc::c_int as libc::c_float * zd) as libc::c_double
            * cos(angle as libc::c_double)) as libc::c_long,
    );
}
#[export_name = "plotRotatedEllipseRect"]
pub unsafe extern "C" fn plotRotatedEllipseRect_0(
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut zd: libc::c_long,
) {
    let mut xd: libc::c_int = x1 - x0;
    let mut yd: libc::c_int = y1 - y0;
    let mut w: libc::c_float = (xd as libc::c_long * yd as libc::c_long)
        as libc::c_float;
    if zd == 0 as libc::c_int as libc::c_long {
        return plotEllipseRect(x0, y0, x1, y1);
    }
    if w as libc::c_double != 0.0f64 {
        w = (w - zd as libc::c_float) / (w + w);
    }
    assert(
        (w as libc::c_double <= 1.0f64 && w as libc::c_double >= 0.0f64) as libc::c_int,
    );
    xd = floor((xd as libc::c_float * w) as libc::c_double + 0.5f64) as libc::c_int;
    yd = floor((yd as libc::c_float * w) as libc::c_double + 0.5f64) as libc::c_int;
    plotQuadRationalBezierSeg(
        x0,
        y0 + yd,
        x0,
        y0,
        x0 + xd,
        y0,
        (1.0f64 - w as libc::c_double) as libc::c_float,
    );
    plotQuadRationalBezierSeg(x0, y0 + yd, x0, y1, x1 - xd, y1, w);
    plotQuadRationalBezierSeg(
        x1,
        y1 - yd,
        x1,
        y1,
        x1 - xd,
        y1,
        (1.0f64 - w as libc::c_double) as libc::c_float,
    );
    plotQuadRationalBezierSeg(x1, y1 - yd, x1, y0, x0 + xd, y0, w);
}
#[no_mangle]
pub unsafe extern "C" fn plotCubicBezierSeg(
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut x1: libc::c_float,
    mut y1: libc::c_float,
    mut x2: libc::c_float,
    mut y2: libc::c_float,
    mut x3: libc::c_int,
    mut y3: libc::c_int,
) {
    let mut f: libc::c_int = 0;
    let mut fx: libc::c_int = 0;
    let mut fy: libc::c_int = 0;
    let mut leg: libc::c_int = 1 as libc::c_int;
    let mut sx: libc::c_int = if x0 < x3 {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    let mut sy: libc::c_int = if y0 < y3 {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    let mut xc: libc::c_float = -fabs(
        (x0 as libc::c_float + x1 - x2 - x3 as libc::c_float) as libc::c_double,
    ) as libc::c_float;
    let mut xa: libc::c_float = xc
        - (4 as libc::c_int * sx) as libc::c_float * (x1 - x2);
    let mut xb: libc::c_float = sx as libc::c_float
        * (x0 as libc::c_float - x1 - x2 + x3 as libc::c_float);
    let mut yc: libc::c_float = -fabs(
        (y0 as libc::c_float + y1 - y2 - y3 as libc::c_float) as libc::c_double,
    ) as libc::c_float;
    let mut ya: libc::c_float = yc
        - (4 as libc::c_int * sy) as libc::c_float * (y1 - y2);
    let mut yb: libc::c_float = sy as libc::c_float
        * (y0 as libc::c_float - y1 - y2 + y3 as libc::c_float);
    let mut ab: libc::c_double = 0.;
    let mut ac: libc::c_double = 0.;
    let mut bc: libc::c_double = 0.;
    let mut cb: libc::c_double = 0.;
    let mut xx: libc::c_double = 0.;
    let mut xy: libc::c_double = 0.;
    let mut yy: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut ex: libc::c_double = 0.;
    let mut pxy: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut EP: libc::c_double = 0.01f64;
    assert(
        ((((x1 - x0 as libc::c_float) * (x2 - x3 as libc::c_float)) as libc::c_double)
            < EP
            && ((((x3 - x0) as libc::c_float * (x1 - x2)) as libc::c_double) < EP
                || ((xb * xb) as libc::c_double) < (xa * xc) as libc::c_double + EP))
            as libc::c_int,
    );
    assert(
        ((((y1 - y0 as libc::c_float) * (y2 - y3 as libc::c_float)) as libc::c_double)
            < EP
            && ((((y3 - y0) as libc::c_float * (y1 - y2)) as libc::c_double) < EP
                || ((yb * yb) as libc::c_double) < (ya * yc) as libc::c_double + EP))
            as libc::c_int,
    );
    if xa == 0 as libc::c_int as libc::c_float && ya == 0 as libc::c_int as libc::c_float
    {
        sx = floor(
            ((3 as libc::c_int as libc::c_float * x1 - x0 as libc::c_float
                + 1 as libc::c_int as libc::c_float) / 2 as libc::c_int as libc::c_float)
                as libc::c_double,
        ) as libc::c_int;
        sy = floor(
            ((3 as libc::c_int as libc::c_float * y1 - y0 as libc::c_float
                + 1 as libc::c_int as libc::c_float) / 2 as libc::c_int as libc::c_float)
                as libc::c_double,
        ) as libc::c_int;
        return plotQuadBezierSeg(x0, y0, sx, sy, x3, y3);
    }
    x1 = (x1 - x0 as libc::c_float) * (x1 - x0 as libc::c_float)
        + (y1 - y0 as libc::c_float) * (y1 - y0 as libc::c_float)
        + 1 as libc::c_int as libc::c_float;
    x2 = (x2 - x3 as libc::c_float) * (x2 - x3 as libc::c_float)
        + (y2 - y3 as libc::c_float) * (y2 - y3 as libc::c_float)
        + 1 as libc::c_int as libc::c_float;
    loop {
        ab = (xa * yb - xb * ya) as libc::c_double;
        ac = (xa * yc - xc * ya) as libc::c_double;
        bc = (xb * yc - xc * yb) as libc::c_double;
        ex = ab * (ab + ac - 3 as libc::c_int as libc::c_double * bc) + ac * ac;
        f = (if ex > 0 as libc::c_int as libc::c_double {
            1 as libc::c_int as libc::c_double
        } else {
            sqrt(
                (1 as libc::c_int as libc::c_float
                    + 1024 as libc::c_int as libc::c_float / x1) as libc::c_double,
            )
        }) as libc::c_int;
        ab *= f as libc::c_double;
        ac *= f as libc::c_double;
        bc *= f as libc::c_double;
        ex *= (f * f) as libc::c_double;
        xy = 9 as libc::c_int as libc::c_double * (ab + ac + bc)
            / 8 as libc::c_int as libc::c_double;
        cb = (8 as libc::c_int as libc::c_float * (xa - ya)) as libc::c_double;
        dx = 27 as libc::c_int as libc::c_double
            * (8 as libc::c_int as libc::c_double * ab
                * (yb * yb - ya * yc) as libc::c_double
                + ex
                    * (ya + 2 as libc::c_int as libc::c_float * yb + yc)
                        as libc::c_double) / 64 as libc::c_int as libc::c_double
            - (ya * ya) as libc::c_double * (xy - ya as libc::c_double);
        dy = 27 as libc::c_int as libc::c_double
            * (8 as libc::c_int as libc::c_double * ab
                * (xb * xb - xa * xc) as libc::c_double
                - ex
                    * (xa + 2 as libc::c_int as libc::c_float * xb + xc)
                        as libc::c_double) / 64 as libc::c_int as libc::c_double
            - (xa * xa) as libc::c_double * (xy + xa as libc::c_double);
        xx = 3 as libc::c_int as libc::c_double
            * (3 as libc::c_int as libc::c_double * ab
                * (3 as libc::c_int as libc::c_float * yb * yb - ya * ya
                    - 2 as libc::c_int as libc::c_float * ya * yc) as libc::c_double
                - ya as libc::c_double
                    * (3 as libc::c_int as libc::c_double * ac
                        * (ya + yb) as libc::c_double + ya as libc::c_double * cb))
            / 4 as libc::c_int as libc::c_double;
        yy = 3 as libc::c_int as libc::c_double
            * (3 as libc::c_int as libc::c_double * ab
                * (3 as libc::c_int as libc::c_float * xb * xb - xa * xa
                    - 2 as libc::c_int as libc::c_float * xa * xc) as libc::c_double
                - xa as libc::c_double
                    * (3 as libc::c_int as libc::c_double * ac
                        * (xa + xb) as libc::c_double + xa as libc::c_double * cb))
            / 4 as libc::c_int as libc::c_double;
        xy = (xa * ya) as libc::c_double
            * (6 as libc::c_int as libc::c_double * ab
                + 6 as libc::c_int as libc::c_double * ac
                - 3 as libc::c_int as libc::c_double * bc + cb);
        ac = (ya * ya) as libc::c_double;
        cb = (xa * xa) as libc::c_double;
        xy = 3 as libc::c_int as libc::c_double
            * (xy
                + (9 as libc::c_int * f) as libc::c_double
                    * (cb * yb as libc::c_double * yc as libc::c_double
                        - (xb * xc) as libc::c_double * ac)
                - (18 as libc::c_int as libc::c_float * xb * yb) as libc::c_double * ab)
            / 8 as libc::c_int as libc::c_double;
        if ex < 0 as libc::c_int as libc::c_double {
            dx = -dx;
            dy = -dy;
            xx = -xx;
            yy = -yy;
            xy = -xy;
            ac = -ac;
            cb = -cb;
        }
        ab = (6 as libc::c_int as libc::c_float * ya) as libc::c_double * ac;
        ac = (-(6 as libc::c_int) as libc::c_float * xa) as libc::c_double * ac;
        bc = (6 as libc::c_int as libc::c_float * ya) as libc::c_double * cb;
        cb = (-(6 as libc::c_int) as libc::c_float * xa) as libc::c_double * cb;
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
                y1 = (2 as libc::c_int as libc::c_double * ex - dy) as libc::c_float;
                if 2 as libc::c_int as libc::c_double * ex >= dx {
                    fx -= 1;
                    dx += xx;
                    ex += dx;
                    xy += ac;
                    dy += xy;
                    yy += bc;
                    xx += ab;
                }
                if y1 <= 0 as libc::c_int as libc::c_float {
                    fy -= 1;
                    dy += yy;
                    ex += dy;
                    xy += bc;
                    dx += xy;
                    xx += ac;
                    yy += cb;
                }
                if !(fx > 0 as libc::c_int && fy > 0 as libc::c_int) {
                    break;
                }
            }
            if 2 as libc::c_int * fx <= f {
                x0 += sx;
                fx += f;
            }
            if 2 as libc::c_int * fy <= f {
                y0 += sy;
                fy += f;
            }
            if pxy == &mut xy as *mut libc::c_double
                && dx < 0 as libc::c_int as libc::c_double
                && dy > 0 as libc::c_int as libc::c_double
            {
                pxy = &mut EP;
            }
        }
        xx = x0 as libc::c_double;
        x0 = x3;
        x3 = xx as libc::c_int;
        sx = -sx;
        xb = -xb;
        yy = y0 as libc::c_double;
        y0 = y3;
        y3 = yy as libc::c_int;
        sy = -sy;
        yb = -yb;
        x1 = x2;
        let fresh5 = leg;
        leg = leg - 1;
        if !(fresh5 != 0) {
            break;
        }
    }
    plotLine(x0, y0, x3, y3);
}
#[no_mangle]
pub unsafe extern "C" fn plotCubicBezier(
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut x2: libc::c_int,
    mut y2: libc::c_int,
    mut x3: libc::c_int,
    mut y3: libc::c_int,
) {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut xc: libc::c_long = (x0 + x1 - x2 - x3) as libc::c_long;
    let mut xa: libc::c_long = xc - (4 as libc::c_int * (x1 - x2)) as libc::c_long;
    let mut xb: libc::c_long = (x0 - x1 - x2 + x3) as libc::c_long;
    let mut xd: libc::c_long = xb + (4 as libc::c_int * (x1 + x2)) as libc::c_long;
    let mut yc: libc::c_long = (y0 + y1 - y2 - y3) as libc::c_long;
    let mut ya: libc::c_long = yc - (4 as libc::c_int * (y1 - y2)) as libc::c_long;
    let mut yb: libc::c_long = (y0 - y1 - y2 + y3) as libc::c_long;
    let mut yd: libc::c_long = yb + (4 as libc::c_int * (y1 + y2)) as libc::c_long;
    let mut fx0: libc::c_float = x0 as libc::c_float;
    let mut fx1: libc::c_float = 0.;
    let mut fx2: libc::c_float = 0.;
    let mut fx3: libc::c_float = 0.;
    let mut fy0: libc::c_float = y0 as libc::c_float;
    let mut fy1: libc::c_float = 0.;
    let mut fy2: libc::c_float = 0.;
    let mut fy3: libc::c_float = 0.;
    let mut t1: libc::c_double = (xb * xb - xa * xc) as libc::c_double;
    let mut t2: libc::c_double = 0.;
    let mut t: [libc::c_double; 5] = [0.; 5];
    if xa == 0 as libc::c_int as libc::c_long {
        if abs(xc as libc::c_int) < 2 as libc::c_int * abs(xb as libc::c_int) {
            let fresh6 = n;
            n = n + 1;
            t[fresh6 as usize] = xc as libc::c_double / (2.0f64 * xb as libc::c_double);
        }
    } else if t1 > 0.0f64 {
        t2 = sqrt(t1);
        t1 = (xb as libc::c_double - t2) / xa as libc::c_double;
        if fabs(t1) < 1.0f64 {
            let fresh7 = n;
            n = n + 1;
            t[fresh7 as usize] = t1;
        }
        t1 = (xb as libc::c_double + t2) / xa as libc::c_double;
        if fabs(t1) < 1.0f64 {
            let fresh8 = n;
            n = n + 1;
            t[fresh8 as usize] = t1;
        }
    }
    t1 = (yb * yb - ya * yc) as libc::c_double;
    if ya == 0 as libc::c_int as libc::c_long {
        if abs(yc as libc::c_int) < 2 as libc::c_int * abs(yb as libc::c_int) {
            let fresh9 = n;
            n = n + 1;
            t[fresh9 as usize] = yc as libc::c_double / (2.0f64 * yb as libc::c_double);
        }
    } else if t1 > 0.0f64 {
        t2 = sqrt(t1);
        t1 = (yb as libc::c_double - t2) / ya as libc::c_double;
        if fabs(t1) < 1.0f64 {
            let fresh10 = n;
            n = n + 1;
            t[fresh10 as usize] = t1;
        }
        t1 = (yb as libc::c_double + t2) / ya as libc::c_double;
        if fabs(t1) < 1.0f64 {
            let fresh11 = n;
            n = n + 1;
            t[fresh11 as usize] = t1;
        }
    }
    i = 1 as libc::c_int;
    while i < n {
        t1 = t[(i - 1 as libc::c_int) as usize];
        if t1 > t[i as usize] {
            t[(i - 1 as libc::c_int) as usize] = t[i as usize];
            t[i as usize] = t1;
            i = 0 as libc::c_int;
        }
        i += 1;
    }
    t1 = -1.0f64;
    t[n as usize] = 1.0f64;
    i = 0 as libc::c_int;
    while i <= n {
        t2 = t[i as usize];
        fx1 = ((t1
            * (t1 * xb as libc::c_double
                - (2 as libc::c_int as libc::c_long * xc) as libc::c_double)
            - t2
                * (t1
                    * (t1 * xa as libc::c_double
                        - (2 as libc::c_int as libc::c_long * xb) as libc::c_double)
                    + xc as libc::c_double) + xd as libc::c_double)
            / 8 as libc::c_int as libc::c_double - fx0 as libc::c_double)
            as libc::c_float;
        fy1 = ((t1
            * (t1 * yb as libc::c_double
                - (2 as libc::c_int as libc::c_long * yc) as libc::c_double)
            - t2
                * (t1
                    * (t1 * ya as libc::c_double
                        - (2 as libc::c_int as libc::c_long * yb) as libc::c_double)
                    + yc as libc::c_double) + yd as libc::c_double)
            / 8 as libc::c_int as libc::c_double - fy0 as libc::c_double)
            as libc::c_float;
        fx2 = ((t2
            * (t2 * xb as libc::c_double
                - (2 as libc::c_int as libc::c_long * xc) as libc::c_double)
            - t1
                * (t2
                    * (t2 * xa as libc::c_double
                        - (2 as libc::c_int as libc::c_long * xb) as libc::c_double)
                    + xc as libc::c_double) + xd as libc::c_double)
            / 8 as libc::c_int as libc::c_double - fx0 as libc::c_double)
            as libc::c_float;
        fy2 = ((t2
            * (t2 * yb as libc::c_double
                - (2 as libc::c_int as libc::c_long * yc) as libc::c_double)
            - t1
                * (t2
                    * (t2 * ya as libc::c_double
                        - (2 as libc::c_int as libc::c_long * yb) as libc::c_double)
                    + yc as libc::c_double) + yd as libc::c_double)
            / 8 as libc::c_int as libc::c_double - fy0 as libc::c_double)
            as libc::c_float;
        fx3 = ((t2
            * (t2
                * ((3 as libc::c_int as libc::c_long * xb) as libc::c_double
                    - t2 * xa as libc::c_double)
                - (3 as libc::c_int as libc::c_long * xc) as libc::c_double)
            + xd as libc::c_double) / 8 as libc::c_int as libc::c_double)
            as libc::c_float;
        fx0 -= fx3;
        fy3 = ((t2
            * (t2
                * ((3 as libc::c_int as libc::c_long * yb) as libc::c_double
                    - t2 * ya as libc::c_double)
                - (3 as libc::c_int as libc::c_long * yc) as libc::c_double)
            + yd as libc::c_double) / 8 as libc::c_int as libc::c_double)
            as libc::c_float;
        fy0 -= fy3;
        x3 = floor(fx3 as libc::c_double + 0.5f64) as libc::c_int;
        y3 = floor(fy3 as libc::c_double + 0.5f64) as libc::c_int;
        if fx0 as libc::c_double != 0.0f64 {
            fx0 = (x0 - x3) as libc::c_float / fx0;
            fx1 *= fx0;
            fx2 *= fx0;
        }
        if fy0 as libc::c_double != 0.0f64 {
            fy0 = (y0 - y3) as libc::c_float / fy0;
            fy1 *= fy0;
            fy2 *= fy0;
        }
        if x0 != x3 || y0 != y3 {
            plotCubicBezierSeg(
                x0,
                y0,
                x0 as libc::c_float + fx1,
                y0 as libc::c_float + fy1,
                x0 as libc::c_float + fx2,
                y0 as libc::c_float + fy2,
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
}
#[no_mangle]
pub unsafe extern "C" fn plotLineAA(
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
) {
    let mut sx: libc::c_int = if x0 < x1 {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    let mut sy: libc::c_int = if y0 < y1 {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    let mut x2: libc::c_int = 0;
    let mut dx: libc::c_long = abs(x1 - x0) as libc::c_long;
    let mut dy: libc::c_long = abs(y1 - y0) as libc::c_long;
    let mut err: libc::c_long = dx * dx + dy * dy;
    let mut e2: libc::c_long = (if err == 0 as libc::c_int as libc::c_long {
        1 as libc::c_int as libc::c_double
    } else {
        0xffff7f as libc::c_long as libc::c_double / sqrt(err as libc::c_double)
    }) as libc::c_long;
    dx *= e2;
    dy *= e2;
    err = dx - dy;
    loop {
        setPixelAA(x0, y0, abs((err - dx + dy) as libc::c_int) >> 16 as libc::c_int);
        e2 = err;
        x2 = x0;
        if 2 as libc::c_int as libc::c_long * e2 >= -dx {
            if x0 == x1 {
                break;
            }
            if e2 + dy < 0xff0000 as libc::c_long {
                setPixelAA(x0, y0 + sy, e2 + dy >> 16 as libc::c_int);
            }
            err -= dy;
            x0 += sx;
        }
        if !(2 as libc::c_int as libc::c_long * e2 <= dy) {
            continue;
        }
        if y0 == y1 {
            break;
        }
        if dx - e2 < 0xff0000 as libc::c_long {
            setPixelAA(x2 + sx, y0, dx - e2 >> 16 as libc::c_int);
        }
        err += dx;
        y0 += sy;
    };
}
