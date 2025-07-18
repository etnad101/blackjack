use macroquad::prelude::*;

pub struct SpriteSheet {
    sheet: Texture2D,
    cols: usize,
    sprite_width: f32,
    sprite_height: f32,
    scale: f32,
}

impl SpriteSheet {
    pub async fn new(
        path: &str,
        sheet_width: f32,
        sheet_height: f32,
        cols: usize,
        rows: usize,
        scale: f32,
    ) -> Result<SpriteSheet, String> {
        match load_texture(path).await {
            Ok(sheet) => Ok(Self {
                sheet,
                cols,
                sprite_width: sheet_width / cols as f32,
                sprite_height: sheet_height / rows as f32,
                scale,
            }),
            Err(e) => Err(e.to_string()),
        }
    }

    fn get_texture_from_index(&self, index: usize) -> DrawTextureParams {
        let row = index / self.cols;
        let col = index % self.cols;

        let source = Rect::new(
            col as f32 * self.sprite_width,
            row as f32 * self.sprite_height,
            self.sprite_width,
            self.sprite_height,
        );

        DrawTextureParams {
            dest_size: Some(Vec2::new(
                self.sprite_width * self.scale,
                self.sprite_height * self.scale,
            )),
            source: Some(source),
            ..Default::default()
        }
    }

    pub fn draw_sprite(&self, index: usize, x: f32, y: f32) {
        let texture_params = self.get_texture_from_index(index);
        draw_texture_ex(&self.sheet, x, y, WHITE, texture_params);
    }
}
