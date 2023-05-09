use crate::systems::grid_systems::window_helper::get_window_size;
use crate::EntitiesTimer;
use bevy::prelude::*;

#[derive(Component)]
pub struct StaticGridMap {
    x_min: f32,
    y_min: f32,
    square_size: usize,
    window_height: f32,
    window_width: f32,
    line_thickness: f32,
    line_color: Color,
}

impl StaticGridMap {
    pub fn from_window(
        window_query: Query<&Window>,
        line_thickness: f32,
        square_size: usize,
        line_color: Color,
    ) -> StaticGridMap {
        let (window_height, window_width) = get_window_size(window_query);
        let x_min = -1. * window_width / 2. - line_thickness / 2.;
        let y_min = -1. * window_height / 2. - line_thickness / 2.;
        StaticGridMap {
            x_min,
            y_min,
            square_size,
            window_height,
            window_width,
            line_thickness,
            line_color,
        }
    }
    pub fn draw_grid_static(
        &self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        use bevy::sprite::MaterialMesh2dBundle;

        for width_div in (0..(self.window_width as usize)).step_by(self.square_size) {
            let x_pos = width_div as f32 + self.x_min;
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(
                        shape::Quad::new(Vec2::new(self.line_thickness, self.window_height)).into(),
                    )
                    .into(),
                material: materials.add(self.line_color.into()),
                transform: Transform::from_translation(Vec3::new(x_pos, 0., 0.)),
                ..default()
            });
        }

        for height_div in (0..(self.window_height as usize)).step_by(self.square_size) {
            let y_pos = height_div as f32 + self.y_min;
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(self.window_width, self.line_thickness)).into())
                    .into(),
                material: materials.add(self.line_color.into()),
                transform: Transform::from_translation(Vec3::new(0., y_pos, 0.)),
                ..default()
            });
        }
    }
}

pub fn timed_init_gridmap(
    time: Res<Time>,
    mut timer: ResMut<EntitiesTimer>,
    window_query: Query<&Window>,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    commands: Commands,
) {
    if timer.0.tick(time.delta()).just_finished() {
        init_gridmap(window_query, materials, meshes, commands);
    }
}

fn init_gridmap(
    window_query: Query<&Window>,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    commands: Commands,
) {
    let line_thickness = 5.;
    let line_color = Color::BLACK;
    let square_size: usize = 50;
    let new_grid =
        StaticGridMap::from_window(window_query, line_thickness, square_size, line_color);
    new_grid.draw_grid_static(commands, meshes, materials);
}
