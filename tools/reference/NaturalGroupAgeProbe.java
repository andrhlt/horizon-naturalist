package naturalist.fixture;

import com.google.gson.Gson;
import java.util.List;
import java.util.TreeMap;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.AgeableMob;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.SpawnGroupData;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class NaturalGroupAgeProbe {
    private static final int TRIALS = 128;

    public NaturalGroupAgeProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("natural_group_age_fixture").executes(ctx -> {
            var level = ctx.getSource().getLevel();
            var difficulty = level.getCurrentDifficultyAt(new BlockPos(8, 100, 8));
            var result = new TreeMap<String, List<Integer>>();
            for (var type : BuiltInRegistries.ENTITY_TYPE) {
                var id = BuiltInRegistries.ENTITY_TYPE.getKey(type);
                if (!id.getNamespace().equals("naturalist")
                        || !(type.create(level, EntitySpawnReason.NATURAL) instanceof AgeableMob)) {
                    continue;
                }
                int[] babies = new int[4];
                for (int i = 0; i < TRIALS; i++) {
                    level.getRandom().setSeed(7845123L + i * 104729L);
                    SpawnGroupData data = null;
                    for (int member = 0; member < babies.length; member++) {
                        var mob = (AgeableMob) type.create(level, EntitySpawnReason.NATURAL);
                        mob.snapTo(8.5, 100, 8.5, 0, 0);
                        data = mob.finalizeSpawn(level, difficulty, EntitySpawnReason.NATURAL, data);
                        if (mob.isBaby()) babies[member]++;
                        mob.discard();
                    }
                }
                result.put(id.toString(), List.of(babies[0], babies[1], babies[2], babies[3]));
            }
            var output = new Gson().toJson(result);
            ctx.getSource().sendSuccess(() -> Component.literal("natural_group_age_fixture " + output), false);
            return 1;
        }));
    }
}
