const WHITE: lodepng::RGBA = lodepng::RGBA::new(u8::MAX, u8::MAX, u8::MAX, u8::MAX);
const BLANK: lodepng::RGBA = lodepng::RGBA::new(0, 0, 0, 0);

#[test]
fn test_line() {
    let w = 13;
    let h = 3;
    let start = (0, 0);
    let end = (w as i32 - 1, h as i32 - 1);

    let mut buf = vec![BLANK; w * h];

    crate::plot_line(start.0, start.1, end.0, end.1, |x, y| {
        let i = y as usize * w + x as usize;
        buf[i] = WHITE;
    });

    lodepng::encode32_file("generated_images/test_line.png", &buf, w, h).unwrap();
}

#[test]
fn test_ellipse() {
    let a = 8;
    let b = 4;
    let center = (10, 10);

    let w = 20;
    let h = 20;

    let mut buf = vec![BLANK; w * h];

    crate::plot_ellipse(center.0, center.1, a, b, |x, y| {
        let i = y as usize * w + x as usize;
        buf[i] = WHITE;
    });

    lodepng::encode32_file("generated_images/test_ellipse.png", &buf, w, h).unwrap();
}

#[test]
fn test_circle() {
    let a = 8;
    let center = (10, 10);

    let w = 20;
    let h = 20;

    let mut buf = vec![BLANK; w * h];

    crate::plot_circle(center.0, center.1, a, |x, y| {
        let i = y as usize * w + x as usize;
        buf[i] = WHITE;
    });

    lodepng::encode32_file("generated_images/test_circle.png", &buf, w, h).unwrap();
}

#[test]
fn test_ellipse_rect() {
    let w = 13;
    let h = 7;

    let min = (0, 0);
    let max = (w as i32 - 1, h as i32 - 1);

    let mut buf = vec![BLANK; w * h];

    crate::plot_ellipse_rect(min.0, min.1, max.0, max.1, |x, y| {
        let i = y as usize * w + x as usize;
        buf[i] = WHITE;
    });

    lodepng::encode32_file("generated_images/test_ellipse_rect.png", &buf, w, h).unwrap();
}

#[test]
fn test_quad_bezier() {
    let w = 50;
    let h = 50;

    let start = (0, 0);
    let pivot = (8, 30);
    let end = (16, 0);

    let mut buf = vec![BLANK; w * h];

    crate::plot_quad_bezier(start.0, start.1, pivot.0, pivot.1, end.0, end.1, |x, y| {
        let i = y as usize * w + x as usize;
        buf[i] = WHITE;
    });

    lodepng::encode32_file("generated_images/test_quad_bezier.png", &buf, w, h).unwrap();
}

#[test]
fn test_quad_bezier_passthrough() {
    let w = 50;
    let h = 50;

    let start = (0, 0);
    let pivot = (8, 30);
    let end = (16, 0);

    let mut buf = vec![BLANK; w * h];

    crate::plot_quad_bezier_passthrough(
        start.0,
        start.1,
        pivot.0,
        pivot.1,
        end.0,
        end.1,
        |x, y| {
            let i = y as usize * w + x as usize;
            buf[i] = WHITE;
        },
    );

    lodepng::encode32_file(
        "generated_images/test_quad_bezier_passthrough.png",
        &buf,
        w,
        h,
    )
    .unwrap();
}

#[test]
fn test_quad_spline() {
    let w = 60;
    let h = 50;

    let mut points = vec![(0, 0), (10, 20), (20, 0), (30, 20), (40, 0), (50, 20)];

    let mut buf = vec![BLANK; w * h];

    crate::plot_quad_spline(&mut points, |x, y| {
        let y = y + 1;
        let i = y as usize * w + x as usize;
        buf[i] = WHITE;
    });

    lodepng::encode32_file("generated_images/test_quad_spline.png", &buf, w, h).unwrap();
}

#[test]
fn test_line_aa() {
    let w = 13;
    let h = 10;
    let start = (0, 0);
    let end = (w as i32 - 1, h as i32 - 1);

    let mut buf = vec![BLANK; w * h];

    crate::anti_aliased::plot_line_aa(start.0, start.1, end.0, end.1, |x, y, c| {
        let i = y as usize * w + x as usize;
        let mut col = WHITE;
        col.a = c;
        buf[i] = col;
    });

    lodepng::encode32_file("generated_images/test_line_aa.png", &buf, w, h).unwrap();
}

#[test]
fn test_line_aa_thick() {
    let w = 13 + 10;
    let h = 10 + 10;
    let start = (5, 5);
    let end = (w as i32 - 1 - 5, h as i32 - 1 - 5);

    let mut buf = vec![BLANK; w * h];

    crate::anti_aliased::plot_line_width(start.0, start.1, end.0, end.1, 5.0, |x, y, c| {
        let i = y as usize * w + x as usize;
        let mut col = WHITE;
        col.a = c;
        buf[i] = col;
    });

    lodepng::encode32_file("generated_images/test_line_aa_thick.png", &buf, w, h).unwrap();
}

#[test]
fn test_circle_aa() {
    let a = 8;
    let center = (10, 10);

    let w = 20;
    let h = 20;

    let mut buf = vec![BLANK; w * h];

    crate::anti_aliased::plot_circle_aa(center.0, center.1, a, |x, y, c| {
        let i = y as usize * w + x as usize;
        let mut col = WHITE;
        col.a = c;
        buf[i] = col;
    });

    lodepng::encode32_file("generated_images/test_circle_aa.png", &buf, w, h).unwrap();
}
