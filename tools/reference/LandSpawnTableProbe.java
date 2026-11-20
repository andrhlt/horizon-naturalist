package naturalist.fixture;

import com.google.gson.Gson;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;
import java.util.Set;
import java.util.TreeMap;
import net.minecraft.commands.Commands;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.core.registries.Registries;
import net.minecraft.network.chat.Component;
import net.minecraft.world.attribute.EnvironmentAttributes;
import net.minecraft.world.entity.MobCategory;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class LandSpawnTableProbe {
    private static final Set<String> TYPES = Set.of(
            "minecraft:cow", "minecraft:pig", "minecraft:sheep", "minecraft:chicken", "minecraft:fox", "minecraft:rabbit");
    private static final Gson GSON = new Gson();

    public LandSpawnTableProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("land_spawn_table_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var biomes = source.getLevel().registryAccess().lookupOrThrow(Registries.BIOME);
            var attribute = EnvironmentAttributes.NATURAL_MOB_SPAWNS;
            var result = new TreeMap<String, List<List<Object>>>();
            for (var biome : biomes.listElements().toList()) {
                var name = biome.key().identifier();
                if (!name.getNamespace().equals("minecraft")) continue;
                var entry = biome.value().getModifiedEnvironmentAttributes().get(attribute);
                var table = entry == null ? attribute.defaultValue() : entry.applyModifier(attribute.defaultValue());
                var rows = new ArrayList<List<Object>>();
                for (MobCategory category : MobCategory.values()) {
                    for (var weighted : table.getMobsToSpawn(category).unwrap()) {
                        var spawn = weighted.value();
                        String id = BuiltInRegistries.ENTITY_TYPE.getKey(spawn.type()).toString();
                        if (id.startsWith("naturalist:") || TYPES.contains(id)) {
                            rows.add(List.of(category.name().toLowerCase(Locale.ROOT), id, weighted.weight(),
                                    spawn.count().minInclusive(), spawn.count().maxInclusive()));
                        }
                    }
                }
                result.put(name.toString(), rows);
            }
            var output = GSON.toJson(result);
            source.sendSuccess(() -> Component.literal("land_spawn_table_fixture " + output), false);
            return 1;
        }));
    }
}
