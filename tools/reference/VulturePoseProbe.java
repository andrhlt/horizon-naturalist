import com.crispytwig.naturalist.client.animation.definitions.VultureAnimation;
import com.crispytwig.naturalist.client.model.NaturalistEntityModel;
import com.crispytwig.naturalist.client.model.animal.vulture.VultureModel;
import com.crispytwig.naturalist.world.entity.SmoothAnimationState;
import java.util.Locale;
import java.util.stream.Stream;
import net.minecraft.client.model.geom.ModelPart;
import net.minecraft.world.entity.Entity;

public final class VulturePoseProbe {
    private static final class Sample extends NaturalistEntityModel<Entity> {
        final ModelPart baked;
        final SmoothAnimationState sit = new SmoothAnimationState();
        final SmoothAnimationState fly = new SmoothAnimationState();

        Sample(ModelPart root) {
            super(root);
            baked = root;
        }

        @Override
        protected String getRootPartName() {
            return "body";
        }

        @Override
        protected void setupAnimations(Entity unused, float swing, float speed, float age,
                                       float partial, float yaw, float pitch) {}

        static String time(SmoothAnimationState clock) {
            return clock.isStarted() ? Long.toString(clock.getAccumulatedTime()) : "null";
        }

        void frame(int tick, float partial, boolean perched, float yaw, float pitch) {
            baked.getAllParts().forEach(ModelPart::resetPose);
            float age = tick + partial;
            animateSmooth(sit, VultureAnimation.VULTURE_SIT, age, partial);
            animateSmooth(fly, VultureAnimation.VULTURE_FLY, age, partial);
            var body = baked.getChild("body");
            var lookup = body.createPartLookup();
            applyHeadLook(lookup.apply("neck"), yaw, pitch);
            String bones = Stream.of("body", "neck", "held_item", "left_wing_tip", "right_wing_tip", "leftFoot")
                .map(name -> {
                    var part = name.equals("body") ? body : lookup.apply(name);
                    return String.format(Locale.ROOT,
                        "\"%s\":[%.7f,%.7f,%.7f,%.7f,%.7f,%.7f]",
                        name, part.x, part.y, part.z, part.xRot, part.yRot, part.zRot);
                }).reduce((a, b) -> a + "," + b).orElseThrow();
            System.out.printf(Locale.ROOT,
                "{\"tick\":%d,\"partial\":%.2f,\"perched\":%s,\"weights\":[%.7f,%.7f],\"time_ms\":[%s,%s],\"bones\":{%s}}%n",
                tick, partial, perched, sit.factor(partial), fly.factor(partial), time(sit), time(fly), bones);
        }
    }

    public static void main(String[] args) {
        var model = new Sample(VultureModel.createBodyLayer().bakeRoot());
        for (int tick = 1; tick <= 48; tick++) {
            boolean perched = 13 <= tick && tick < 34;
            float yaw = perched ? 18.0F : -9.0F;
            float pitch = perched ? 10.0F : 6.0F;
            model.sit.animateWhen(perched, tick);
            model.fly.animateWhen(!perched, tick);
            for (float partial : new float[]{0.0F, 0.5F, 0.9F}) {
                model.frame(tick, partial, perched, yaw, pitch);
            }
        }
    }
}
