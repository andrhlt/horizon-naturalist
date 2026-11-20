package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import java.util.Locale;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class CaterpillarClimbProbe {
    public CaterpillarClimbProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("caterpillar_climb_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var larva = NaturalistEntityTypes.CATERPILLAR.get().create(level, EntitySpawnReason.COMMAND);
            if (larva == null) throw new IllegalStateException("Missing fixture Caterpillar");
            double adultWidth = larva.getBbWidth();
            larva.setBaby(true);
            var base = new Vec3(7.6, 93, 8.5);
            larva.snapTo(base.x, base.y, base.z, 0, 0);
            larva.setPersistenceRequired();
            level.addFreshEntity(larva);
            try {
                boolean climbed = false;
                for (int i = 0; i < 8; i++) {
                    var velocity = larva.getDeltaMovement();
                    larva.setDeltaMovement(0.4, velocity.y, 0);
                    larva.tick();
                    climbed |= larva.isClimbing();
                }
                String json = String.format(Locale.ROOT,
                    "{\"adult_width\":%.6f,\"baby_width\":%.6f,\"climbing\":%s,\"rise\":%.6f,\"can_portal\":%s}",
                    adultWidth, larva.getBbWidth(), climbed, larva.getY() - base.y, larva.canUsePortal(false));
                source.sendSuccess(() -> Component.literal("caterpillar_climb_fixture " + json), false);
                return 1;
            } finally {
                larva.discard();
            }
        }));
    }
}
