package naturalist.fixture;

import com.crispytwig.naturalist.NaturalistConfig;
import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.level.NaturalistSpawns;
import com.google.gson.Gson;
import java.util.List;
import java.util.TreeMap;
import net.minecraft.commands.Commands;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.core.registries.Registries;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;
import net.minecraft.resources.ResourceKey;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.MobCategory;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class AquaticSpawnProbe {
    private static final Gson GSON = new Gson();

    public AquaticSpawnProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("aquatic_spawn_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var biomes = level.registryAccess().lookupOrThrow(Registries.BIOME);
            var types = List.of(NaturalistEntityTypes.BASS.get(), NaturalistEntityTypes.PIRANHA.get(),
                NaturalistEntityTypes.CATFISH.get(), NaturalistEntityTypes.BLOBFISH.get(),
                NaturalistEntityTypes.JELLYFISH.get(), NaturalistEntityTypes.RAY.get(), NaturalistEntityTypes.ANGLERFISH.get());
            var caps = new TreeMap<String, Integer>();
            for (var type : types) {
                var fish = (Mob) type.create(level, EntitySpawnReason.NATURAL);
                caps.put(BuiltInRegistries.ENTITY_TYPE.getKey(type).toString(), fish.getMaxSpawnClusterSize());
            }
            var tables = new TreeMap<String, TreeMap<String, List<Integer>>>();
            for (var name : List.of("ocean", "warm_ocean", "frozen_ocean", "deep_frozen_ocean", "river", "swamp", "jungle")) {
                var biome = biomes.getOrThrow(ResourceKey.create(Registries.BIOME, Identifier.parse("minecraft:" + name)));
                var table = new TreeMap<String, List<Integer>>();
                NaturalistSpawns.forEachSpawn((hasTag, blacklistTag, category, type, weight, min, max) -> {
                    if (category == MobCategory.WATER_AMBIENT && types.contains(type) && !NaturalistConfig.isRemoved(type)
                        && biome.is(hasTag) && (blacklistTag == null || !biome.is(blacklistTag))) {
                        table.put(BuiltInRegistries.ENTITY_TYPE.getKey(type).toString(), List.of(weight, min, max));
                    }
                });
                tables.put(name, table);
            }
            String json = "{\"cluster_limits\":" + GSON.toJson(caps) + ",\"biomes\":" + GSON.toJson(tables) + "}";
            source.sendSuccess(() -> Component.literal("aquatic_spawn_fixture " + json), false);
            return 1;
        }));
    }
}
