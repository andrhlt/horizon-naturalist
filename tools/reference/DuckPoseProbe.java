import com.crispytwig.naturalist.client.animation.definitions.DuckAnimation;
import com.crispytwig.naturalist.client.animation.definitions.DuckBabyAnimation;
import com.crispytwig.naturalist.client.model.NaturalistEntityModel;
import com.crispytwig.naturalist.client.model.animal.duck.DuckBabyModel;
import com.crispytwig.naturalist.client.model.animal.duck.DuckModel;
import com.crispytwig.naturalist.world.entity.SmoothAnimationState;
import java.util.Arrays;
import java.util.Locale;
import net.minecraft.client.model.geom.ModelPart;
import net.minecraft.util.Mth;
import net.minecraft.world.entity.Entity;

public final class DuckPoseProbe {
    private static final class Sample extends NaturalistEntityModel<Entity> {
        final ModelPart baked;
        final boolean baby;
        final SmoothAnimationState[] states = {SmoothAnimationState.pose(), new SmoothAnimationState(),
            new SmoothAnimationState(), new SmoothAnimationState(), new SmoothAnimationState(), new SmoothAnimationState()};

        Sample(ModelPart root, boolean baby) {
            super(root);
            this.baked = root;
            this.baby = baby;
        }

        @Override
        protected void setupAnimations(Entity ignored, float limbSwing, float speed, float age,
                                       float partial, float headYaw, float pitch) {}

        void tick(int tick, boolean sitting, boolean swimming, boolean moving, boolean sprinting, boolean airborne) {
            var active = new boolean[]{sitting, !sitting && !swimming && !moving, swimming,
                !sitting && !swimming && moving && !sprinting, !sitting && !swimming && moving && sprinting,
                airborne};
            for (int i = 0; i < active.length; ++i) states[i].animateWhen(active[i], tick);
        }

        void frame(int tick, float partial, float speed, float pitch, float yaw) {
            baked.getAllParts().forEach(ModelPart::resetPose);
            float age = tick + partial;
            float reference = (float)(Math.min(8.64D * 0.2D, 1.0D) * 0.65D);
            float gait = Mth.clamp(speed / reference, 0.4F, 2.0F);
            float swim = Mth.clamp(speed / 0.5F, 0.4F, 2.0F);
            if (baby) {
                animateSmooth(states[0], DuckBabyAnimation.DUCK_SIT_IDLE, age, partial);
                animateIdleSmooth(states[1], DuckBabyAnimation.DUCK_IDLE, age, partial, speed);
                animateSmooth(states[2], DuckBabyAnimation.DUCK_SWIM, age, partial, swim);
                animateSmooth(states[3], DuckBabyAnimation.DUCK_WALK, age, partial, 1.5F * gait);
                animateSmooth(states[4], DuckBabyAnimation.DUCK_WALK, age, partial, 2.0F * gait);
            } else {
                animateSmooth(states[0], DuckAnimation.DUCK_SIT, age, partial);
                animateIdleSmooth(states[1], DuckAnimation.DUCK_IDLE, age, partial, speed);
                animateSmooth(states[2], DuckAnimation.DUCK_SWIM, age, partial, swim);
                animateSmooth(states[3], DuckAnimation.DUCK_WALK, age, partial, 1.5F * gait);
                animateSmooth(states[4], DuckAnimation.DUCK_WALK, age, partial, 2.0F * gait);
                animateSmooth(states[5], DuckAnimation.DUCK_FLAP, age, partial);
            }
            var root = baked.getChild("root");
            var lookup = root.createPartLookup();
            applyHeadLook(lookup.apply("neck"), yaw, pitch);
            String[] names = baby ? new String[]{"root", "body", "neck", "bowtie", "leftLeg", "rightLeg"}
                : new String[]{"root", "body", "tail", "neck", "bill", "bowtie", "leftWing", "rightWing",
                    "leftLeg", "leftFoot", "rightLeg", "rightFoot"};
            String bones = Arrays.stream(names).map(name -> {
                var part = name.equals("root") ? root : lookup.apply(name);
                return String.format(Locale.ROOT,
                    "\"%s\":[%.8f,%.8f,%.8f,%.8f,%.8f,%.8f,%.8f,%.8f,%.8f]",
                    name, part.x, part.y, part.z, part.xRot, part.yRot, part.zRot, part.xScale, part.yScale, part.zScale);
            }).reduce((a, b) -> a + "," + b).orElseThrow();
            var weights = Arrays.stream(states)
                .map(state -> String.format(Locale.ROOT, "%.8f", state.factor(partial)))
                .reduce((a, b) -> a + "," + b).orElseThrow();
            var times = Arrays.stream(states)
                .map(state -> state.isStarted() ? Long.toString(state.getAccumulatedTime()) : "null")
                .reduce((a, b) -> a + "," + b).orElseThrow();
            System.out.printf(Locale.ROOT, "{\"tick\":%d,\"partial\":%.2f,\"baby\":%s,\"weights\":[%s],\"time_ms\":[%s],\"bones\":{%s}}%n",
                tick, partial, baby, weights, times, bones);
        }
    }

    public static void main(String[] args) {
        for (boolean baby : new boolean[]{false, true}) {
            var duck = new Sample(baby ? DuckBabyModel.createBodyLayer().bakeRoot() : DuckModel.createBodyLayer().bakeRoot(), baby);
            for (int tick = 1; tick <= 80; ++tick) {
                boolean sitting = 49 <= tick && tick < 67;
                boolean swimming = 26 <= tick && tick < 39;
                boolean sprinting = 18 <= tick && tick < 26;
                boolean airborne = 39 <= tick && tick < 49;
                boolean moving = 9 <= tick && tick < 45;
                float velocity = !moving ? 0.0F : swimming ? 0.31F : sprinting ? 0.48F : 0.22F;
                float pitch = airborne ? 22.0F : swimming ? -12.0F : 4.0F;
                float yaw = sitting ? -16.0F : 8.0F;
                duck.tick(tick, sitting, swimming, moving, sprinting, airborne);
                for (float partial : new float[]{0.0F, 0.5F, 0.9F}) {
                    duck.frame(tick, partial, velocity, pitch, yaw);
                }
            }
        }
    }
}
