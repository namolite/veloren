use super::{
    super::{vek::*, Animation},
    ArthropodSkeleton, SkeletonAttr,
};
use common::{comp::item::ToolKind, states::utils::StageSection};
use core::f32::consts::PI;

pub struct LeapShockAnimation;

impl Animation for LeapShockAnimation {
    type Dependency<'a> = (
        Option<ToolKind>,
        Option<ToolKind>,
        Vec3<f32>,
        f32,
        Option<StageSection>,
    );
    type Skeleton = ArthropodSkeleton;

    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"arthropod_leapshockwave\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "arthropod_leapshockwave")]
    fn update_skeleton_inner(
        skeleton: &Self::Skeleton,
        (_active_tool_kind, _second_tool_kind, _velocity, global_time, stage_section): Self::Dependency<'_>,
        anim_time: f32,
        _rate: &mut f32,
        s_a: &SkeletonAttr,
    ) -> Self::Skeleton {
        let mut next = (*skeleton).clone();

        let (movement1base, movement2base, movement3base, movement4) = match stage_section {
            Some(StageSection::Buildup) => (anim_time, 0.0, 0.0, 0.0),
            Some(StageSection::Movement) => (1.0, anim_time.powf(0.1), 0.0, 0.0),
            Some(StageSection::Action) => (1.0, 1.0, anim_time.powf(0.1), 0.0),
            Some(StageSection::Recover) => (0.0, 1.0, 1.0, anim_time.powi(4)),
            _ => (0.0, 0.0, 0.0, 0.0),
        };
        let pullback = 1.0 - movement4;
        let early_pullback = 1.0 - movement3base;
        let movement1abs = movement1base * pullback;
        let movement2abs = movement2base * pullback;
        let movement3abs = movement3base * pullback;

        let shortalt = (global_time * 80.0).sin() * movement2base * early_pullback;

        next.chest.scale = Vec3::one() / s_a.scaler;

        next.head.position = Vec3::new(0.0, s_a.head.0, s_a.head.1);
        next.head.orientation =
            Quaternion::rotation_x(movement1abs * -0.2 + movement2abs * 0.4 + movement3abs * -1.0)
                * Quaternion::rotation_z((movement1abs * 4.0 * PI).sin() * 0.08);

        next.chest.position = Vec3::new(0.0, s_a.chest.0, s_a.chest.1 + movement1abs * -0.5);
        next.chest.orientation = Quaternion::rotation_x(movement2abs * 0.3)
            * Quaternion::rotation_z((movement1abs * 4.0 * PI).sin() * 0.08);

        next.mandible_l.position = Vec3::new(-s_a.mandible.0, s_a.mandible.1, s_a.mandible.2);
        next.mandible_r.position = Vec3::new(s_a.mandible.0, s_a.mandible.1, s_a.mandible.2);
        next.mandible_l.orientation = Quaternion::rotation_x(
            (movement1abs * 4.0 * PI).sin() * 0.08 + movement2abs * 0.3 + movement3abs * -0.4,
        );
        next.mandible_r.orientation = Quaternion::rotation_x(
            (movement1abs * 4.0 * PI).sin() * 0.08 + movement2abs * 0.3 + movement3abs * -0.4,
        );

        next.wing_fl.position = Vec3::new(-s_a.wing_f.0, s_a.wing_f.1, s_a.wing_f.2);
        next.wing_fr.position = Vec3::new(s_a.wing_f.0, s_a.wing_f.1, s_a.wing_f.2);

        next.wing_bl.position = Vec3::new(-s_a.wing_b.0, s_a.wing_b.1, s_a.wing_b.2);
        next.wing_br.position = Vec3::new(s_a.wing_b.0, s_a.wing_b.1, s_a.wing_b.2);

        next.leg_fl.position = Vec3::new(-s_a.leg_f.0, s_a.leg_f.1, s_a.leg_f.2);
        next.leg_fr.position = Vec3::new(s_a.leg_f.0, s_a.leg_f.1, s_a.leg_f.2);
        next.leg_fl.orientation =
            Quaternion::rotation_x(movement1abs * 0.2 + movement2abs * 0.8 + movement3abs * -1.5)
                * Quaternion::rotation_z(s_a.leg_ori.0);
        next.leg_fr.orientation =
            Quaternion::rotation_x(movement1abs * 0.2 + movement2abs * 0.8 + movement3abs * -1.5)
                * Quaternion::rotation_z(-s_a.leg_ori.0);

        next.leg_fcl.position = Vec3::new(-s_a.leg_fc.0, s_a.leg_fc.1, s_a.leg_fc.2);
        next.leg_fcr.position = Vec3::new(s_a.leg_fc.0, s_a.leg_fc.1, s_a.leg_fc.2);
        next.leg_fcl.orientation =
            Quaternion::rotation_y(movement1abs * 0.2 + movement2abs * -1.0 + movement3abs * 0.8)
                * Quaternion::rotation_z(s_a.leg_ori.1);
        next.leg_fcr.orientation = Quaternion::rotation_y(movement1abs * -0.2 + movement2abs * 1.0)
            * Quaternion::rotation_z(-s_a.leg_ori.1);

        next.leg_bcl.position = Vec3::new(-s_a.leg_bc.0, s_a.leg_bc.1, s_a.leg_bc.2);
        next.leg_bcr.position = Vec3::new(s_a.leg_bc.0, s_a.leg_bc.1, s_a.leg_bc.2);
        next.leg_bcl.orientation =
            Quaternion::rotation_y(movement1abs * 0.2 + movement2abs * -1.0 + movement3abs * 0.8)
                * Quaternion::rotation_z(s_a.leg_ori.2);
        next.leg_bcr.orientation = Quaternion::rotation_y(movement1abs * -0.2 + movement2abs * 1.0)
            * Quaternion::rotation_z(-s_a.leg_ori.2);

        next.leg_bl.position = Vec3::new(-s_a.leg_b.0, s_a.leg_b.1, s_a.leg_b.2);
        next.leg_br.position = Vec3::new(s_a.leg_b.0, s_a.leg_b.1, s_a.leg_b.2);
        next.leg_bl.orientation =
            Quaternion::rotation_y(movement1abs * 0.2 + movement2abs * -1.0 + movement3abs * 0.8)
                * Quaternion::rotation_z(s_a.leg_ori.3);
        next.leg_br.orientation = Quaternion::rotation_y(movement1abs * -0.2 + movement2abs * 1.0)
            * Quaternion::rotation_z(-s_a.leg_ori.3);

        next.wing_fl.position = Vec3::new(-s_a.wing_f.0, s_a.wing_f.1, s_a.wing_f.2);
        next.wing_fr.position = Vec3::new(s_a.wing_f.0, s_a.wing_f.1, s_a.wing_f.2);
        next.wing_fl.orientation =
            Quaternion::rotation_x(movement1abs * -0.4 + movement2abs * -0.2)
                * Quaternion::rotation_y(movement1abs * 0.5 + movement2abs * 0.1)
                * Quaternion::rotation_z(movement1abs * -0.2);
        next.wing_fr.orientation =
            Quaternion::rotation_x(movement1abs * -0.4 + movement2abs * -0.2)
                * Quaternion::rotation_y(movement1abs * -0.5 + movement2abs * -0.1)
                * Quaternion::rotation_z(movement1abs * 0.2);

        next.wing_bl.position = Vec3::new(-s_a.wing_b.0, s_a.wing_b.1, s_a.wing_b.2);
        next.wing_br.position = Vec3::new(s_a.wing_b.0, s_a.wing_b.1, s_a.wing_b.2);
        next.wing_bl.orientation =
            Quaternion::rotation_x((movement1abs * -0.2 + movement2abs * -0.6) * early_pullback)
                * Quaternion::rotation_y(movement1abs * 0.4 + shortalt * 2.0 + movement2abs * 0.1)
                * Quaternion::rotation_z(movement1abs * -1.4);
        next.wing_br.orientation =
            Quaternion::rotation_x((movement1abs * -0.2 + movement2abs * -0.6) * early_pullback)
                * Quaternion::rotation_y(
                    movement1abs * -0.4 + shortalt * 2.0 + movement2abs * -0.1,
                )
                * Quaternion::rotation_z(movement1abs * 1.4);

        next
    }
}
