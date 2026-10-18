mod octree;
use octree::{
    Octree,
    BoundingBox,
    Entity
};

use jni::{
    EnvUnowned,
    objects::JClass,
    sys::{
        jint,
        jdouble,
        jboolean,
        JNI_TRUE,
        JNI_FALSE
    },
};
use std::sync::{
    Mutex, OnceLock
};

const BOUNDS: BoundingBox = BoundingBox { min: [-1024.0, -1024.0, -1024.0], max: [1024.0, 1024.0, 1024.0]};
static OCTREE: OnceLock<Mutex<Octree>> = OnceLock::new();

fn get_octree() -> &'static Mutex<Octree> {
    OCTREE.get_or_init(|| Mutex::new(
        Octree::new(BOUNDS)
    ))
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_optimization_eco_NativeCollision_clearTree(
    _env: EnvUnowned,
    _class: JClass,
) {
    let mut tree = get_octree()
        .lock()
        .expect("Octree mutex poisoned");

    
    *tree = Octree::new(BOUNDS);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_optimization_eco_NativeCollision_insertEntity(
    _env: EnvUnowned,
    _class: JClass,
    id: jint,
    min_x: jdouble,
    min_y: jdouble,
    min_z: jdouble,
    max_x: jdouble,
    max_y: jdouble,
    max_z: jdouble,
) -> jboolean {
    let id = id as i32;

    let bounds = BoundingBox {
        min: [min_x, min_y, min_z],
        max: [max_x, max_y, max_z],
    };

    let mut tree = get_octree()
        .lock()
        .expect("Octree mutex poisoned");

    let entity = Entity {
        id: id,
        bounding_box: bounds
    };

    if tree.insert(entity) {
        return JNI_TRUE;
    }
    return JNI_FALSE
}