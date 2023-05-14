use crate::systems::grid_systems::window_helper::get_window_size;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

// To-Do:
//  - Add camera parameter for FlexGrid::draw so it draws to the correct camera
//  - Add non-consuming .iter() method for FlexGrid
//      - change build_2d to use .iter() method on self

// A bundle composed of the components require for a grid to be displayed or
// transformed.
#[derive(Component)]
pub struct FlexGrid {
    pub flex_points_2d_vec: FlexGrid2dPointVec,
    pub square_size: SquareSize,
    pub window_height: WindowHeight,
    pub window_width: WindowWidth,
    pub line_thickness: LineThickness,
    pub line_color: LineColor,
    pub entity_ids: EntityIds,
}

pub fn setup_flex_grid(
    window_query: Query<&Window>,
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    FlexGrid::build_2d(
        window_query,
        crate::GRID_SQUARE_SIZE.into(),
        crate::GRID_LINE_THICKNESS.into(),
        crate::GRID_LINE_COLOR.into(),
        crate::GRID_Z_POS,
    )
    .draw(commands, meshes, materials);
}

impl FlexGrid {
    pub fn new() -> Self {
        FlexGrid {
            flex_points_2d_vec: FlexGrid2dPointVec::new(),
            square_size: SquareSize::from(0),
            window_height: WindowHeight::from(0.),
            window_width: WindowWidth::from(0.),
            line_thickness: LineThickness::from(0.),
            line_color: LineColor::from(Color::RED),
            entity_ids: EntityIds(None),
        }
    }
    // Build a new FlexGrid for a given window size so it can be drawn.
    pub fn build_2d(
        window_query: Query<&Window>,
        square_size: SquareSize,
        line_thickness: LineThickness,
        line_color: LineColor,
        z_pos: f32,
    ) -> FlexGrid {
        let (height, width) = get_window_size(window_query);

        let x_min: f32 = -1. - width / 2. - square_size.0 as f32 / 2.;
        let y_min: f32 = -1. - height / 2. - square_size.0 as f32 / 2.;

        let num_y_points: usize = height as usize / square_size.0 + 1;
        let num_x_points: usize = width as usize / square_size.0 + 1;

        let flex_points_2d_vec = (0..=num_y_points)
            .into_iter()
            .map(|y_point| y_min + (square_size.0 * y_point) as f32)
            .map(|y_pos| {
                (0..=num_x_points)
                    .into_iter()
                    .map(|x_point| x_min + (square_size.0 * x_point) as f32)
                    .map(|x_pos| (x_pos, y_pos, z_pos).into())
                    .collect::<FlexGridPointVec>()
            })
            .collect::<FlexGrid2dPointVec>();

        FlexGrid {
            flex_points_2d_vec,
            square_size,
            window_height: height.into(),
            window_width: width.into(),
            line_thickness,
            line_color,
            entity_ids: EntityIds(None),
        }
    }

