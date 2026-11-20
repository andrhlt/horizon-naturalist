package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import java.util.ArrayList;
import java.util.Locale;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class WingDescentProbe {
    public WingDescentProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("wing_descent_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var measurements = new ArrayList<String>();
            for (var type : java.util.List.of(NaturalistEntityTypes.BUTTERFLY.get(),
                NaturalistEntityTypes.DRAGONFLY.get(), NaturalistEntityTypes.FIREFLY.get(),
                NaturalistEntityTypes.VULTURE.get(), NaturalistEntityTypes.BIRD.get())) {
                var mob = type.create(level, EntitySpawnReason.COMMAND);
                if (mob == null) throw new IllegalStateException("Missing fixture flyer");
                mob.getRandom().setSeed(5L);
                mob.snapTo(8.5, 100.0, 8.5, 0, 0);
                mob.setPersistenceRequired();
                mob.setNoGravity(true);
                mob.setDeltaMovement(0, -0.2, 0);
                level.addFreshEntity(mob);
                try {
                    mob.tick();
                    measurements.add(String.format(Locale.ROOT, "\"%s\":{\"rise\":%.9f,\"velocity\":%.9f}",
                        type.toShortString(), mob.getY() - 100.0, mob.getDeltaMovement().y));
                } finally {
                    mob.discard();
                }
            }
            var json = "{" + String.join(",", measurements) + "}";
            source.sendSuccess(() -> Component.literal("wing_descent_fixture " + json), false);
            return 1;
        }));
    }
}
