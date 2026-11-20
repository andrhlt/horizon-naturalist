package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import java.util.Locale;
import java.util.ArrayList;
import java.util.Set;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class SnailOrientationProbe {
    public SnailOrientationProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("snail_orientation_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var snail = NaturalistEntityTypes.SNAIL.get().create(level, EntitySpawnReason.COMMAND);
            if (snail == null) throw new IllegalStateException("Missing fixture Snail");
            snail.snapTo(7.2, 93.0, 8.5, 0, 0);
            level.addFreshEntity(snail);
            var crawler = snail.getClimbing();
            var samples = new ArrayList<String>();
            var sampleTicks = Set.of(1, 5, 7, 8, 10, 20, 46, 47, 48, 50, 57);
            var clientTick = crawler.getClass().getDeclaredMethods();
            var update = java.util.Arrays.stream(clientTick).filter(m -> m.getName().equals("tickClient")).findFirst().orElseThrow();
            update.setAccessible(true);
            int wallTick = -1, ceilingTick = -1;
            double wallY = 0, ceilingY = 0;
            try {
                for (int tick = 1; tick <= 800; tick++) {
                    snail.setOldPosAndRot();
                    Vec3 direction = wallTick < 0 ? new Vec3(1, 0, 0) : new Vec3(0, 1, 0);
                    crawler.setDesired(direction, 0.22, direction);
                    if (!crawler.travel()) break;
                    crawler.tick();
                    update.invoke(crawler);
                    Vec3 normal = crawler.getNormal();
                    if (sampleTicks.contains(tick)) {
                        for (float partial : new float[]{0.0F, 0.5F, 0.9F}) {
                            var renderNormal = crawler.getRenderNormal(partial);
                            var forward = crawler.getRenderForwardFlattened(partial, renderNormal);
                            var anchor = crawler.getRenderAnchor(partial);
                            samples.add(String.format(Locale.ROOT, "{\"tick\":%d,\"partial\":%.2f,\"normal\":%s,\"forward\":%s,\"anchor\":%s,\"tail_lag\":%.8f}",
                                tick, partial, vector(renderNormal), vector(forward), vector(anchor), crawler.getTailLag(partial)));
                        }
                    }
                    if (wallTick < 0 && normal.x == -1) {
                        wallTick = tick;
                        wallY = snail.getY();
                    }
                    if (normal.y == -1 && ceilingTick < 0) {
                        ceilingTick = tick;
                        ceilingY = snail.getY();
                    }
                    if (tick == 57) break;
                }
                String json = "{\"wall_tick\":" + wallTick + ",\"ceiling_tick\":" + ceilingTick
                    + ",\"poses\":[" + String.join(",", samples) + "]}";
                source.sendSuccess(() -> Component.literal("snail_orientation_fixture " + json), false);
                return 1;
            } catch (ReflectiveOperationException exception) {
                throw new IllegalStateException(exception);
            } finally {
                snail.discard();
            }
        }));
    }
    private static String vector(Vec3 value) {
        return String.format(Locale.ROOT, "[%.8f,%.8f,%.8f]", value.x, value.y, value.z);
    }
}
