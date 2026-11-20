package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.hippo.Hippo;
import com.google.gson.Gson;
import java.util.LinkedHashMap;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.properties.BlockStateProperties;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class HippoSpawnProbe {
    public HippoSpawnProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("hippo_spawn_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var origin = new BlockPos(8, 132, 8);
            var solid = origin.below();
            level.setBlockAndUpdate(solid, Blocks.GRASS_BLOCK.defaultBlockState());
            var cases = new LinkedHashMap<String, Boolean>();
            cases.put("no_water", test(level, origin, EntitySpawnReason.NATURAL));
            var water = origin.offset(16, 0, 16);
            level.setBlockAndUpdate(water, Blocks.WATER.defaultBlockState());
            cases.put("natural_corner_16", test(level, origin, EntitySpawnReason.NATURAL));
            cases.put("generated_cross_chunk", test(level, origin, EntitySpawnReason.CHUNK_GENERATION));
            level.setBlockAndUpdate(water, Blocks.AIR.defaultBlockState());

            water = origin.offset(17, 0, 0);
            level.setBlockAndUpdate(water, Blocks.WATER.defaultBlockState());
            cases.put("horizontal_17", test(level, origin, EntitySpawnReason.NATURAL));
            level.setBlockAndUpdate(water, Blocks.AIR.defaultBlockState());
            water = origin.offset(3, 2, 0);
            level.setBlockAndUpdate(water, Blocks.WATER.defaultBlockState());
            cases.put("two_above", test(level, origin, EntitySpawnReason.NATURAL));
            level.setBlockAndUpdate(water, Blocks.AIR.defaultBlockState());
            water = origin.offset(3, -2, 0);
            level.setBlockAndUpdate(water, Blocks.WATER.defaultBlockState());
            cases.put("two_below", test(level, origin, EntitySpawnReason.NATURAL));
            level.setBlockAndUpdate(water, Blocks.AIR.defaultBlockState());

            water = origin.offset(7, -1, 7);
            level.setBlockAndUpdate(water, Blocks.OAK_SLAB.defaultBlockState().setValue(BlockStateProperties.WATERLOGGED, true));
            cases.put("waterlogged_one_below", test(level, origin, EntitySpawnReason.NATURAL));
            cases.put("generated_same_chunk", test(level, origin, EntitySpawnReason.CHUNK_GENERATION));
            level.setBlockAndUpdate(solid, Blocks.STONE.defaultBlockState());
            cases.put("stone_rejected", test(level, origin, EntitySpawnReason.NATURAL));

            String result = new Gson().toJson(cases);
            source.sendSuccess(() -> Component.literal("hippo_spawn_fixture " + result), false);
            return 1;
        }));
    }

    private static boolean test(net.minecraft.server.level.ServerLevel level, BlockPos pos, EntitySpawnReason reason) {
        return Hippo.checkHippoSpawnRules(NaturalistEntityTypes.HIPPO.get(), level, reason, pos, level.getRandom());
    }
}
