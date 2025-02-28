// use std::collections::{HashMap, HashSet};
use rustc_hash::{FxHashSet, FxHashMap};
use rand::Rng;
use raylib::prelude::*;
use std::cmp::min;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Grid {
    pub min_x : f64,
    pub max_x : f64,
    pub min_y : f64,
    pub max_y : f64,
    pub spatial_size : (u32, u32),
    pub spatial : FxHashMap<(u32,u32), FxHashSet<usize>>,
    pub fps : f64,
    pub width : f64,
    pub height : f64
}

#[derive(Debug)]
pub struct Object {
    pub x : f64,
    pub y : f64,
    pub dx : f64,
    pub dy : f64,
    pub radius : f64,
    pub color : Color,
    pub spatial : (u32, u32),
    pub id : usize
}

/* Determine the current spatial location for an object.  Note that 
 * the spatial value inside of the object was the previous spatial
 * location prior to the object moving.
 */
fn get_spatial_loc(object : &Object, grid : &Grid) -> (u32, u32) {
    let width = grid.max_x - grid.min_x + 1.0;
    let height = grid.max_y - grid.min_y + 1.0;
    let col= min(grid.spatial_size.0-1, (object.x / (width / grid.spatial_size.0 as f64)) as u32);
    let row = min(grid.spatial_size.1-1, (object.y / (height / grid.spatial_size.1 as f64)) as u32);
    (col,row)
}

/* If the object has moved spatial locations, then the index is removed
 * from the old location and is added to the new location.  The object
 * is updated to reference the new location.
 */
fn update_spatial(object : &mut Object, grid : &mut Grid) {
    let spatial_new = get_spatial_loc(object, grid);
    if object.spatial != spatial_new {
        grid.spatial.get_mut(&object.spatial).unwrap().remove(&object.id);
        grid.spatial.get_mut(&spatial_new).unwrap().insert(object.id);
        object.spatial = spatial_new;
    }
}

/* Create a random object and put it into both objects vector and 
 * the spatial hash based on its starting location.
 */
pub fn create_object(objects : &mut Vec<Object>,
        grid : &mut Grid,
        color : Color, radius : f64) {
    let mut object = Object {
        x : rand::thread_rng().gen_range(grid.min_x+30.0..=grid.max_x-30.0),
        y : rand::thread_rng().gen_range(grid.min_y+30.0..=grid.max_y-30.0),
        dx : rand::thread_rng().gen_range(-10.0..=10.0)*2.0,
        dy : rand::thread_rng().gen_range(-10.0..=10.0)*2.0,
        radius,
        color,
        spatial : (0,0),
        id: objects.len()
    };
    object.spatial = get_spatial_loc(&object, grid);
    grid.spatial.get_mut(&object.spatial).unwrap().insert(object.id);
    objects.push(object);
    
}

/* Helper function to get two object references based on their indicies
 */
fn index_to_objects(objects : &[Object], (index1, index2) : (usize,usize)) -> 
        (&Object, &Object) {
    let o1 = objects.get(index1).unwrap();
    let o2 = objects.get(index2).unwrap();
    (o1,o2)
}

/* Helper function to get two mutable object references based on their indicies.
 */
fn index_to_objects_mut(objects : &mut [Object], (index1, index2) : (usize,usize)) -> 
        (&mut Object, &mut Object) {
    let (o1, o2) = if index1 < index2 {
        let (first, second) = objects.split_at_mut(index2);
        (&mut first[index1], &mut second[0])
    }
    else {
        let (first, second) = objects.split_at_mut(index1);
        (&mut second[0], &mut first[index2])
    };
    (o1,o2)
}

/* Determine if two objects have collided
 */
fn is_collision(o1 : &Object, o2 : &Object) -> bool {
    let dist = ((o1.x - o2.x).powi(2) + (o1.y - o2.y).powi(2)).sqrt();
    dist < (o1.radius + o2.radius) 
}

/* Handle the collision of two objects.  Update the spatial as needed.
 */
fn handle_collision(o1 : &mut Object, o2 : &mut Object, grid : &mut Grid) {
    if !is_collision(o1, o2) {
        return;
    }
    o1.dx *= -1.0 * 0.9;
    o1.dy *= -1.0 * 0.9;
    o2.dx *= -1.0 * 0.9;
    o2.dy *= -1.0 * 0.9;
    loop {
        if !is_collision(o1, o2) {
            break;
        }
        advance_object(o1, (1.0 / grid.fps) / 10.0);
        advance_object(o2, (1.0 / grid.fps) / 10.0);
    }
    update_spatial(o1, grid);
    update_spatial(o2, grid);
}

/* Advance based on velocity and time
 */
pub fn advance_object(object : &mut Object, time : f64) {
    object.x += object.dx * time;
    object.y += object.dy * time;
}

pub fn apply_gravity(object : &mut Object, time : f64) {
    object.dy += 9.8 * time;
}

