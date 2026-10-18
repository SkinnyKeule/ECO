use std::{collections::{HashMap, HashSet}, mem::take};

const MAX_ENTRIES: usize = 10;
const MAX_DEPTH: usize = 8;


#[derive(Debug)]
pub struct Octree {
    pub entities: HashMap<i32, BoundingBox>,
    pub root: OctreeNode
}

#[derive(Debug)]
pub struct OctreeNode {
    children: Option<[Box<OctreeNode>; 8]>, // 8 children for each non-leaf node allocated on the heap by Box
    bounding_box: BoundingBox,
    entities: HashSet<i32>,
    has_been_split: bool,
    depth: usize
}

#[derive(Debug, PartialEq)]
pub struct BoundingBox {
    pub min: [f64; 3], // min_x, min_y, min_z
    pub max: [f64; 3] // max_x, max_y, max_z
}


impl Octree {
    pub fn new(bounding_box: BoundingBox) -> Self {
        Self {
            entities: HashMap::new(),
            // root always has depth 0
            root: OctreeNode::new(bounding_box, 0),
        }
    }

    pub fn insert(&mut self, entity_id: i32, bounding_box: BoundingBox) -> bool {
        // only insert entities that actually fit into the tree (for testing for now)
        if !self.root.overlaps(&bounding_box) {
            return false;
        }
        // insert the bounding box into the map first so all nodes can have access to it
        self.entities.insert(entity_id, bounding_box);

        self.root.insert(entity_id, &self.entities)
    }

    pub fn query(&self, bounding_box: &BoundingBox) -> HashSet<i32> {
        self.root.query(bounding_box, &self.entities)
    }

    pub fn delete(&mut self, entity_id: i32) {
        // removes the bounding box from the map and passes it to the node for deletion handling
        if let Some(query_box) = self.entities.remove(&entity_id) {
            self.root.delete(&query_box, entity_id);
        }
        // bounding box dropped here
    }

    #[allow(dead_code)]
    pub fn size(&self) -> i32 {
        self.root.size()
    }
}

impl OctreeNode {
    pub fn new(bounding_box: BoundingBox, depth: usize) -> Self {
        Self {
            children: None,
            bounding_box: bounding_box,
            entities: HashSet::new(),
            has_been_split: false,
            depth: depth
        }
    }

    pub fn overlaps(&self, other: &BoundingBox) -> bool {
        // overlap calculation logic handled in the fn eq impl of BoundingBox
        self.bounding_box.overlaps(other)
    }

    // pub fn covers(&self, other: &BoundingBox) -> bool {
    //     self.bounding_box.covers(other)
    // }

    pub fn insert(&mut self, entity_id: i32, entity_bounds: &HashMap<i32, BoundingBox>) -> bool {
        // unwrap is fine here as the Octree::insert always inserts into the entity bounds first
        if !self.overlaps(entity_bounds.get(&entity_id).unwrap()) {
            return false;
        }

        if self.depth >= MAX_DEPTH {
           self.entities.insert(entity_id);
            return true;
        }

        if !self.has_been_split && self.entities.len() < MAX_ENTRIES {
            self.entities.insert(entity_id);
            return true;
        }
        self.split(entity_bounds);

        let mut inserted = false;

        for child in self.children.as_mut().unwrap().iter_mut() {
            if child.insert(entity_id, entity_bounds) {
                inserted = true;
            }
        }

        inserted
    }

    pub fn query(&self, query_box: &BoundingBox, entity_bounds: &HashMap<i32, BoundingBox>) -> HashSet<i32> {
        let mut ret_val: HashSet<i32> = HashSet::new();

        if !self.bounding_box.overlaps(query_box) {
            return ret_val;
        }
        
        if self.has_been_split { // if the node has been split into smaller boxes we check those first
            for child in self.children.as_ref().unwrap() {
                ret_val.extend(child.query(query_box, entity_bounds));
            }
        } else { // if the node has not been split the entity has to be in here
            for &entity_id in &self.entities {
                if let Some(entity) = entity_bounds.get(&entity_id) {
                    if entity.overlaps(query_box) {
                        ret_val.insert(entity_id);
                    }
                }
            }
        }
        ret_val
    }

