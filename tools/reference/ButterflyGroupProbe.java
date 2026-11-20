package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.butterfly.Butterfly;
import java.util.HashSet;
import java.util.List;
import java.util.TreeSet;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.SpawnGroupData;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class ButterflyGroupProbe {
    public ButterflyGroupProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("butterfly_group_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var types = new TreeSet<String>();
            boolean repeatsPairs = false, mixesPairs = false, adult = true, share = true, onlyPair = true;
            var type = NaturalistEntityTypes.BUTTERFLY.get();
            for (int sample = 0; sample < 128; sample++) {
                SpawnGroupData group = null;
                for (int member = 0; member < 6; member++) {
                    var mob = type.create(level, EntitySpawnReason.NATURAL);
                    if (mob == null) throw new IllegalStateException("Missing fixture butterfly");
                    mob.snapTo(8.5 + member, 93, 8.5, 0, 0);
                    var current = mob.finalizeSpawn(level, level.getCurrentDifficultyAt(mob.blockPosition()), EntitySpawnReason.NATURAL, group);
                    if (group == null) group = current;
                    share &= current == group;
                    var pool = ((Butterfly.ButterflyGroupData)current).types;
                    var colours = new HashSet<>(List.of(pool));
                    repeatsPairs |= colours.size() == 1;
                    mixesPairs |= colours.size() == 2;
                    onlyPair &= pool.length == 2 && colours.contains(mob.getVariantString());
                    adult &= !mob.isBaby();
                    types.add(mob.getVariantString());
                    mob.discard();
                }
            }
            String variants = "[\"" + String.join("\",\"", types) + "\"]";
            String json = "{\"all_adult\":" + adult + ",\"shares_pool\":" + share + ",\"two_draws_including_duplicates\":" + (repeatsPairs && mixesPairs)
                + ",\"in_pool\":" + onlyPair + ",\"observed_variants\":" + variants + "}";
            source.sendSuccess(() -> Component.literal("butterfly_group_fixture " + json), false);
            return 1;
        }));
    }
}
