package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistMobVariants;
import com.google.gson.Gson;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.core.Holder;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.core.registries.Registries;
import net.minecraft.network.chat.Component;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class MobVariantOrderProbe {
    public MobVariantOrderProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_variant_order").executes(context -> {
            var source = context.getSource();
            var registries = source.getLevel().registryAccess();
            var biomes = registries.lookupOrThrow(Registries.BIOME).listElements().toList();
            var gson = new Gson();
            BuiltInRegistries.ENTITY_TYPE.keySet().stream()
                .filter(id -> id.getNamespace().equals("naturalist"))
                .sorted().forEach(id -> registries.lookup(NaturalistMobVariants.registryFor(id.getPath())).ifPresent(registry -> {
                    var variants = registry.listElements().toList();
                    var ordered = variants.stream().map(variant -> variant.key().identifier().toString()).toList();
                    Map<String, List<List<Object>>> pools = new LinkedHashMap<>();
                    for (var biome : biomes) {
                        var pool = variants.stream().filter(variant -> variant.value().weight() > 0)
                            .filter(variant -> variant.value().biomes().filter(set -> set.contains(biome)).isPresent()).toList();
                        if (pool.isEmpty()) {
                            pool = variants.stream().filter(variant -> variant.value().weight() > 0)
                                .filter(variant -> variant.value().biomes().isEmpty()).toList();
                        }
                        int top = pool.stream().mapToInt(variant -> variant.value().priority()).max().orElse(0);
                        var rows = pool.stream().filter(variant -> variant.value().priority() == top)
                            .map(variant -> List.<Object>of(variant.key().identifier().toString(), variant.value().weight()))
                            .toList();
                        if (!rows.isEmpty()) pools.put(biome.key().identifier().toString(), rows);
                    }
                    var report = Map.of("species", id.toString(), "order", ordered, "pools", pools);
                    source.sendSuccess(() -> Component.literal("naturalist_variant_order " + gson.toJson(report)), false);
                }));
            return 1;
        }));
    }
}
