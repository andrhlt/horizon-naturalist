package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.vulture.Vulture;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.ai.goal.MoveToBlockGoal;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class VulturePerchProbe {
    public VulturePerchProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("vulture_perch_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var first = NaturalistEntityTypes.VULTURE.get().create(level, EntitySpawnReason.COMMAND);
            var second = NaturalistEntityTypes.VULTURE.get().create(level, EntitySpawnReason.COMMAND);
            if (first == null || second == null) throw new IllegalStateException("Missing vulture");
            first.snapTo(8.6, 95, 8.5, 0, 0);
            second.snapTo(9.0, 95, 8.5, 0, 0);
            level.addFreshEntity(first);
            level.addFreshEntity(second);
            try {
                Class<?> type = Class.forName(Vulture.class.getName() + "$VulturePerchOnCactusGoal");
                var ctor = type.getDeclaredConstructor(Vulture.class, double.class, int.class, int.class);
                ctor.setAccessible(true);
                var goal = (MoveToBlockGoal)ctor.newInstance(first, 1.0, 32, 32);
                var other = (MoveToBlockGoal)ctor.newInstance(second, 1.0, 32, 32);
                var delay = MoveToBlockGoal.class.getDeclaredField("nextStartTick");
                delay.setAccessible(true);
                delay.setInt(goal, 0);
                delay.setInt(other, 0);
                boolean findsCactus = goal.canUse();
                goal.start();
                goal.tick();
                var worldPosition = new Vec3(8.5, 95, 8.5);
                boolean exact = first.isPerched() && first.position().equals(worldPosition)
                    && first.getPerchTarget().equals(new BlockPos(8, 94, 8));
                boolean reservedAgainstSecond = !other.canUse();
                for (int i = 1; i < 300; i++) goal.tick();
                boolean holdsFullDuration = !goal.canContinueToUse() && first.isPerched();
                goal.stop();
                boolean released = !first.isPerched() && first.getPerchTarget() == null;
                first.startle(null);
                var cooldown = Vulture.class.getDeclaredField("perchCooldown");
                cooldown.setAccessible(true);
                boolean startledCooldown = !first.canPerch() && cooldown.getInt(first) == 200;
                String report = ("{\"finds_cactus\":%s,\"snaps_to_top\":%s,\"reserves_against_second\":%s,"
                    + "\"holds_300_goal_ticks\":%s,\"releases_reservation\":%s,\"startle_cooldown_200\":%s}")
                    .formatted(findsCactus, exact, reservedAgainstSecond, holdsFullDuration, released, startledCooldown);
                source.sendSuccess(() -> Component.literal("vulture_perch_fixture " + report), false);
                return 1;
            } catch (ReflectiveOperationException error) {
                throw new IllegalStateException(error);
            } finally {
                first.discard();
                second.discard();
            }
        }));
    }
}