/* Bounce after hitting the wall
 */
pub fn bounce_object(object : &mut Object, grid : &Grid) {
    if object.x <= (grid.min_x + object.radius) {
        object.dx *= -1.0 * 0.9;
        object.x = grid.min_x + object.radius;
    }
    else if object.x >= (grid.max_x - object.radius) {
        object.dx *= -1.0 * 0.9;
        object.x = grid.max_x - object.radius;
    }
    if object.y <= (grid.min_y + object.radius) {
        object.y = grid.min_y + object.radius;
        object.dy *= -1.0 * 0.9;
    }
    else if object.y >= (grid.max_y - object.radius) {
        object.y = grid.max_y - object.radius;
        object.dy *= -1.0 * 0.9;
    }
}

/* Draw the object on the screen
 */
pub fn draw_object(object : &Object, rl_handle : &mut RaylibDrawHandle, grid : &Grid) {
    let coord_x = ((object.x - grid.min_x) / 
        (grid.max_x - grid.min_x) * grid.width) as i32;
    let coord_y = ((object.y - grid.min_y) / 
        (grid.max_y - grid.min_y) * grid.height) as i32;
    let coord_radius_x = ((object.radius - grid.min_x) / 
        (grid.max_x - grid.min_x) * grid.width) as i32;
    let coord_radius_y = ((object.radius - grid.min_y) / 
        (grid.max_y - grid.min_y) * grid.width) as i32;
    let coord_radius = (coord_radius_x + coord_radius_y) / 2;

    rl_handle.draw_circle(coord_x, coord_y, 
        coord_radius as f32, object.color);
}

/* Run a single frame wihch includes:
 *   1. Clear Screen
 *   2. Advance and Bounce Walls
 *   3. Look for collisions 
 *   4. Handle collisions
 *   5. Draw objects
 */
pub fn frame(objects : &mut [Object], 
             mut rl_handle : RaylibDrawHandle,
             grid : &mut Grid) {
    
    rl_handle.clear_background(Color::RAYWHITE);

    for object in objects.iter_mut() {
        apply_gravity(object, 1.0 / grid.fps);
        advance_object(object, 1.0 / grid.fps);
        bounce_object(object, grid);
        update_spatial(object, grid);
    }

    let mut collisions = Vec::<(usize, usize)>::new();

    for indices in grid.spatial.values() {
        for (i,o1_index) in indices.iter().enumerate() {
            for o2_index in indices.iter().skip(i+1) {
                let (o1, o2) = index_to_objects(objects, (*o1_index, *o2_index));
                if is_collision(o1, o2) {
                    collisions.push((*o1_index, *o2_index));
                }
            }
        }
    }

    for (o1_index, o2_index) in collisions {
        let (o1, o2) = index_to_objects_mut(objects, 
            (o1_index, o2_index));
        handle_collision(o1, o2, grid);
    }

    for object in objects.iter() {
        draw_object(object, &mut rl_handle, grid);
    }

}

/* Create the engine grid and a vector of objects.  Use both of these
 * to execute frames infinitely.
 */
pub fn engine() {

    raylib::logging::set_trace_log(TraceLogLevel::LOG_WARNING);

    // Engine Configuration Values
    let win_width = 1800;
    let win_height = 900;
    let num_objects = 100;
    let fps = 60;
    let min_x = 0.0;
    let min_y = 0.0;
    let max_x = 1000.0;
    let max_y = 1000.0;
    let title = "Physics Engine";
    let object_color = Color::BLUE;
    let object_radius = 5.0;

    let (mut rl, thread) = raylib::init()
                .size(win_width, win_height)
                .title(title)
                .build();    

    let mut spatial = FxHashMap::<(u32,u32),FxHashSet<usize>>::default();
    let spatial_dim = ((num_objects as f64) / (num_objects as f64).log10()).sqrt() as u32;
    let spatial_size = (spatial_dim, spatial_dim);
    for col in 0..spatial_size.0 {
        for row in 0..spatial_size.1 {
            spatial.insert((col,row), FxHashSet::<usize>::default());
        }
    }

    rl.set_target_fps(fps);

    let mut grid = Grid {
        min_x,
        max_x,
        min_y,
        max_y,
        spatial_size,
        spatial,
        fps: fps as f64,
        width: win_width as f64, 
        height: win_height as f64 
    };

    let mut objects = Vec::<Object>::new();


    for _ in 1..=num_objects {
        create_object(&mut objects, &mut grid, 
            object_color, object_radius);
    }

    let mut total: u128 = 0;
    let mut count = 0;
    while !rl.window_should_close() {
        let rl_handle = rl.begin_drawing(&thread);
        let now = Instant::now();
        frame(&mut objects,rl_handle, &mut grid);
        let duration = now.elapsed().as_nanos();
        total += duration;
        count += 1;
    }
    println!("{}",total as f64/count as f64)
}



fn main() {
    engine();
}
