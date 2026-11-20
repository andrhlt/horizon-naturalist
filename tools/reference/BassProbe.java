package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class BassProbe {
    public BassProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("bass_fixture_hunt").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var predator = NaturalistEntityTypes.BASS.get().create(level, EntitySpawnReason.COMMAND);
            var prey = NaturalistEntityTypes.BASS.get().create(level, EntitySpawnReason.COMMAND);
            if (predator == null || prey == null) throw new IllegalStateException("Bass did not register");
            predator.snapTo(8.0, 93.0, 8.0, 0.0f, 0.0f);
            prey.snapTo(8.5, 93.0, 8.0, 0.0f, 0.0f);
            predator.setVariantString("naturalist:bass_large");
            boolean canHunt = predator.canHunt() && predator.canEatTarget(prey);
            boolean hit = predator.doHurtTarget(level, prey);
            var result = ("{\"can_hunt\":%s,\"discards_prey\":%s,\"can_hunt_after\":%s," +
                          "\"variant\":\"%s\"}").formatted(canHunt, hit && prey.isRemoved(),
                                                          predator.canHunt(), predator.getVariantString());
            source.sendSuccess(() -> Component.literal("bass_fixture_hunt " + result), false);
            return 1;
        }));
    }
}
