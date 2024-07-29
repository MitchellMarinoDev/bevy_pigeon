//! Types in this file:
//!  - [AmbientLight]
//!  - [DirectionalLight]
//!  - [PointLight]

use bevy::prelude::*;

use bevy_pbr::{
    AmbientLight,
    DirectionalLight,
    PointLight,
};
use serde::{Deserialize, Serialize};

/// The network-able version of [AmbientLight].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NetAmbientLight {
    pub color: Color,
    /// A direct scale factor multiplied with `color` before being passed to the shader.
    pub brightness: f32,
}

impl From<AmbientLight> for NetAmbientLight {
    fn from(o: AmbientLight) -> Self {
        NetAmbientLight {
            color: o.color,
            brightness: o.brightness,
        }
    }
}

impl From<NetAmbientLight> for AmbientLight {
    fn from(o: NetAmbientLight) -> Self {
        AmbientLight {
            color: o.color,
            brightness: o.brightness,
        }
    }
}

/// The network-able version of [DirectionalLight].
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct NetDirectionalLight {
    pub color: Color,
    /// Illuminance in lux
    pub illuminance: f32,
    pub shadows_enabled: bool,
    pub shadow_depth_bias: f32,
    /// A bias applied along the direction of the fragment's surface normal. It is scaled to the
    /// shadow map's texel size so that it is automatically adjusted to the orthographic projection.
    pub shadow_normal_bias: f32,
}

impl From<DirectionalLight> for NetDirectionalLight {
    fn from(o: DirectionalLight) -> Self {
        NetDirectionalLight {
            color: o.color,
            illuminance: o.illuminance,
            shadows_enabled: o.shadows_enabled,
            shadow_depth_bias: o.shadow_depth_bias,
            shadow_normal_bias: o.shadow_normal_bias,
        }
    }
}

impl From<NetDirectionalLight> for DirectionalLight {
    fn from(o: NetDirectionalLight) -> Self {
        DirectionalLight {
            color: o.color,
            illuminance: o.illuminance,
            shadows_enabled: o.shadows_enabled,
            shadow_depth_bias: o.shadow_depth_bias,
            shadow_normal_bias: o.shadow_normal_bias,
        }
    }
}

/// The network-able version of [PointLight].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NetPointLight {
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub radius: f32,
    pub shadows_enabled: bool,
    pub shadow_depth_bias: f32,
    /// A bias applied along the direction of the fragment's surface normal. It is scaled to the
    /// shadow map's texel size so that it can be small close to the camera and gets larger further
    /// away.
    pub shadow_normal_bias: f32,
}

impl From<PointLight> for NetPointLight {
    fn from(o: PointLight) -> Self {
        NetPointLight {
            color: o.color,
            intensity: o.intensity,
            range: o.range,
            radius: o.radius,
            shadows_enabled: o.shadows_enabled,
            shadow_depth_bias: o.shadow_depth_bias,
            shadow_normal_bias: o.shadow_normal_bias,
        }
    }
}

impl From<NetPointLight> for PointLight {
    fn from(o: NetPointLight) -> Self {
        PointLight {
            color: o.color,
            intensity: o.intensity,
            range: o.range,
            radius: o.radius,
            shadows_enabled: o.shadows_enabled,
            shadow_depth_bias: o.shadow_depth_bias,
            shadow_normal_bias: o.shadow_normal_bias,
        }
    }
}
