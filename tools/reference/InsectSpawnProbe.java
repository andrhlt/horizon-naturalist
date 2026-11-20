package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.butterfly.Butterfly;
import com.crispytwig.naturalist.world.entity.animal.dragonfly.Dragonfly;
import com.crispytwig.naturalist.world.entity.animal.firefly.Firefly;
import java.util.ArrayList;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.util.RandomSource;
import net.minecraft.world.entity.EntitySpawnReason;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class InsectSpawnProbe {
    public InsectSpawnProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("insect_spawn_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var rows = new ArrayList<String>();
            String[] names = {"open_mud", "open_stone", "dark_grass", "dark_stone"};
            BlockPos[] sites = {new BlockPos(2, 93, 2), new BlockPos(11, 93, 2), new BlockPos(2, 93, 34), new BlockPos(11, 93, 34)};
            for (int i = 0; i < names.length; i++) {
                var pos = sites[i];
                int butterfly = 0, dragonfly = 0, firefly = 0;
                for (int seed = 0; seed < 64; seed++) {
                    if (Butterfly.checkButterflySpawnRules(NaturalistEntityTypes.BUTTERFLY.get(), level,
                        EntitySpawnReason.NATURAL, pos, RandomSource.create(seed))) butterfly++;
                    if (Dragonfly.checkDragonflySpawnRules(NaturalistEntityTypes.DRAGONFLY.get(), level,
                        EntitySpawnReason.NATURAL, pos, RandomSource.create(seed))) dragonfly++;
                    if (Firefly.checkFireflySpawnRules(NaturalistEntityTypes.FIREFLY.get(), level,
                        EntitySpawnReason.NATURAL, pos, RandomSource.create(seed))) firefly++;
                }
                rows.add("\"" + names[i] + "\":{\"butterfly\":" + butterfly + ",\"dragonfly\":" + dragonfly
                    + ",\"firefly\":" + firefly + ",\"raw_light\":" + level.getRawBrightness(pos, 0) + "}");
            }
            String json = "{" + String.join(",", rows) + "}";
            source.sendSuccess(() -> Component.literal("insect_spawn_fixture " + json), false);
            return 1;
        }));
    }
}
