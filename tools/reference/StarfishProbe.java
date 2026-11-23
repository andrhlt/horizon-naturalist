package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.starfish.Starfish;
import com.google.gson.Gson;
import java.util.LinkedHashMap;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.component.DataComponents;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.util.RandomSource;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.component.CustomData;
import net.minecraft.world.item.context.UseOnContext;
import net.minecraft.world.level.block.MultifaceBlock;
import net.minecraft.world.level.block.state.properties.BlockStateProperties;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class StarfishProbe {
    private static final Gson GSON = new Gson();

    public StarfishProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("starfish_fixture").executes(ctx -> {
            var source = ctx.getSource();
            ServerLevel level = source.getLevel();
            var fish = add(level, 8.5);
            var output = new LinkedHashMap<String, Object>();
            output.put("max_health", fish.getAttributeBaseValue(Attributes.MAX_HEALTH));
            output.put("speed", fish.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
            output.put("variant", fish.getVariantString());
            output.put("pushable", fish.isPushable());
            output.put("portal", fish.canUsePortal(false));
            output.put("cluster", fish.getMaxSpawnClusterSize());
            output.put("supported_water", allows(level, 8, 30, 8));
            output.put("open_water", allows(level, 8, 31, 8));
            output.put("sea_surface", allows(level, 8, 63, 8));

            fish.setVariantString("naturalist:purple");
            var stack = fish.getBucketItemStack();
            fish.saveToBucketTag(stack);
            var data = stack.getOrDefault(DataComponents.BUCKET_ENTITY_DATA, CustomData.EMPTY).copyTag();
            var custom = stack.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
            output.put("bucket_variant", data.getStringOr("Variant", ""));
            output.put("custom_variant", custom.getStringOr("Variant", ""));
            var released = NaturalistEntityTypes.STARFISH.get().create(level, EntitySpawnReason.BUCKET);
            released.loadFromBucketTag(data);
            output.put("released_variant", released.getVariantString());
            data.putInt("Variant", -1);
            released.loadFromBucketTag(data);
            output.put("legacy_variant", released.getVariantString());

            output.put("death_color", killAndRead(level, fish));
            var unknown = add(level, 13.5);
            unknown.setVariantString("other:fantasy");
            output.put("fallback_death_color", killAndRead(level, unknown));

            var support = new BlockPos(7, 91, 8);
            // Keep placement and neighbor updates inside a loaded test chunk;
            // otherwise the support break can be deferred and look like a
            // Starfish attachment failure in headless runs.
            level.getChunkAt(support);
            level.getChunkAt(support.above());
            var player = FakePlayerFactory.getMinecraft(level);
            player.snapTo(7.5, 92, 6.5, 0, 0);
            var item = new ItemStack(NaturalistRegistry.BLUE_STARFISH.get());
            player.setItemInHand(InteractionHand.MAIN_HAND, item);
            var click = new BlockHitResult(new Vec3(7.5, 92.0, 8.5), Direction.UP, support, false);
            var placed = item.getItem().useOn(new UseOnContext(player, InteractionHand.MAIN_HAND, click));
            var block = level.getBlockState(support.above());
            output.put("placed_block", placed.consumesAction()
                ? BuiltInRegistries.BLOCK.getKey(block.getBlock()).toString() : "");
            output.put("waterlogged", block.getOptionalValue(BlockStateProperties.WATERLOGGED).orElse(false));
            output.put("attached_down", block.getOptionalValue(MultifaceBlock.getFaceProperty(Direction.DOWN)).orElse(false));

            var placedPos = support.above();
            level.getEntitiesOfClass(ItemEntity.class,
                new AABB(7, 91, 8, 8, 93, 9)).forEach(ItemEntity::discard);
            level.destroyBlock(support, false);
            var afterSupportRemoval = level.getBlockState(placedPos);
            output.put("support_removed_block", BuiltInRegistries.BLOCK.getKey(afterSupportRemoval.getBlock()).toString());
            var supportDrops = level.getEntitiesOfClass(ItemEntity.class,
                new AABB(7, 91, 8, 8, 93, 9));
            output.put("support_drop", supportDrops.isEmpty() ? "" :
                BuiltInRegistries.ITEM.getKey(supportDrops.getFirst().getItem().getItem()).toString());
            output.put("support_drop_count", supportDrops.stream()
                .mapToInt(drop -> drop.getItem().getCount()).sum());

            source.sendSuccess(() -> Component.literal("starfish_fixture " + GSON.toJson(output)), false);
            return 1;
        }));
    }

    private static Starfish add(ServerLevel level, double x) {
        var fish = NaturalistEntityTypes.STARFISH.get().create(level, EntitySpawnReason.COMMAND);
        fish.snapTo(x, 92.0, 8.5, 0, 0);
        level.addFreshEntity(fish);
        return fish;
    }

    private static String killAndRead(ServerLevel level, Starfish fish) {
        var center = fish.position();
        var area = new AABB(center.add(-1, -1, -1), center.add(1, 1, 1));
        level.getEntitiesOfClass(ItemEntity.class, area).forEach(ItemEntity::discard);
        fish.hurtServer(level, level.damageSources().generic(), 100.0F);
        var drops = level.getEntitiesOfClass(ItemEntity.class, area);
        return drops.isEmpty() ? "" : BuiltInRegistries.ITEM.getKey(drops.getFirst().getItem().getItem()).toString();
    }

    private static boolean allows(ServerLevel level, int x, int y, int z) {
        return Starfish.checkStarfishSpawnRules(NaturalistEntityTypes.STARFISH.get(), level,
            EntitySpawnReason.NATURAL, new BlockPos(x, y, z), RandomSource.create(0));
    }
}
