use std::collections::HashSet;

mod octree;

use octree::{
    Octree,
    BoundingBox
};

use jni::{
    EnvUnowned,
    objects::JClass,
    sys::{
        jint,
        jdouble,
        jboolean,
        JNI_TRUE,
        JNI_FALSE,
        jintArray
    },
    errors::{
        ThrowRuntimeExAndDefault,
        Result
    }
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
pub extern "system" fn Java_com_optimization_eco_NativeCollision_queryNative(
    mut env: EnvUnowned,
    _class: JClass,
    min_x: jdouble,
    min_y: jdouble,
    min_z: jdouble,
    max_x: jdouble,
    max_y: jdouble,
    max_z: jdouble,
) -> jintArray {
    let bounding_box: BoundingBox = BoundingBox {
        min: [min_x, min_y, min_z],
        max: [max_x, max_y, max_z],
    };

    let results: HashSet<i32> = {
        let tree = get_octree()
            .lock()
            .expect("Octree mutex poisoned");

        tree.query(&bounding_box)
    };
    let ids: Vec<jint> = results.into_iter().collect();

    env.with_env(|env| -> Result<jintArray> {
        let array = env.new_int_array(ids.len())?;
        array.set_region(env, 0, &ids)?;

        Ok(array.into_raw() as jintArray)
    }).resolve::<ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_optimization_eco_NativeCollision_upsertNative(
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
    let bounds: BoundingBox = BoundingBox {
        min: [min_x, min_y, min_z],
        max: [max_x, max_y, max_z],
    };

    let mut tree = get_octree().lock().unwrap();

    // delete the BoundingBox first if it exists already. This will do nothing if it doesn't exist yet
    tree.delete(id);
    if tree.insert(id, bounds) {
        return JNI_TRUE;
    }
    return JNI_FALSE;
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_optimization_eco_NativeCollision_deleteNative(
    _env: EnvUnowned,
    _class: JClass,
    id: jint
) -> jint {
    let mut tree = get_octree().lock().expect("Octree mutex poisoned");

    tree.delete(id);

    return tree.size();
}