    // might have to improce this to explicitily 
    fn delete(&mut self, query_box: &BoundingBox, entity_id: i32) {
        if !self.overlaps(query_box) {
            return;
        }

        if self.has_been_split {
            for child in self.children.as_mut().unwrap().iter_mut() {
                child.delete(query_box, entity_id)
            }
        }

        self.entities.remove(&entity_id);
    }

    pub fn split(&mut self, entity_bounds: &HashMap<i32, BoundingBox>) {
        if self.has_been_split || self.depth >= MAX_DEPTH { // node has already been split, just a failsafe
            return;
        }
        self.has_been_split = true;

        let [min_x, min_y, min_z] = self.bounding_box.min;
        let [max_x, max_y, max_z] = self.bounding_box.max;

        let mid_x: f64 = (min_x + max_x) / 2.0; // (min_x + max_x) / 2
        let mid_y: f64 = (min_y + max_y) / 2.0; // (min_y + max_y) / 2
        let mid_z: f64 = (min_z + max_z) / 2.0; // (min_z + max_z) / 2

        let children: [Box<OctreeNode>; 8] = [
            // Child 0
            Box::new(OctreeNode::new(
                BoundingBox { min: [min_x, min_y, min_z], max: [mid_x, mid_y, mid_z] },
                self.depth + 1
            )),
            // Child 1
            Box::new(OctreeNode::new(
                BoundingBox { min: [mid_x, min_y, min_z], max: [max_x, mid_y, mid_z] },
                self.depth + 1
            )),
            // Child 2
            Box::new(OctreeNode::new(
                BoundingBox { min: [min_x, mid_y, min_z], max: [mid_x, max_y, mid_z] },
                self.depth + 1
            )),
            // Child 3
            Box::new(OctreeNode::new(
                BoundingBox { min: [mid_x, mid_y, min_z], max: [max_x, max_y, mid_z] },
                self.depth + 1
            )),
            // Child 4
            Box::new(OctreeNode::new(
                BoundingBox { min: [min_x, min_y, mid_z], max: [mid_x, mid_y, max_z] },
                self.depth + 1
            )),
            // Child 5
            Box::new(OctreeNode::new(
                BoundingBox { min: [mid_x, min_y, mid_z], max: [max_x, mid_y, max_z] },
                self.depth + 1
            )),
            // Child 6
            Box::new(OctreeNode::new(
                BoundingBox { min: [min_x, mid_y, mid_z], max: [mid_x, max_y, max_z] },
                self.depth + 1
            )),
            // Child 7
            Box::new(OctreeNode::new(
                BoundingBox { min: [mid_x, mid_y, mid_z], max: [max_x, max_y, max_z] },
                self.depth + 1
            ))
        ];

        self.children = Some(children);

        // takes the current entities out of the entities set and reinserts into the children
        for entity in take( &mut self.entities ) {
            for child in self.children.as_mut().unwrap().iter_mut() {
                child.insert(entity, entity_bounds);
            }
        }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> i32 {
        let mut size: i32 = 0;
        
        // if the node has children it has been split before
        match &self.children {
            Some(children) => {
                for child in children {
                    size += child.size();
                };
            },

            None => {
                // add the amount of all leaf nodes
                size = self.entities.len() as i32;
            }
        }

        size
    }
}

impl BoundingBox {
    fn overlaps(&self, other: &BoundingBox) -> bool {
        // checks for overlap on the x axis (index 0 = x)
        (self.min[0] < other.max[0] && other.min[0] < self.max[0]) &&
        // checks for overlap on the y axis (index 1 = y)
        (self.min[1] < other.max[1] && other.min[1] < self.max[1]) &&
        // checks for overlap on the z_axis (index 2 = z)
        (self.min[2] < other.max[2] && other.min[2] < self.max[2])
    }

    // fn covers(&self, other: &BoundingBox) -> bool {
    //     (self.min[0] <= other.min[0] && self.max[0] >= other.max[0]) && // if the x axis of other is fully contained within the bounds of self 
    //     (self.min[1] <= other.min[1] && self.max[1] >= other.max[1]) && // if the y axis of other is fully contained within the bounds of self
    //     (self.min[2] <= other.min[2] && self.max[2] >= other.max[2]) // if the z axis of other is fully contained within the bounds of self
    // }
}