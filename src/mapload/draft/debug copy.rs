use std::collections::BTreeMap;

use bevy::prelude::*;

pub(super) fn draw_level_gizmos(
    mut gizmos: Gizmos,
    levels: Res<Assets<Level>>,
    level_handle: Option<Res<CurrentLevelHandle>>,
) {
    let Some(level_handle) = level_handle else {
        return;
    };
    let Some(level) = levels.get(&level_handle.0) else {
        // 还没加载完
        return;
    };

    for obj in &level.objects {
        let (x, y) = obj.pos;
        let (w, h) = obj.size;

        // 这里假设 pos 是左上角，如果你用中心，就不要 + w/2, h/2
        let center = Vec3::new(x + w / 2.0, y + h / 2.0, obj.z);
        let size = Vec2::new(w, h);

        // 绿色框：物体可见区域
        gizmos.rect(center, Quat::IDENTITY, size, Color::GREEN);

        // 红色框：碰撞区（目前 == 可见区域；以后如果你有单独的 collision rect，就改成对应位置/尺寸）
        gizmos.rect(center, Quat::IDENTITY, size, Color::RED);
    }
}

pub(super) fn spawn_object_labels(
    mut commands: Commands,
    levels: Res<Assets<Level>>,
    level_handle: Option<Res<CurrentLevelHandle>>,
    mut spawned: ResMut<DebugLabelsSpawned>,
    asset_server: Res<AssetServer>,
) {
    // 只生成一次
    if spawned.0 {
        return;
    }

    let Some(level_handle) = level_handle else {
        return;
    };
    let Some(level) = levels.get(&level_handle.0) else {
        // 关卡还没加载好
        return;
    };

    // 随便选一个字体，你项目里有啥就改成啥
    let font = asset_server.load("fonts/NotoSans-VariableFont_wdth,wght.ttf");

    for obj in &level.objects {
        let (x, y) = obj.pos;
        let (w, h) = obj.size;
        let label_pos = Vec3::new(x + w / 2.0, y + h / 2.0 + 20.0, obj.z + 10.0);

        commands.spawn(Text2dBundle {
            text: Text::from_section(
                obj.id.clone(),
                TextStyle {
                    font: font.clone(),
                    font_size: 14.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_translation(label_pos),
            ..Default::default()
        });
    }

    spawned.0 = true;
}
