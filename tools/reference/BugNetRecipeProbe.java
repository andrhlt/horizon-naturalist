package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.google.gson.Gson;
import java.util.LinkedHashMap;
import java.util.List;
import net.minecraft.commands.Commands;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.core.component.DataComponents;
import net.minecraft.core.UUIDUtil;

import net.minecraft.world.item.component.CustomData;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityType;
import net.minecraft.world.entity.EntityTypes;

import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class BugNetRecipeProbe {
    public BugNetRecipeProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    record Capture(boolean removed, int net_damage, List<Integer> water_slots, List<String> dropped,
                   String caught, boolean original_uuid) {}

    private static Capture capture(ServerLevel level, EntityType<? extends Mob> type,
                                   InteractionHand hand, boolean creative, int... waterSlots) {
        return capture(level, type, hand, creative, false, waterSlots);
    }

    private static Capture capture(ServerLevel level, EntityType<? extends Mob> type,
                                   InteractionHand hand, boolean creative, boolean mounted, int... waterSlots) {
        var player = FakePlayerFactory.getMinecraft(level);
        var inventory = player.getInventory();
        inventory.clearContent();
        player.getAbilities().instabuild = creative;
        var net = new ItemStack(NaturalistRegistry.CAPTURE_NET.get());
        player.setItemInHand(hand, net);
        for (int slot : waterSlots) inventory.setItem(slot, new ItemStack(Items.WATER_BUCKET));
        var mob = type.create(level, EntitySpawnReason.COMMAND);
        if (mob == null) throw new IllegalStateException("Missing " + type);
        var at = new Vec3(8.5, 94, 9.5);
        mob.snapTo(at.x, at.y, at.z, 0, 0);
        mob.setNoAi(true);
        if (!level.addFreshEntity(mob)) throw new IllegalStateException("Mob did not spawn");
        var pig = mounted ? EntityTypes.PIG.create(level, EntitySpawnReason.COMMAND) : null;
        if (pig != null) {
            pig.snapTo(at.x, at.y, at.z, 0, 0);
            pig.setNoAi(true);
            if (!level.addFreshEntity(pig) || !mob.startRiding(pig, true, false))
                throw new IllegalStateException("Bee could not mount the fixture pig");
        }
        player.interactOn(mob, hand, Vec3.ZERO);
        var drops = level.getEntitiesOfClass(ItemEntity.class, AABB.ofSize(at, 2, 2, 2));
        var caught = java.util.stream.IntStream.range(0, inventory.getContainerSize())
                .mapToObj(inventory::getItem)
                .filter(stack -> stack.getItem() instanceof com.crispytwig.naturalist.world.item.CaughtEntityItem)
                .findFirst().orElse(ItemStack.EMPTY);
        var tag = caught.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY);
        var output = tag.copyTag();
        var result = new Capture(mob.isRemoved(), net.getDamageValue(),
                java.util.stream.IntStream.range(0, inventory.getContainerSize())
                    .filter(i -> inventory.getItem(i).is(Items.WATER_BUCKET)).boxed().toList(),
                drops.stream().map(drop -> BuiltInRegistries.ITEM.getKey(drop.getItem().getItem()).toString())
                    .sorted().toList(),
                caught.isEmpty() ? "" : BuiltInRegistries.ITEM.getKey(caught.getItem()).toString(),
                output.read("UUID", UUIDUtil.CODEC).orElse(new java.util.UUID(0, 0))
                    .equals(mob.getUUID())
                    && output.getStringOr("id", "").equals(BuiltInRegistries.ENTITY_TYPE.getKey(type).toString()));
        mob.discard();
        if (pig != null) pig.discard();
        drops.forEach(ItemEntity::discard);
        inventory.clearContent();
        player.getAbilities().instabuild = false;
        return result;
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_net_recipes").executes(ctx -> {
            var level = ctx.getSource().getLevel();
            var results = new LinkedHashMap<String, Capture>();
            results.put("bee", capture(level, EntityTypes.BEE, InteractionHand.MAIN_HAND, false));
            results.put("bee_mounted", capture(level, EntityTypes.BEE, InteractionHand.MAIN_HAND, false, true));
            results.put("silverfish", capture(level, EntityTypes.SILVERFISH, InteractionHand.MAIN_HAND, false));
            results.put("endermite", capture(level, EntityTypes.ENDERMITE, InteractionHand.MAIN_HAND, false));
            results.put("cod_dry", capture(level, EntityTypes.COD, InteractionHand.MAIN_HAND, false));
            results.put("cod_first_bucket", capture(level, EntityTypes.COD, InteractionHand.MAIN_HAND, false, 2, 10));
            results.put("cod_creative_without_bucket", capture(level, EntityTypes.COD, InteractionHand.MAIN_HAND, true));
            results.put("cod_creative_with_bucket", capture(level, EntityTypes.COD, InteractionHand.MAIN_HAND, true, 10));
            results.put("cod_offhand", capture(level, EntityTypes.COD, InteractionHand.OFF_HAND, false, 10));
            results.put("cod_bucket_offhand", capture(level, EntityTypes.COD, InteractionHand.MAIN_HAND, false, 40));
            results.put("salmon", capture(level, EntityTypes.SALMON, InteractionHand.MAIN_HAND, false, 10));
            ctx.getSource().sendSuccess(() -> Component.literal("naturalist_fixture_net_recipes " + new Gson().toJson(results)), false);
            return 1;
        }));
    }
}
