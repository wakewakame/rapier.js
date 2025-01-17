use crate::dynamics::{RawIslandManager, RawRigidBodySet};
use crate::geometry::{
    RawColliderSet, RawPointColliderProjection, RawRayColliderIntersection, RawRayColliderToi,
    RawShape, RawShapeColliderTOI,
};
use crate::math::{RawRotation, RawVector};
use crate::utils::{self, FlatHandle};
use rapier::geometry::{Collider, ColliderHandle, Ray, AABB};
use rapier::math::{Isometry, Point};
use rapier::pipeline::{QueryFilter, QueryFilterFlags, QueryPipeline};
use rapier::prelude::FeatureId;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RawQueryPipeline(pub(crate) QueryPipeline);

#[wasm_bindgen]
impl RawQueryPipeline {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        RawQueryPipeline(QueryPipeline::new())
    }

    pub fn update(
        &mut self,
        islands: &RawIslandManager,
        bodies: &RawRigidBodySet,
        colliders: &RawColliderSet,
    ) {
        self.0.update(&islands.0, &bodies.0, &colliders.0);
    }

    pub fn castRay(
        &self,
        bodies: &RawRigidBodySet,
        colliders: &RawColliderSet,
        rayOrig: &RawVector,
        rayDir: &RawVector,
        maxToi: f32,
        solid: bool,
        filter_flags: u32,
        filter_groups: Option<u32>,
        filter_exclude_collider: Option<FlatHandle>,
        filter_exclude_rigid_body: Option<FlatHandle>,
        filter_predicate: &js_sys::Function,
    ) -> Option<RawRayColliderToi> {
        let predicate = wrap_filter(filter_predicate);
        let predicate = predicate
            .as_ref()
            .map(|f| f as &dyn Fn(ColliderHandle, &Collider) -> bool);
        let query_filter = QueryFilter {
            flags: QueryFilterFlags::from_bits(filter_flags).unwrap_or(QueryFilterFlags::empty()),
            groups: filter_groups.map(crate::geometry::unpack_interaction_groups),
            exclude_collider: filter_exclude_collider.map(crate::utils::collider_handle),
            exclude_rigid_body: filter_exclude_rigid_body.map(crate::utils::body_handle),
            predicate,
        };

        let ray = Ray::new(rayOrig.0.into(), rayDir.0);
        let (handle, toi) =
            self.0
                .cast_ray(&bodies.0, &colliders.0, &ray, maxToi, solid, query_filter)?;
        Some(RawRayColliderToi { handle, toi })
    }

    pub fn castRayAndGetNormal(
        &self,
        bodies: &RawRigidBodySet,
        colliders: &RawColliderSet,
        rayOrig: &RawVector,
        rayDir: &RawVector,
        maxToi: f32,
        solid: bool,
        filter_flags: u32,
        filter_groups: Option<u32>,
        filter_exclude_collider: Option<FlatHandle>,
        filter_exclude_rigid_body: Option<FlatHandle>,
        filter_predicate: &js_sys::Function,
    ) -> Option<RawRayColliderIntersection> {
        let predicate = wrap_filter(filter_predicate);
        let predicate = predicate
            .as_ref()
            .map(|f| f as &dyn Fn(ColliderHandle, &Collider) -> bool);
        let query_filter = QueryFilter {
            flags: QueryFilterFlags::from_bits(filter_flags).unwrap_or(QueryFilterFlags::empty()),
            groups: filter_groups.map(crate::geometry::unpack_interaction_groups),
            exclude_collider: filter_exclude_collider.map(crate::utils::collider_handle),
            exclude_rigid_body: filter_exclude_rigid_body.map(crate::utils::body_handle),
            predicate,
        };

        let ray = Ray::new(rayOrig.0.into(), rayDir.0);
        let (handle, inter) = self.0.cast_ray_and_get_normal(
            &bodies.0,
            &colliders.0,
            &ray,
            maxToi,
            solid,
            query_filter,
        )?;
        Some(RawRayColliderIntersection { handle, inter })
    }

    // The callback is of type (RawRayColliderIntersection) => bool
    pub fn intersectionsWithRay(
        &self,
        bodies: &RawRigidBodySet,
        colliders: &RawColliderSet,
        rayOrig: &RawVector,
        rayDir: &RawVector,
        maxToi: f32,
        solid: bool,
        callback: &js_sys::Function,
        filter_flags: u32,
        filter_groups: Option<u32>,
        filter_exclude_collider: Option<FlatHandle>,
        filter_exclude_rigid_body: Option<FlatHandle>,
        filter_predicate: &js_sys::Function,
    ) {
        let predicate = wrap_filter(filter_predicate);
        let predicate = predicate
            .as_ref()
            .map(|f| f as &dyn Fn(ColliderHandle, &Collider) -> bool);
        let query_filter = QueryFilter {
            flags: QueryFilterFlags::from_bits(filter_flags).unwrap_or(QueryFilterFlags::empty()),
            groups: filter_groups.map(crate::geometry::unpack_interaction_groups),
            exclude_collider: filter_exclude_collider.map(crate::utils::collider_handle),
            exclude_rigid_body: filter_exclude_rigid_body.map(crate::utils::body_handle),
            predicate,
        };

        let ray = Ray::new(rayOrig.0.into(), rayDir.0);
        let rcallback = |handle, inter| {
            let result = RawRayColliderIntersection { handle, inter };
            match callback.call1(&JsValue::null(), &JsValue::from(result)) {
                Err(_) => true,
                Ok(val) => val.as_bool().unwrap_or(true),
            }
        };

        self.0.intersections_with_ray(
            &bodies.0,
            &colliders.0,
            &ray,
            maxToi,
            solid,
            query_filter,
            rcallback,
        );
    }

    pub fn intersectionWithShape(
        &self,
        bodies: &RawRigidBodySet,
        colliders: &RawColliderSet,
        shapePos: &RawVector,
        shapeRot: &RawRotation,
        shape: &RawShape,
        filter_flags: u32,
        filter_groups: Option<u32>,
        filter_exclude_collider: Option<FlatHandle>,
        filter_exclude_rigid_body: Option<FlatHandle>,
        filter_predicate: &js_sys::Function,
    ) -> Option<FlatHandle> {
        let predicate = wrap_filter(filter_predicate);
        let predicate = predicate
            .as_ref()
            .map(|f| f as &dyn Fn(ColliderHandle, &Collider) -> bool);
        let query_filter = QueryFilter {
            flags: QueryFilterFlags::from_bits(filter_flags).unwrap_or(QueryFilterFlags::empty()),
            groups: filter_groups.map(crate::geometry::unpack_interaction_groups),
            exclude_collider: filter_exclude_collider.map(crate::utils::collider_handle),
            exclude_rigid_body: filter_exclude_rigid_body.map(crate::utils::body_handle),
            predicate,
        };

        let pos = Isometry::from_parts(shapePos.0.into(), shapeRot.0);
        self.0
            .intersection_with_shape(&bodies.0, &colliders.0, &pos, &*shape.0, query_filter)
            .map(|h| utils::flat_handle(h.0))
    }

    pub fn projectPoint(
        &self,
        bodies: &RawRigidBodySet,
        colliders: &RawColliderSet,
        point: &RawVector,
        solid: bool,
        filter_flags: u32,
        filter_groups: Option<u32>,
        filter_exclude_collider: Option<FlatHandle>,
        filter_exclude_rigid_body: Option<FlatHandle>,
        filter_predicate: &js_sys::Function,
    ) -> Option<RawPointColliderProjection> {
        let predicate = wrap_filter(filter_predicate);
        let predicate = predicate
            .as_ref()
            .map(|f| f as &dyn Fn(ColliderHandle, &Collider) -> bool);
        let query_filter = QueryFilter {
            flags: QueryFilterFlags::from_bits(filter_flags).unwrap_or(QueryFilterFlags::empty()),
            groups: filter_groups.map(crate::geometry::unpack_interaction_groups),
            exclude_collider: filter_exclude_collider.map(crate::utils::collider_handle),
            exclude_rigid_body: filter_exclude_rigid_body.map(crate::utils::body_handle),
            predicate,
        };

        self.0
            .project_point(
                &bodies.0,
                &colliders.0,
                &point.0.into(),
                solid,
                query_filter,
            )
            .map(|(handle, proj)| RawPointColliderProjection {
                handle,
                proj,
                feature: FeatureId::Unknown,
            })
    }

    pub fn projectPointAndGetFeature(
        &self,
        bodies: &RawRigidBodySet,
        colliders: &RawColliderSet,
        point: &RawVector,
        filter_flags: u32,
        filter_groups: Option<u32>,
        filter_exclude_collider: Option<FlatHandle>,
        filter_exclude_rigid_body: Option<FlatHandle>,
        filter_predicate: &js_sys::Function,
    ) -> Option<RawPointColliderProjection> {
        let predicate = wrap_filter(filter_predicate);
        let predicate = predicate
            .as_ref()
            .map(|f| f as &dyn Fn(ColliderHandle, &Collider) -> bool);
        let query_filter = QueryFilter {
            flags: QueryFilterFlags::from_bits(filter_flags).unwrap_or(QueryFilterFlags::empty()),
            groups: filter_groups.map(crate::geometry::unpack_interaction_groups),
            exclude_collider: filter_exclude_collider.map(crate::utils::collider_handle),
            exclude_rigid_body: filter_exclude_rigid_body.map(crate::utils::body_handle),
            predicate,
        };

        self.0
            .project_point_and_get_feature(&bodies.0, &colliders.0, &point.0.into(), query_filter)
            .map(|(handle, proj, feature)| RawPointColliderProjection {
                handle,
                proj,
                feature,
            })
    }

    // The callback is of type (u32) => bool
    pub fn intersectionsWithPoint(
        &self,
        bodies: &RawRigidBodySet,
        colliders: &RawColliderSet,
        point: &RawVector,
        callback: &js_sys::Function,
        filter_flags: u32,
        filter_groups: Option<u32>,
        filter_exclude_collider: Option<FlatHandle>,
        filter_exclude_rigid_body: Option<FlatHandle>,
        filter_predicate: &js_sys::Function,
    ) {
        let predicate = wrap_filter(filter_predicate);
        let predicate = predicate
            .as_ref()
            .map(|f| f as &dyn Fn(ColliderHandle, &Collider) -> bool);
        let query_filter = QueryFilter {
            flags: QueryFilterFlags::from_bits(filter_flags).unwrap_or(QueryFilterFlags::empty()),
            groups: filter_groups.map(crate::geometry::unpack_interaction_groups),
            exclude_collider: filter_exclude_collider.map(crate::utils::collider_handle),
            exclude_rigid_body: filter_exclude_rigid_body.map(crate::utils::body_handle),
            predicate,
        };

        let rcallback = |handle: ColliderHandle| match callback.call1(
            &JsValue::null(),
            &JsValue::from(utils::flat_handle(handle.0)),
        ) {
            Err(_) => true,
            Ok(val) => val.as_bool().unwrap_or(true),
        };
        self.0.intersections_with_point(
            &bodies.0,
            &colliders.0,
            &point.0.into(),
            query_filter,
            rcallback,
        )
    }

    pub fn castShape(
        &self,
        bodies: &RawRigidBodySet,
        colliders: &RawColliderSet,
        shapePos: &RawVector,
        shapeRot: &RawRotation,
        shapeVel: &RawVector,
        shape: &RawShape,
        maxToi: f32,
        filter_flags: u32,
        filter_groups: Option<u32>,
        filter_exclude_collider: Option<FlatHandle>,
        filter_exclude_rigid_body: Option<FlatHandle>,
        filter_predicate: &js_sys::Function,
    ) -> Option<RawShapeColliderTOI> {
        let predicate = wrap_filter(filter_predicate);
        let predicate = predicate
            .as_ref()
            .map(|f| f as &dyn Fn(ColliderHandle, &Collider) -> bool);
        let query_filter = QueryFilter {
            flags: QueryFilterFlags::from_bits(filter_flags).unwrap_or(QueryFilterFlags::empty()),
            groups: filter_groups.map(crate::geometry::unpack_interaction_groups),
            exclude_collider: filter_exclude_collider.map(crate::utils::collider_handle),
            exclude_rigid_body: filter_exclude_rigid_body.map(crate::utils::body_handle),
            predicate,
        };

        let pos = Isometry::from_parts(shapePos.0.into(), shapeRot.0);
        self.0
            .cast_shape(
                &bodies.0,
                &colliders.0,
                &pos,
                &shapeVel.0,
                &*shape.0,
                maxToi,
                query_filter,
            )
            .map(|(handle, toi)| RawShapeColliderTOI { handle, toi })
    }

    // The callback has type (u32) => boolean
    pub fn intersectionsWithShape(
        &self,
        bodies: &RawRigidBodySet,
        colliders: &RawColliderSet,
        shapePos: &RawVector,
        shapeRot: &RawRotation,
        shape: &RawShape,
        callback: &js_sys::Function,
        filter_flags: u32,
        filter_groups: Option<u32>,
        filter_exclude_collider: Option<FlatHandle>,
        filter_exclude_rigid_body: Option<FlatHandle>,
        filter_predicate: &js_sys::Function,
    ) {
        let predicate = wrap_filter(filter_predicate);
        let predicate = predicate
            .as_ref()
            .map(|f| f as &dyn Fn(ColliderHandle, &Collider) -> bool);
        let query_filter = QueryFilter {
            flags: QueryFilterFlags::from_bits(filter_flags).unwrap_or(QueryFilterFlags::empty()),
            groups: filter_groups.map(crate::geometry::unpack_interaction_groups),
            exclude_collider: filter_exclude_collider.map(crate::utils::collider_handle),
            exclude_rigid_body: filter_exclude_rigid_body.map(crate::utils::body_handle),
            predicate,
        };

        let rcallback = |handle: ColliderHandle| match callback.call1(
            &JsValue::null(),
            &JsValue::from(utils::flat_handle(handle.0)),
        ) {
            Err(_) => true,
            Ok(val) => val.as_bool().unwrap_or(true),
        };

        let pos = Isometry::from_parts(shapePos.0.into(), shapeRot.0);
        self.0.intersections_with_shape(
            &bodies.0,
            &colliders.0,
            &pos,
            &*shape.0,
            query_filter,
            rcallback,
        )
    }

    pub fn collidersWithAabbIntersectingAabb(
        &self,
        aabbCenter: &RawVector,
        aabbHalfExtents: &RawVector,
        callback: &js_sys::Function,
    ) {
        let rcallback = |handle: &ColliderHandle| match callback.call1(
            &JsValue::null(),
            &JsValue::from(utils::flat_handle(handle.0)),
        ) {
            Err(_) => true,
            Ok(val) => val.as_bool().unwrap_or(true),
        };

        let center = Point::from(aabbCenter.0);
        let aabb = AABB::new(center - aabbHalfExtents.0, center + aabbHalfExtents.0);

        self.0
            .colliders_with_aabb_intersecting_aabb(&aabb, rcallback)
    }
}

fn wrap_filter(
    filter: &js_sys::Function,
) -> Option<impl Fn(ColliderHandle, &Collider) -> bool + '_> {
    if filter.is_function() {
        let filtercb = move |handle: ColliderHandle, _: &Collider| match filter.call1(
            &JsValue::null(),
            &JsValue::from(utils::flat_handle(handle.0)),
        ) {
            Err(_) => true,
            Ok(val) => val.as_bool().unwrap_or(true),
        };

        Some(filtercb)
    } else {
        None
    }
}
