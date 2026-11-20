package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistMobVariants;
import com.crispytwig.naturalist.world.entity.variant.MobVariantUtil;
import com.google.gson.Gson;
import com.mojang.brigadier.arguments.LongArgumentType;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class VariantRngProbe {
    private static final String[] SPECIES = {"bird", "deer", "bass", "crab", "lizard", "dragonfly", "blobfish"};

    public VariantRngProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_variant_rng")
            .then(Commands.argument("seed", LongArgumentType.longArg()).executes(context -> {
                var source = context.getSource();
                var level = source.getLevel();
                var random = level.getRandom();
                var seed = LongArgumentType.getLong(context, "seed");
                var pos = new BlockPos(8, 93, 8);
                random.setSeed(seed);
                var choices = new ArrayList<List<String>>();
                for (int i = 0; i < 84; i++) {
                    String species = SPECIES[i % SPECIES.length];
                    var variant = MobVariantUtil.selectVariantForSpawn(level, pos, NaturalistMobVariants.registryFor(species));
                    choices.add(List.of(species, variant.map(holder -> holder.key().identifier().toString()).orElse("")));
                }
                var sentinel = random.nextInt(100_000);
                var report = new Gson().toJson(Map.of("random_type", random.getClass().getSimpleName(),
                    "sentinel", sentinel, "choices", choices,
                    "biome", level.getBiome(pos).unwrapKey().orElseThrow().identifier().toString()));
                source.sendSuccess(() -> Component.literal("naturalist_variant_rng " + report), false);
                return 1;
            })));
    }
}
