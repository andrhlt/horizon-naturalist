package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.mojang.brigadier.arguments.IntegerArgumentType;
import java.util.Map;
import java.util.TreeMap;
import java.util.stream.Collectors;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class BirdVariantProbe {
    public BirdVariantProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("bird_variant_fixture")
            .then(Commands.argument("count", IntegerArgumentType.integer(1, 1000)).executes(ctx -> {
                var source = ctx.getSource();
                var level = source.getLevel();
                int count = IntegerArgumentType.getInteger(ctx, "count");
                var pool = new TreeMap<String, Integer>();
                for (int i = 0; i < count; i++) {
                    var bird = NaturalistEntityTypes.BIRD.get().create(level, EntitySpawnReason.NATURAL);
                    if (bird == null) throw new IllegalStateException("Missing bird");
                    bird.snapTo(8.5, 93.0, 8.5, 0, 0);
                    bird.finalizeSpawn(level, level.getCurrentDifficultyAt(new BlockPos(8, 93, 8)),
                        EntitySpawnReason.NATURAL, null);
                    pool.merge(bird.getVariantString(), 1, Integer::sum);
                }
                String report = pool.entrySet().stream().map(BirdVariantProbe::field).collect(Collectors.joining(","));
                source.sendSuccess(() -> Component.literal("bird_variant_fixture {" + report + "}"), false);
                return 1;
            })));
    }

    private static String field(Map.Entry<String, Integer> entry) {
        return "\"" + entry.getKey() + "\":" + entry.getValue();
    }
}
