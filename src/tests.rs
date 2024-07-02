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
    let w = 13;
    let h = 7;

    let start = (0, 0);
    let end = (w as i32 - 1, 0);
    let pivot = (w as i32 / 2, h as i32 - 1);

    let mut buf = vec![BLANK; w * h];

    crate::plot_quad_bezier(start.0, start.1, pivot.0, pivot.1, end.0, end.1, |x, y| {
        let i = y as usize * w + x as usize;
        buf[i] = WHITE;
    });

    lodepng::encode32_file("generated_images/test_quad_bezier.png", &buf, w, h).unwrap();
}
