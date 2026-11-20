package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.EquipmentSlot;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.context.BlockPlaceContext;
import net.minecraft.world.level.block.SoundType;
import net.minecraft.world.level.block.state.properties.BlockStateProperties;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.Vec3;
import net.minecraft.world.phys.shapes.CollisionContext;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class PlushAndDiscProbe {
    private static final BlockPos SUPPORT = new BlockPos(8, 101, 8);

    public PlushAndDiscProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("plush_and_disc_probe").executes(context -> {
            var source = context.getSource();
            var result = Map.of("plush", plush(source.getLevel()), "disc", disc(source.getLevel()));
            source.sendSuccess(() -> Component.literal("plush_and_disc_probe " + new Gson().toJson(result)), false);
            return 1;
        }));
    }

    private static Map<String, Object> plush(ServerLevel level) {
        var block = NaturalistRegistry.PLUSH_BEAR.get();
        var player = new ServerPlayer(level.getServer(), level,
                new GameProfile(UUID.randomUUID(), "NaturalistFixture"), ClientInformation.createDefault());
        var shape = CollisionContext.empty();
        var output = new LinkedHashMap<String, Object>();
        var placed = new LinkedHashMap<String, Object>();
        for (int yaw : new int[]{0,90,180,270}) {
            player.snapTo(8.5, 102, 7.0, yaw, 0);
            var hit = new BlockHitResult(new Vec3(8.5, 102, 8.5), Direction.UP, SUPPORT, false);
            var context = new BlockPlaceContext(player, InteractionHand.MAIN_HAND, new ItemStack(block), hit);
            var state = block.getStateForPlacement(context);
            var pos = SUPPORT.above();
            placed.put(Integer.toString(yaw), List.of(
                    state.getValue(BlockStateProperties.HORIZONTAL_FACING).getName(),
                    pixels(state.getShape(level, pos, shape).bounds()),
                    pixels(state.getCollisionShape(level, pos, shape).bounds())));
        }
        var defaultState = block.defaultBlockState();
        output.put("placed", placed);
        output.put("default", defaultState.getValue(BlockStateProperties.HORIZONTAL_FACING).getName());
        output.put("hardness", defaultState.getDestroySpeed(level, SUPPORT.above()));
        output.put("resistance", block.getExplosionResistance());
        output.put("occludes", defaultState.canOcclude());
        output.put("requires_tool", defaultState.requiresCorrectToolForDrops());
        output.put("wool_sound", defaultState.getSoundType() == SoundType.WOOL);
        return output;
    }

    private static int[] pixels(AABB bounds) {
        return new int[]{(int)(16 * bounds.minX), (int)(16 * bounds.minY), (int)(16 * bounds.minZ),
                (int)(16 * bounds.maxX), (int)(16 * bounds.maxY), (int)(16 * bounds.maxZ)};
    }

    private static Map<String, Object> disc(ServerLevel level) {
        var res = new LinkedHashMap<String, Object>();
        var difficulty = level.getCurrentDifficultyAt(SUPPORT);
        for (var reason : List.of(EntitySpawnReason.NATURAL, EntitySpawnReason.CHUNK_GENERATION,
                EntitySpawnReason.SPAWN_ITEM_USE, EntitySpawnReason.DISPENSER, EntitySpawnReason.COMMAND,
                EntitySpawnReason.BREEDING)) {
            var seeds = new ArrayList<Integer>();
            var babyWins = 0;
            var kept = true;
            for (int seed = 0; seed < 1000; seed++) {
                for (boolean baby : new boolean[]{false,true}) {
                    var fox = EntityTypes.FOX.create(level, reason);
                    fox.snapTo(8.5, 102, 8.5, 0, 0);
                    if (baby) fox.setAge(-24000);
                    fox.getRandom().setSeed(0x9e3779b97f4a7c15L * (seed + 1L));
                    fox.finalizeSpawn(level, difficulty, reason, null);
                    if (!fox.getMainHandItem().is(NaturalistRegistry.MUSIC_DISC_WILD_ONES.get())) continue;
                    if (baby) babyWins++;
                    else seeds.add(seed);
                    kept &= !fox.canPickUpLoot() && fox.getDropChances().byEquipment(EquipmentSlot.MAINHAND) == 2.0F;
                }
            }
            res.put(reason.name(), List.of(seeds, babyWins, kept));
        }
        return res;
    }
}
