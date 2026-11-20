import com.crispytwig.naturalist.client.animation.definitions.BirdAnimation;
import com.crispytwig.naturalist.client.model.NaturalistEntityModel;
import com.crispytwig.naturalist.client.model.animal.bird.BirdModel;
import com.crispytwig.naturalist.world.entity.SmoothAnimationState;
import java.util.Locale;
import java.util.stream.Stream;
import net.minecraft.client.model.geom.ModelPart;
import net.minecraft.util.Mth;
import net.minecraft.world.entity.Entity;

public final class BirdPoseProbe {
    private static final class Sample extends NaturalistEntityModel<Entity> {
        final ModelPart baked;
        final SmoothAnimationState idle = new SmoothAnimationState();
        final SmoothAnimationState walk = new SmoothAnimationState();
        final SmoothAnimationState fly = new SmoothAnimationState();
        float oldPitch;
        float flyPitch;

        Sample(ModelPart root) {
            super(root);
            baked = root;
        }

        @Override
        protected void setupAnimations(Entity unused, float limbSwing, float speed, float age,
                                       float partial, float headYaw, float pitch) {}

        void tick(int tick, boolean grounded, boolean walking, float pitch, float vertical) {
            idle.animateWhen(grounded && !walking, tick);
            walk.animateWhen(grounded && walking, tick);
            fly.animateWhen(!grounded, tick);
            oldPitch = flyPitch;
            float target = grounded ? 0.0F : pitch * (Mth.PI / 360F) - vertical * 3.0F;
            flyPitch += (target - flyPitch) * 0.3F;
        }

        static String phase(SmoothAnimationState clock) {
            return clock.isStarted() ? Long.toString(clock.getAccumulatedTime()) : "null";
        }

        void frame(int tick, float partial, boolean grounded, float speed, float pitch, float yaw) {
            baked.getAllParts().forEach(ModelPart::resetPose);
            float age = tick + partial;
            animateIdleSmooth(idle, BirdAnimation.PASSERINE_IDLE, age, partial, speed);
            animateSmooth(walk, BirdAnimation.PASSERINE_WALK, age, partial,
                Mth.clamp(speed / (float) (Math.min(8.64D * 0.2D, 1.0D) * 0.65D), 0.4F, 2.0F));
            animateSmooth(fly, BirdAnimation.PASSERINE_FLY, age, partial);
            var root = baked.getChild("root");
            var lookup = root.createPartLookup();
            applyHeadLook(lookup.apply("head"), yaw, pitch);
            if (!grounded) {
                root.xRot += Mth.lerp(partial, oldPitch, flyPitch);
                root.zRot = yaw * (Mth.PI / 600F);
            }
            String bones = Stream.of("root", "head", "leftWingRot", "rightWingRot", "left_leg", "right_leg")
                .map(name -> {
                    var part = name.equals("root") ? root : lookup.apply(name);
                    return String.format(Locale.ROOT,
                        "\"%s\":[%.7f,%.7f,%.7f,%.7f,%.7f,%.7f]",
                        name, part.x, part.y, part.z, part.xRot, part.yRot, part.zRot);
                }).reduce((a, b) -> a + "," + b).orElseThrow();
            System.out.printf(Locale.ROOT,
                "{\"tick\":%d,\"partial\":%.2f,\"weights\":[%.7f,%.7f,%.7f],\"time_ms\":[%s,%s,%s],\"bones\":{%s}}%n",
                tick, partial, idle.factor(partial), walk.factor(partial), fly.factor(partial),
                phase(idle), phase(walk), phase(fly), bones);
        }
    }

    public static void main(String[] args) {
        var bird = new Sample(BirdModel.createBodyLayer().bakeRoot());
        for (int tick = 1; tick <= 48; tick++) {
            boolean walking = 9 <= tick && tick < 20;
            boolean flying = 20 <= tick && tick < 36;
            float speed = walking ? 0.26F : 0.0F;
            float pitch = flying ? 22.0F : 6.0F;
            float yaw = flying ? -16.0F : 8.0F;
            float vertical = flying ? -0.22F : 0.0F;
            bird.tick(tick, !flying, walking, pitch, vertical);
            for (float partial : new float[]{0.0F, 0.5F, 0.9F}) {
                bird.frame(tick, partial, !flying, speed, pitch, yaw);
            }
        }
    }
}
