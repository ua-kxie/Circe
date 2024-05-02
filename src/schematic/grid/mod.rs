use bevy::{
    input::mouse::MouseWheel, math::I16Vec2, prelude::*, render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages}, window::PrimaryWindow
};
use euclid::{Box2D, Point2D, Vector2D};

use crate::types::{CanvasSpace, SSPoint, SchematicSpace};

use super::{ui::{self, GridMaterial}, NewVisibleCanvasAABB, SchematicCameraMarker, VisibleCanvasAABB};


pub struct Grid;

impl Plugin for Grid {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<GridMaterial>::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update,main);
    }
}


#[derive(Component)]
struct GridMarker;

#[derive(Bundle)]
struct GridBundle {
    mesh: MaterialMeshBundle<ui::GridMaterial>,
    grid: GridMarker,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut grid_materials: ResMut<Assets<ui::GridMaterial>>,
) {
    // grid
    commands.spawn(
        GridBundle{
            mesh: MaterialMeshBundle{
                material: grid_materials.add(ui::GridMaterial{color: Color::WHITE}),
                mesh: meshes
                .add(
                    Mesh::new(
                        PrimitiveTopology::PointList,
                        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
                    ),
                )
                .into(),
                ..default()
            },
            grid: GridMarker
        }
    );
    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes
    //         .add(
    //             Mesh::new(
    //                 PrimitiveTopology::PointList,
    //                 RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    //             )
    //             .with_inserted_attribute(
    //                 Mesh::ATTRIBUTE_POSITION,
    //                 vec![
    //                     Vec3::from([2.0, 2.0, 0.0]),
    //                     Vec3::from([-2.0, -2.0, 0.0]),
    //                     Vec3::from([2.0, -2.0, 0.0]),
    //                     ],
    //             ),
    //             // Mesh::from(Cuboid::default())
    //         )
    //         .into(),
    //     material: grid_materials.add(ui::GridMaterial{color: Color::WHITE}),
    //     ..default()
    // });
}

// place grid dots according to visible canvas aabb
fn main(
    mut g: Query<(Entity, &mut Handle<Mesh>), With<GridMarker>>,
    mut e_new_viewport: EventReader<NewVisibleCanvasAABB>,
    visible_canvas_aabb: ResMut<VisibleCanvasAABB>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    if let Some(_) = e_new_viewport.read().last() {
        let aabb = visible_canvas_aabb.0.unwrap();
        let veclen = (aabb.width() * aabb.height()).try_into().unwrap();
        let mut gridvec = vec![Vec3::splat(0.0); veclen];
        for x in 0..aabb.width() {
            for y in 0..aabb.height() {
                gridvec[(x * aabb.height() + y) as usize] = Vec3::from_array([(aabb.min.x + x) as f32, (aabb.min.y + y) as f32, 0.0])
            }
        }

        let grid = g.get_single_mut().unwrap();
        let gridmesh = meshes.get_mut(grid.1.id()).unwrap();
        gridmesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            gridvec,
        );
        if let Some(aabb) = gridmesh.compute_aabb() {
            commands.entity(grid.0).insert(aabb);
        }
    }
}