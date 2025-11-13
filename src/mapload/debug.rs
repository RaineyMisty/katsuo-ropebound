use std::fmt;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub(super) struct LevelObjectLabel;

use super::resource::{LevelRes, Kind};

// 根据 Kind 给不同颜色
fn color_for_kind(kind: &Kind) -> Color {
    match kind {
        Kind::Platform    => Color::srgb(1.0, 0.0, 0.0),   // 红
        Kind::Coin        => Color::srgb(1.0, 1.0, 0.0),   // 黄
        Kind::JumpPad     => Color::srgb(0.0, 1.0, 0.0),   // 绿
        Kind::Spikes      => Color::srgb(1.0, 0.5, 0.0),   // 橙
        Kind::Door        => Color::srgb(0.0, 0.0, 1.0),   // 蓝
        Kind::Switch      => Color::srgb(0.0, 1.0, 1.0),   // 青
        Kind::Enemy       => Color::srgb(0.6, 0.0, 0.6),   // 紫
        Kind::SpawnPoint  => Color::srgb(0.5, 1.0, 0.0),   // 黄绿
    }
}

// 给 Kind 实现一个简易 Display，方便生成默认名字
impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Platform => write!(f, "Platform"),
            Kind::Coin => write!(f, "Coin"),
            Kind::JumpPad => write!(f, "JumpPad"),
            Kind::Spikes => write!(f, "Spikes"),
            Kind::Door => write!(f, "Door"),
            Kind::Switch => write!(f, "Switch"),
            Kind::Enemy => write!(f, "Enemy"),
            Kind::SpawnPoint => write!(f, "Spawn"),
        }
    }
}

/// 只在 LevelRes 刚刚出现时跑一次：为每个 Object 生成一个 Text2d 作为 id 名字
pub(super) fn spawn_level_labels(
    mut commands: Commands,
    level_res: Res<LevelRes>,
    asset_server: Res<AssetServer>,
    // 如果你的世界里还没有 2D 摄像机，可以顺手在别处加一个 Camera2dBundle
) {
    let level = &level_res.0;

    // 改成你自己项目里的字体路径
    let font = asset_server.load("fonts/NotoSans-VariableFont_wdth,wght.ttf");

    for (i, obj) in level.objects.iter().enumerate() {
        let pos_px = obj.pos;

        // 如果你想把 pixel 转 meter，可以改成：
        // let x = x_px / level.pixels_per_meter;
        // let y = y_px / level.pixels_per_meter;
        let x = pos_px.x;
        let y = pos_px.y;

        // label 文本优先用 object.id，否则用 Kind + index
        let name = obj
            .id
            .clone()
            .unwrap_or_else(|| format!("{}#{i}", obj.kind));

        commands.spawn((
            // 显示文本内容（新 API）
            Text2d::new(name),
            // 字体和字号
            TextFont {
                font: font.clone(),
                font_size: 14.0,
                ..default()
            },
            // 居中对齐（其实对单行影响不大，但写上没坏处）
            TextLayout::default(),
            // 世界空间位置
            Transform::from_translation(Vec3::new(x, y + 16.0, 10.0)),
            LevelObjectLabel,
        ));
    }
}

/// 每一帧画 gizmo 框（不占实体的那种 debug 线框）
/// - chunks 用白色框
/// - objects 用按 Kind 上色的框
pub(super) fn draw_level_gizmos(mut gizmos: Gizmos, level_res: Option<Res<LevelRes>>) {
    let Some(level_res) = level_res else { return };
    let level = &level_res.0;

    // 1) 画 chunks 的大框
    for chunk in &level.chunks {
        let (x, y, w, h) = chunk.rect;
        let x = x as f32;
        let y = y as f32;
        let w = w as f32;
        let h = h as f32;

        let center = Vec2::new(x + w / 2.0, y + h / 2.0);
        let full_size = Vec2::new(w, h);

        // Bevy 0.16 的 API：rect_2d(center, half_size, color)
        gizmos.rect_2d(center, full_size, Color::srgb(1.0, 1.0, 1.0));
    }

    // 画每个 Object 的框（彩色）
    for obj in &level.objects {
        let pos_px = obj.pos;
        let size_px = obj.size;

        let center = pos_px;
        let full_size = size_px;

        let color = color_for_kind(&obj.kind);

        gizmos.rect_2d(center, full_size, color);
    }
}