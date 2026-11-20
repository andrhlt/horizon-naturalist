import com.crispytwig.naturalist.client.animation.definitions.ElephantAnimation;
import com.crispytwig.naturalist.client.animation.definitions.ElephantBabyAnimation;
import com.crispytwig.naturalist.client.model.NaturalistEntityModel;
import com.crispytwig.naturalist.client.model.animal.elephant.MammothBabyModel;
import com.crispytwig.naturalist.world.entity.SmoothAnimationState;
import java.util.Locale;
import net.minecraft.client.model.geom.ModelPart;
import net.minecraft.world.entity.Entity;

public final class MammothPoseProbe extends NaturalistEntityModel<Entity> {
    private final ModelPart baked;
    private final SmoothAnimationState idle = new SmoothAnimationState();

    private MammothPoseProbe(ModelPart root) {
        super(root);
        baked = root;
    }

    @Override
    protected void setupAnimations(Entity ignored, float stride, float speed, float age,
                                   float partial, float yaw, float pitch) {}

    private void sample(boolean babyClip) {
        for (int tick = 1; tick <= 120; ++tick) {
            idle.animateWhen(true, tick);
            baked.getAllParts().forEach(ModelPart::resetPose);
            var clip = babyClip ? ElephantBabyAnimation.ELEPHANT_IDLE : ElephantAnimation.ELEPHANT_IDLE;
            animateIdleSmooth(idle, clip, tick + 0.5F, 0.5F, 0.0F);
            if (tick % 10 == 0) {
                var lookup = baked.getChild("root").createPartLookup();
                var body = lookup.apply("body");
                var trunk = lookup.apply("trunk");
                var ear = lookup.apply("leftEar");
                System.out.printf(Locale.ROOT,
                    "{\"tick\":%d,\"baby_clip\":%s,\"body_z\":%.9f,\"trunk_x\":%.9f,\"ear_z\":%.9f}%n",
                    tick, babyClip, body.zRot, trunk.xRot, ear.zRot);
            }
        }
    }

    public static void main(String[] args) {
        new MammothPoseProbe(MammothBabyModel.createBodyLayer().bakeRoot()).sample(true);
        new MammothPoseProbe(MammothBabyModel.createBodyLayer().bakeRoot()).sample(false);
    }
}
