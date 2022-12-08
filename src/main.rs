use macroquad::prelude::*;

struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    vx : f32,
    vy : f32,
    color: Color,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut rects: Vec<Rect> = Vec::new();

    const N: i32 = 1000;

    for i in 0..N {
        let w = rand::gen_range(0, screen_width() as i64 / 4);
        let h = rand::gen_range(0, screen_height() as i64 / 4);
        rects.push(Rect {
            x: rand::gen_range(0, screen_width() as i64 - w) as f32,
            y: rand::gen_range(0, screen_height() as i64 - h) as f32,
            w: w as f32,
            h: h as f32,
            vx: rand::gen_range(-1.0, 1.0),
            vy: rand::gen_range(-1.0, 1.0),
            color: Color::from_rgba(
                rand::gen_range(0, 255),
                rand::gen_range(0, 255),
                rand::gen_range(0, 255),
                255,//rand::gen_range(0, 255),
            ),
        });
    }

    let x: i32 = rand::gen_range(0, 4);

    loop {
        clear_background(WHITE);

        for r in &mut rects {
            draw_rectangle(r.x, r.y, r.w, r.h, r.color);
            r.x += r.vx;
            r.y += r.vy;
            if r.x < 0.0 {
                r.vx = -r.vx;
            }
            if r.y < 0.0 {
                r.vy = -r.vy;
            }
            if r.x + r.w > screen_width() {
                r.vx = -r.vx;
            }
            if r.y + r.h > screen_height() {
                r.vy = -r.vy;
            }
        }

        draw_text(&format!("FPS: {}", macroquad::time::get_fps()), 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
