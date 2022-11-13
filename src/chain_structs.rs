use anchor_lang::prelude::*;
use bevy::prelude::*;

use dominarisystems::components::{
    ComponentFeature, ComponentLastUsedSlot, ComponentLocation, ComponentMapMeta,
    ComponentMetadata, ComponentOccupant, ComponentOwner,
};

#[derive(Debug)]
pub struct Entities {
    pub map: MapEntity,
    pub tiles: Vec<TileEntity>,
    pub features: Vec<FeatureEntity>,
    pub units: Vec<UnitEntity>,
}

#[derive(Debug)]
pub struct MapEntity {
    pub map_meta: ComponentMapMeta,
    pub metadata: ComponentMetadata,
}

#[derive(Debug)]
pub struct TileEntity {
    pub metadata: ComponentMetadata,
    pub location: ComponentLocation,
    pub feature: Option<ComponentFeature>,
    pub owner: ComponentOwner,
    pub occupant: Option<ComponentOccupant>,
}

#[derive(Debug)]
pub struct FeatureEntity {
    pub metadata: ComponentMetadata,
    pub location: ComponentLocation,
    pub owner: ComponentOwner,
    pub last_used: ComponentLastUsedSlot,
}

#[derive(Debug)]
pub struct UnitEntity {
    pub metadata: ComponentMetadata,
    pub owner: ComponentOwner,
    pub last_used: ComponentLastUsedSlot,
}

#[derive(Component, Debug)]
pub struct CMetadata(pub ComponentMetadata);

#[derive(Component, Debug)]
pub struct CLocation(pub ComponentLocation);

#[derive(Component, Debug)]
pub struct COwner(pub ComponentOwner);

#[derive(Component, Debug)]
pub struct CFeature(pub ComponentFeature);

#[derive(Component, Debug)]
pub struct COccupant(pub ComponentOccupant);

#[derive(Component, Debug)]
pub struct CMapMeta(pub ComponentMapMeta);


#[derive(Component, Debug)]
pub struct TileMarker;

//todo: PlayerEntity

pub fn mock_data() -> Entities {
    use dominarisystems::components::*;

    fn meta(s: &str) -> ComponentMetadata {
        ComponentMetadata {
            name: s.into(),
            entity_type: s.into(),
            world_instance: default(),
        }
    }

    let owner = Pubkey::new_unique();
    let player = Pubkey::new_unique();
    let feature_1 = Pubkey::new_unique();

    let mut tiles: Vec<TileEntity> = (0..8)
        .flat_map(|x| {
            (0..8).map(move |y| TileEntity {
                metadata: meta("Tile"),
                location: ComponentLocation { x, y },
                feature: None,
                owner: ComponentOwner { owner, player },
                occupant: None,
            })
        })
        .collect();

    tiles[0].feature = Some(ComponentFeature {
        feature_id: feature_1,
    });

    Entities {
        map: MapEntity {
            map_meta: ComponentMapMeta {
                max_x: 8,
                max_y: 8,
                play_phase: true,
            },
            metadata: ComponentMetadata {
                name: "Map".into(),
                entity_type: "Map".into(),
                world_instance: default(),
            },
        },
        features: vec![FeatureEntity {
            metadata: meta("Feature"),
            location: ComponentLocation {
                x: tiles[0].location.x,
                y: tiles[0].location.y,
            },
            owner: ComponentOwner { owner, player },
            last_used: ComponentLastUsedSlot {
                last_used: 0,
                recovery: 5,
            },
        }],
        tiles,
        units: vec![],
    }
}
