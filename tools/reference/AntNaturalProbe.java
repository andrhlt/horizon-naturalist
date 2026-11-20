package naturalist.fixture;

import com.crispytwig.naturalist.world.level.block.AntHillBlock;
import com.crispytwig.naturalist.world.level.block.entity.AntHillBlockEntity;
import com.mojang.brigadier.arguments.IntegerArgumentType;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.registries.Registries;
import net.minecraft.world.level.biome.Biomes;
import net.minecraft.world.level.levelgen.Heightmap;
import net.minecraft.network.chat.Component;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

import java.util.TreeSet;

@Mod("naturalist_fixture")
public final class AntNaturalProbe {
    public AntNaturalProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("ant_natural")
            .then(Commands.argument("x", IntegerArgumentType.integer())
                .then(Commands.argument("z", IntegerArgumentType.integer()).executes(ctx -> {
                    var source = ctx.getSource();
                    var level = source.getLevel();
                    int centerX = IntegerArgumentType.getInteger(ctx, "x");
                    int centerZ = IntegerArgumentType.getInteger(ctx, "z");
                    var entries = new TreeSet<String>();
                    for (int x = (centerX - 64) >> 4; x <= (centerX + 64) >> 4; ++x) {
                        for (int z = (centerZ - 64) >> 4; z <= (centerZ + 64) >> 4; ++z) {
                            var chunk = level.getChunk(x, z);
                            for (int wx = Math.max(x << 4, centerX - 64); wx <= Math.min((x << 4) + 15, centerX + 64); ++wx) {
                                for (int wz = Math.max(z << 4, centerZ - 64); wz <= Math.min((z << 4) + 15, centerZ + 64); ++wz) {
                                    int surface = chunk.getHeight(Heightmap.Types.WORLD_SURFACE, wx & 15, wz & 15);
                                    for (int y = surface; y >= surface - 5; --y) {
                                        var pos = new BlockPos(wx, y, wz);
                                        var state = chunk.getBlockState(pos);
                                        if (!(state.getBlock() instanceof AntHillBlock)) continue;
                                        var entity = chunk.getBlockEntity(pos);
                                        entries.add("[%d,%d,%d,%s,%d,%s,%s]".formatted(wx, y, wz,
                                            state.getValue(AntHillBlock.OPEN), state.getValue(AntHillBlock.WORKERS),
                                            state.getValue(AntHillBlock.HAS_QUEEN),
                                            entity instanceof AntHillBlockEntity hill && hill.getOwner() != null));
                                    }
                                }
                            }
                        }
                    }
                    var registry = level.registryAccess().lookupOrThrow(Registries.BIOME);
                    var biome = registry.getOrThrow(Biomes.SAVANNA).value();
                    var features = new TreeSet<String>();
                    for (var step : biome.getGenerationSettings().features()) {
                        for (var feature : step) {
                            var key = feature.unwrapKey().toString();
                            if (key.contains("naturalist:")) features.add("\"" + key + "\"");
                        }
                    }
                    String result = "{\"seed\":" + level.getSeed() + ",\"center\":[" + centerX + "," + centerZ
                        + "],\"features\":[" + String.join(",", features) + "],\"blocks\":[" + String.join(",", entries) + "]}";
                    source.sendSuccess(() -> Component.literal("ant_natural " + result), false);
                    return entries.size();
                }))));
    }
}
