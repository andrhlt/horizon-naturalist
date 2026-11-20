fn elephant_inventory_background(
    game: &Game<'_>,
    block: &str,
    batch: &mut HudBatch,
    origin: [f32; 2],
) {
    use horizon_client::{mobs::Mob, text::Text};
    if block != self::naturalist_elephant_inventory::MENU {
        return;
    }
    let Some((id, chested)) = game.naturalist_elephant_inventory() else {
        return;
    };
    let Some(mob) = game.mobs.iter().find(|candidate| candidate.id == id) else {
        return;
    };
    let texture = "naturalist:gui/container/elephant";
    let [x, y] = origin;
    game.native_sprite(
        batch,
        texture,
        [0.0, 0.0, 176.0, 204.0],
        [x, y, 176.0, 204.0],
    );
    if chested {
        game.native_sprite(
            batch,
            texture,
            [176.0, 17.0, 90.0, 90.0],
            [x + 79.0, y + 17.0, 90.0, 90.0],
        );
    }
    draw_preview(game, mob, batch, [x + 8.0, y + 18.0, x + 77.0, y + 87.0]);
    let atlas = &game.gui.atlas;
    let (key, fallback) = if mob.kind.id == "naturalist:mammoth" {
        ("entity.naturalist.mammoth", "Mammoth")
    } else {
        ("entity.naturalist.elephant", "Elephant")
    };
    let title = mob
        .custom_name_text
        .clone()
        .or_else(|| mob.custom_name.clone().map(Text::literal))
        .unwrap_or_else(|| Text::literal(game.translate(key, fallback)));
    super::screen_info::name_container(|| title.plain());
    let color = rgba(64, 64, 64, 255);
    batch.text_styled(atlas, &title, x + 8.0, y + 6.0, color, false);
    batch.text(
        atlas,
        &game.translate("container.inventory", "Inventory"),
        x + 8.0,
        y + 110.0,
        color,
        false,
    );

    fn draw_preview(game: &Game<'_>, mob: &Mob, batch: &mut HudBatch, [x0, y0, x1, y1]: [f32; 4]) {
        use glam::{DVec3, Mat4, Vec3};
        let (cx, cy) = ((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let (x_angle, y_angle) = (
            ((cx - game.cursor.0) / 40.0).atan(),
            ((cy - game.cursor.1) / 40.0).atan(),
        );
        let mut model = mob.clone();
        model.previous = model.position;
        model.body_yaw = 180.0 + x_angle * 20.0;
        model.previous_body_yaw = model.body_yaw;
        model.head_yaw = model.body_yaw + x_angle * 20.0;
        model.previous_head_yaw = model.head_yaw;
        model.head_pitch = -y_angle * 20.0;
        model.previous_head_pitch = model.head_pitch;
        model.hurt_time = 0;
        model.death_time = 0;
        let camera = DVec3::from_array(model.position.components());
        let mut vertices = Vec::new();
        horizon_client::mobs::cem_anim::in_gui(|| {
            game.mod_mob_geometry(&model, 1.0, camera, 240 | (240 << 8), &mut vertices);
        });
        let projection = Mat4::from_translation(Vec3::new(cx, cy, 0.0))
            * Mat4::from_scale(Vec3::new(16.0, 16.0, -16.0))
            * Mat4::from_translation(Vec3::new(0.0, model.height() as f32 * 0.5 + 0.0625, 0.0))
            * Mat4::from_rotation_z(std::f32::consts::PI)
            * Mat4::from_rotation_x((y_angle * 20.0).to_radians());
        let mut quads = Vec::new();
        for face in vertices.as_chunks::<4>().0 {
            let positions =
                face.map(|vertex| projection.transform_point3(Vec3::from_array(vertex.pos)));
            let normal = (positions[1] - positions[0]).cross(positions[2] - positions[0]);
            if normal.z >= 0.0 {
                continue;
            }
            let normal = normal.normalize_or_zero();
            let shade = (0.55 + 0.45 * -normal.z - 0.15 * normal.y).clamp(0.4, 1.0);
            let color = face[0].color;
            let channel = |shift: u32| {
                (((f32::from(((color >> shift) & 255) as u8)) * shade) as u32) << shift
            };
            let color = (color & 0xFF00_0000) | channel(0) | channel(8) | channel(16);
            let depth = positions.iter().map(|point| point.z).sum::<f32>() * 0.25;
            quads.push((
                depth,
                positions.map(|point| [point.x, point.y]),
                face.map(|vertex| vertex.uv),
                color,
            ));
        }
        quads.sort_by(|left, right| left.0.total_cmp(&right.0));
        let start = batch.vertices.len();
        for (_, positions, uv, color) in quads {
            batch.quad(positions, uv, color);
        }
        batch.clip_since(start, x0, y0, x1, y1);
    }
}
