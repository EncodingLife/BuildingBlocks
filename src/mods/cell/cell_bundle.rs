use bevy::prelude::*;

use super::{Cell, BNA};

#[derive(Bundle)]
pub struct CellBundle {
    cell: Cell,
    global_transform: GlobalTransform,
    visibility: InheritedVisibility,
}

impl CellBundle {
    pub fn new(bna: BNA) -> Self {
        CellBundle {
            cell: Cell { bna },
            global_transform: GlobalTransform::IDENTITY,
            visibility: InheritedVisibility::VISIBLE,
        }
    }
}
