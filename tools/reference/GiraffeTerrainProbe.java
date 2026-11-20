package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.google.gson.Gson;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.level.block.Blocks;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class GiraffeTerrainProbe {
    public GiraffeTerrainProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_giraffe_terrain").executes(context -> {
            var source = context.getSource();
            var level = source.getLevel();
            for (int x = 6; x <= 10; x++) {
                for (int z = 6; z <= 10; z++) {
                    level.setBlockAndUpdate(new BlockPos(x, 98, z), Blocks.STONE.defaultBlockState());
                    level.setBlockAndUpdate(new BlockPos(x, 99, z), Blocks.AIR.defaultBlockState());
                    level.setBlockAndUpdate(new BlockPos(x, 100, z), Blocks.AIR.defaultBlockState());
                }
            }
            level.setBlockAndUpdate(new BlockPos(8, 99, 8), Blocks.STONE.defaultBlockState());
            level.setBlockAndUpdate(new BlockPos(7, 99, 8), Blocks.STONE_SLAB.defaultBlockState());
            level.setBlockAndUpdate(new BlockPos(7, 99, 7), Blocks.STONE.defaultBlockState());

            var results = new ArrayList<Map<String, Object>>();
            for (var age : new int[]{0, -24000}) {
                for (float yaw : new float[]{0f, 45f, 90f}) {
                    var giraffe = NaturalistEntityTypes.GIRAFFE.get().create(level, EntitySpawnReason.COMMAND);
                    giraffe.snapTo(8.15, 100.0, 8.15, yaw, 0f);
                    giraffe.setAge(age);
                    giraffe.setOnGround(true);
                    for (int tick = 1; tick <= 8; tick++) {
                        giraffe.legSolver.update(giraffe, yaw, age < 0 ? 0.5F : 1.0F);
                        var heights = Arrays.stream(giraffe.legSolver.legs).map(leg -> leg.getHeight(0.5F)).toList();
                        var back = (heights.get(0) + heights.get(1)) * 0.5F;
                        var front = (heights.get(2) + heights.get(3)) * 0.5F;
                        var left = (heights.get(0) + heights.get(2)) * 0.5F;
                        var right = (heights.get(1) + heights.get(3)) * 0.5F;
                        results.add(Map.of("age", age, "yaw", yaw, "tick", tick, "feet", heights,
                            "body", List.of((front + back) * 8F, (float)Math.atan2(front - back, 1.5F),
                                            (float)Math.atan2(left - right, 0.8F) * 0.5F)));
                    }
                }
            }
            source.sendSuccess(() -> Component.literal("naturalist_giraffe_terrain " + new Gson().toJson(results)), false);
            return 1;
        }));
    }
}
