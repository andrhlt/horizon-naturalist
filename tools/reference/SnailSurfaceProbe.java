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
public final class SnailSurfaceProbe {
    public SnailSurfaceProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("snail_surface_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var snail = NaturalistEntityTypes.SNAIL.get().create(level, EntitySpawnReason.COMMAND);
            if (snail == null) throw new IllegalStateException("Missing fixture Snail");
            snail.snapTo(7.2, 93.0, 8.5, 0, 0);
            level.addFreshEntity(snail);
            var crawler = snail.getClimbing();
            int wallTick = -1, ceilingTick = -1;
            double wallY = 0, ceilingY = 0;
            try {
                for (int tick = 1; tick <= 800; tick++) {
                    Vec3 direction = wallTick < 0 ? new Vec3(1, 0, 0) : new Vec3(0, 1, 0);
                    crawler.setDesired(direction, 0.22, direction);
                    if (!crawler.travel()) break;
                    crawler.tick();
                    Vec3 normal = crawler.getNormal();
                    if (wallTick < 0 && normal.x == -1) {
                        wallTick = tick;
                        wallY = snail.getY();
                    }
                    if (normal.y == -1) {
                        ceilingTick = tick;
                        ceilingY = snail.getY();
                        break;
                    }
                }
                String json = String.format(Locale.ROOT,
                    "{\"wall_tick\":%d,\"wall_y\":%.6f,\"ceiling_tick\":%d,\"ceiling_y\":%.6f,\"attached\":%s}",
                    wallTick, wallY, ceilingTick, ceilingY, crawler.isAttached());
                source.sendSuccess(() -> Component.literal("snail_surface_fixture " + json), false);
                return 1;
            } finally {
                snail.discard();
            }
        }));
    }
}
