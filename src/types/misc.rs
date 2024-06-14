#![allow(deprecated)]

//! Types in this file:
//! - [OrthographicProjection]
//! - [Name]
//! - [Visibility]
//! - [AlphaMode]
//! - [EulerRot]

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_pbr::AlphaMode;
use serde::{Deserialize, Serialize};

/// The network-able version of [OrthographicProjection].
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct NetOrthographicProjection {
    pub near: f32,
    pub far: f32,
    pub viewport_origin: Vec2,
    pub scaling_mode: ScalingMode,
    pub scale: f32,
    pub area: Rect,
}

impl From<OrthographicProjection> for NetOrthographicProjection {
    fn from(o: OrthographicProjection) -> Self {
        NetOrthographicProjection {
            near: o.near,
            far: o.far,
            viewport_origin: o.viewport_origin,
            scaling_mode: o.scaling_mode,
            scale: o.scale,
            area: o.area,
        }
    }
}

impl From<NetOrthographicProjection> for OrthographicProjection {
    fn from(o: NetOrthographicProjection) -> Self {
        OrthographicProjection {
            near: o.near,
            far: o.far,
            viewport_origin: o.viewport_origin,
            scaling_mode: o.scaling_mode,
            scale: o.scale,
            area: o.area,
        }
    }
}

/// The network-able version of [Name].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetName {
    pub name: String,
}

impl From<Name> for NetName {
    fn from(o: Name) -> Self {
        NetName {
            name: o.as_str().into(),
        }
    }
}

impl From<NetName> for Name {
    fn from(o: NetName) -> Self {
        Name::new(o.name)
    }
}

/// The network-able version of [Visibility].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NetVisibility {
    Inherited,
    Hidden,
    Visible,
}

impl From<Visibility> for NetVisibility {
    fn from(o: Visibility) -> Self {
        match o {
            Visibility::Inherited => NetVisibility::Inherited,
            Visibility::Hidden => NetVisibility::Hidden,
            Visibility::Visible => NetVisibility::Visible,
        }
    }
}

impl From<NetVisibility> for Visibility {
    fn from(o: NetVisibility) -> Self {
        match o {
            NetVisibility::Inherited => Visibility::Inherited,
            NetVisibility::Hidden => Visibility::Hidden,
            NetVisibility::Visible => Visibility::Visible,
        }
    }
}

/// The network-able version of [AlphaMode].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NetAlphaMode {
    Opaque,
    /// An alpha cutoff must be supplied where alpha values >= the cutoff
    /// will be fully opaque and < will be fully transparent
    Mask(f32),
    Blend,
    Premultiplied,
    Add,
    Multiply,
}

impl From<AlphaMode> for NetAlphaMode {
    fn from(o: AlphaMode) -> Self {
        match o {
            AlphaMode::Opaque => NetAlphaMode::Opaque,
            AlphaMode::Mask(v) => NetAlphaMode::Mask(v),
            AlphaMode::Blend => NetAlphaMode::Blend,
            AlphaMode::Premultiplied => NetAlphaMode::Premultiplied,
            AlphaMode::Add => NetAlphaMode::Add,
            AlphaMode::Multiply => NetAlphaMode::Multiply,
        }
    }
}

impl From<NetAlphaMode> for AlphaMode {
    fn from(o: NetAlphaMode) -> Self {
        match o {
            NetAlphaMode::Opaque => AlphaMode::Opaque,
            NetAlphaMode::Mask(v) => AlphaMode::Mask(v),
            NetAlphaMode::Blend => AlphaMode::Blend,
            NetAlphaMode::Premultiplied => AlphaMode::Premultiplied,
            NetAlphaMode::Add => AlphaMode::Add,
            NetAlphaMode::Multiply => AlphaMode::Multiply,
        }
    }
}

/// The network-able version of [EulerRot].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NetEulerRot {
    /// Intrinsic three-axis rotation ZYX
    ZYX,
    /// Intrinsic three-axis rotation ZXY
    ZXY,
    /// Intrinsic three-axis rotation YXZ
    YXZ,
    /// Intrinsic three-axis rotation YZX
    YZX,
    /// Intrinsic three-axis rotation XYZ
    XYZ,
    /// Intrinsic three-axis rotation XZY
    XZY,
}

impl From<EulerRot> for NetEulerRot {
    fn from(o: EulerRot) -> Self {
        match o {
            EulerRot::ZYX => NetEulerRot::ZYX,
            EulerRot::ZXY => NetEulerRot::ZXY,
            EulerRot::YXZ => NetEulerRot::YXZ,
            EulerRot::YZX => NetEulerRot::YZX,
            EulerRot::XYZ => NetEulerRot::XYZ,
            EulerRot::XZY => NetEulerRot::XZY,
        }
    }
}

impl From<NetEulerRot> for EulerRot {
    fn from(o: NetEulerRot) -> Self {
        match o {
            NetEulerRot::ZYX => EulerRot::ZYX,
            NetEulerRot::ZXY => EulerRot::ZXY,
            NetEulerRot::YXZ => EulerRot::YXZ,
            NetEulerRot::YZX => EulerRot::YZX,
            NetEulerRot::XYZ => EulerRot::XYZ,
            NetEulerRot::XZY => EulerRot::XZY,
        }
    }
}