    // Draws the grid into the window by spawning a material mesh at grid points.
    // The material mesh is spawned onto the center of the point, and the mesh extends
    // equally in the y and -y, and x and -x directions.
    // It stores the entity ids once drawn, so they can be referenced and transformed
    // by other commands (yet to be written) instead of redrawn each time.
    // Currently consumes the flexgrid.
    pub fn draw(
        &mut self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let mut into_entity_ids: Vec<Entity> = Vec::new();
        for vec_2d in self.flex_points_2d_vec.0.iter() {
            for flex_point in vec_2d.0.iter() {
                // Lines in the x dimension, from each flex_point
                let x_id = commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(
                                shape::Quad::new(Vec2::new(
                                    self.line_thickness.0,
                                    self.square_size.0 as f32,
                                ))
                                .into(),
                            )
                            .into(),
                        material: materials.add(self.line_color.0.into()),
                        transform: Transform::from_translation(Vec3::new(
                            flex_point.x_pos.0,
                            flex_point.y_pos.0,
                            flex_point.z_pos.0,
                        )),
                        ..default()
                    })
                    .id();

                // Lines in the y dimension, from each flex_point
                let y_id = commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(
                                shape::Quad::new(Vec2::new(
                                    self.square_size.0 as f32,
                                    self.line_thickness.0,
                                ))
                                .into(),
                            )
                            .into(),
                        material: materials.add(self.line_color.0.into()),
                        transform: Transform::from_translation(Vec3::new(
                            flex_point.x_pos.0,
                            flex_point.y_pos.0,
                            flex_point.z_pos.0,
                        )),
                        ..default()
                    })
                    .id();
                into_entity_ids.push(x_id);
                into_entity_ids.push(y_id);
            }
        }
        self.entity_ids = Some(into_entity_ids).into();
    }

    fn rows_len(&self) -> usize {
        self.flex_points_2d_vec.0.len()
    }

    fn cols_len(&self) -> usize {
        self.flex_points_2d_vec.0[0].0.len()
    }

    // Builds the grid which the object and characters will occupy,
    // the squares inside the grid.
    // This is done by adding square_size/2 to the x and y positions
    // for everything other than the top row and right row.
    pub fn build_square_grid(&self) -> Grid2dPointVec {
        let square_size = self.square_size.0;
        let flex_grid_2d: FlexGrid2dPointVec = self
            .flex_points_2d_vec
            .0
            .clone()
            .into_iter()
            .skip(1)
            .map(|row| {
                row.0
                    .into_iter()
                    .skip(1)
                    .map(|flex_pos| {
                        FlexGridPoint::from_pos(
                            flex_pos.x_pos + (square_size / 2) as f32,
                            flex_pos.y_pos + (square_size / 2) as f32,
                            flex_pos.z_pos,
                        )
                    })
                    .collect()
            })
            .collect();
        Grid2dPointVec {
            flex_grid_2d,
            entity_ids: EntityIds(None),
        }
    }
}

#[derive(Component)]
pub struct Grid2dPointVec {
    flex_grid_2d: FlexGrid2dPointVec,
    entity_ids: EntityIds,
}

impl Grid2dPointVec {
    // Draw a circle on the point, meant to be at the center of the circle of the
    // flexpoint grid.
    pub fn draw_indicators(
        &self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        indicator_size: f32,
        indicator_color: Color,
    ) {
        for vec_2d in self.flex_grid_2d.0.iter() {
            for flex_point in vec_2d.0.iter() {
                println!(
                    "x_pos: {}, y_pos: {}, z_pos: {}",
                    flex_point.x_pos.0, flex_point.y_pos.0, flex_point.z_pos.0
                );
                commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(indicator_size).into()).into(),
                    material: materials.add(indicator_color.into()),
                    transform: Transform::from_translation(Vec3::new(
                        flex_point.x_pos.0,
                        flex_point.y_pos.0,
                        flex_point.z_pos.0,
                    )),
                    ..default()
                });
            }
        }
    }
}

// 2d vec of x,y,z points
#[derive(Component, Clone)]
pub struct FlexGrid2dPointVec(Vec<FlexGridPointVec>);

impl FlexGrid2dPointVec {
    fn new() -> Self {
        FlexGrid2dPointVec(Vec::new())
    }

    fn add(&mut self, elem: FlexGridPointVec) {
        self.0.push(elem);
    }
}

impl From<Vec<FlexGridPointVec>> for FlexGrid2dPointVec {
    fn from(item: Vec<FlexGridPointVec>) -> Self {
        FlexGrid2dPointVec(item)
    }
}

impl FromIterator<FlexGridPointVec> for FlexGrid2dPointVec {
    fn from_iter<I: IntoIterator<Item = FlexGridPointVec>>(iter: I) -> Self {
        let mut c = FlexGrid2dPointVec::new();

        for i in iter {
            c.add(i);
        }
        c
    }
}

impl IntoIterator for FlexGrid2dPointVec {
    type Item = FlexGridPointVec;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// 1d vec of x,y,z points
#[derive(Component, Clone)]
pub struct FlexGridPointVec(Vec<FlexGridPoint>);

impl FlexGridPointVec {
    fn new() -> Self {
        FlexGridPointVec(Vec::new())
    }

