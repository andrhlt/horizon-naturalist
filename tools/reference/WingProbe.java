package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.dragonfly.Dragonfly;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.EntitySpawnReason;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class WingProbe {
    public WingProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("insect_fixture_wings").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var dragonfly = NaturalistEntityTypes.DRAGONFLY.get().create(level, EntitySpawnReason.COMMAND);
            var firefly = NaturalistEntityTypes.FIREFLY.get().create(level, EntitySpawnReason.COMMAND);
            if (dragonfly == null || firefly == null) throw new IllegalStateException("Missing insect");
            dragonfly.snapTo(8.5, 95.0, 8.5, 0, 0);
            dragonfly.getRandom().setSeed(859832);
            int initialHover = dragonfly.getHoverTicks();
            int firstPause;
            boolean stillAt30;
            boolean movesAt46;
            try {
                var step = Dragonfly.class.getDeclaredMethod("customServerAiStep", ServerLevel.class);
                step.setAccessible(true);
                for (int i = 0; i < 30; i++) step.invoke(dragonfly, level);
                stillAt30 = dragonfly.getDeltaMovement().lengthSqr() == 0.0;
                step.invoke(dragonfly, level);
                firstPause = dragonfly.getHoverTicks();
                for (int i = 0; i < 15; i++) step.invoke(dragonfly, level);
                movesAt46 = dragonfly.getDeltaMovement().lengthSqr() > 0.0;
            } catch (ReflectiveOperationException cause) {
                throw new IllegalStateException(cause);
            }
            firefly.snapTo(10.5, 94.0, 10.5, 0, 0);
            firefly.setNoAi(true);
            firefly.getRandom().setSeed(1500000);
            for (int i = 0; i < 4000 && !firefly.isGlowing(); i++) firefly.tick();
            int glow = firefly.getGlowTicksRemaining();
            var result = ("{\"dragonfly_initial_hover\":%d,\"dragonfly_still_after_30\":%s,"
                        + "\"dragonfly_waypoint_pause\":%d,\"dragonfly_moves_at_46\":%s,"
                        + "\"dragonfly_no_gravity\":%s,\"firefly_glow_40_to_59\":%s,"
                        + "\"firefly_first_luminance\":%d}")
                .formatted(initialHover, stillAt30, firstPause, movesAt46, dragonfly.isNoGravity(),
                           glow >= 40 && glow <= 59, firefly.getGlowLuminance());
            source.sendSuccess(() -> Component.literal("insect_fixture_wings " + result), false);
            return 1;
        }));
    }
}