    fn add(&mut self, elem: FlexGridPoint) {
        self.0.push(elem);
    }
}

impl FromIterator<FlexGridPoint> for FlexGridPointVec {
    fn from_iter<I: IntoIterator<Item = FlexGridPoint>>(iter: I) -> Self {
        let mut c = FlexGridPointVec::new();

        for i in iter {
            c.add(i);
        }
        c
    }
}

impl IntoIterator for FlexGridPointVec {
    type Item = FlexGridPoint;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<Vec<FlexGridPoint>> for FlexGridPointVec {
    fn from(item: Vec<FlexGridPoint>) -> Self {
        FlexGridPointVec(item)
    }
}

// x,y,z position of an object
#[derive(Bundle, Clone)]
pub struct FlexGridPoint {
    pub x_pos: Xpos,
    pub y_pos: Ypos,
    pub z_pos: Zpos,
}

impl FlexGridPoint {
    pub fn from_pos(x_pos: Xpos, y_pos: Ypos, z_pos: Zpos) -> FlexGridPoint {
        FlexGridPoint {
            x_pos,
            y_pos,
            z_pos,
        }
    }
}

impl From<(f32, f32, f32)> for FlexGridPoint {
    fn from(item: (f32, f32, f32)) -> Self {
        FlexGridPoint {
            x_pos: item.0.into(),
            y_pos: item.1.into(),
            z_pos: item.2.into(),
        }
    }
}

impl From<[f32; 3]> for FlexGridPoint {
    fn from(item: [f32; 3]) -> Self {
        FlexGridPoint {
            x_pos: item[0].into(),
            y_pos: item[1].into(),
            z_pos: item[2].into(),
        }
    }
}

// x-position of an object
#[derive(Component, Clone, Copy)]
pub struct Xpos(f32);

impl From<f32> for Xpos {
    fn from(item: f32) -> Self {
        Xpos(item)
    }
}

use std::ops::Add;
impl Add<f32> for Xpos {
    type Output = Xpos;

    fn add(self, other: f32) -> Xpos {
        (self.0 + other).into()
    }
}

// y-position of an object
#[derive(Component, Clone, Copy)]
pub struct Ypos(f32);

impl From<f32> for Ypos {
    fn from(item: f32) -> Self {
        Ypos(item)
    }
}

impl Add<f32> for Ypos {
    type Output = Ypos;

    fn add(self, other: f32) -> Ypos {
        (self.0 + other).into()
    }
}

// z-position of an object
#[derive(Component, Clone, Copy)]
pub struct Zpos(f32);

impl From<f32> for Zpos {
    fn from(item: f32) -> Self {
        Zpos(item)
    }
}

// The length of the side of an unflexed grid square, in pixels
#[derive(Component)]
pub struct SquareSize(usize);

impl From<usize> for SquareSize {
    fn from(item: usize) -> Self {
        SquareSize(item)
    }
}

// The height of the window in which the grid is rendered, in pixels
#[derive(Component)]
pub struct WindowHeight(f32);

impl From<f32> for WindowHeight {
    fn from(item: f32) -> Self {
        WindowHeight(item)
    }
}

// The width of the window in which the grid is rendered, in pixels
#[derive(Component)]
pub struct WindowWidth(f32);

impl From<f32> for WindowWidth {
    fn from(item: f32) -> Self {
        WindowWidth(item)
    }
}

// The thickness of the grid lines, in pixels
#[derive(Component)]
pub struct LineThickness(f32);

impl From<f32> for LineThickness {
    fn from(item: f32) -> Self {
        LineThickness(item)
    }
}

// The color of the lines, as a Color
#[derive(Component)]
pub struct LineColor(Color);

impl From<Color> for LineColor {
    fn from(item: Color) -> Self {
        LineColor(item)
    }
}

// A list of entity ids, here used to store the ids of the spawned grid bars, but
// also can be used elsewhere to store a list of entities for further use.
#[derive(Component)]
pub struct EntityIds(Option<Vec<Entity>>);

impl From<Option<Vec<Entity>>> for EntityIds {
    fn from(item: Option<Vec<Entity>>) -> Self {
        EntityIds(item)
    }
}
