package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.rat.Rat;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.UUID;
import net.minecraft.network.Connection;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.level.GameType;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.component.DataComponents;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.Container;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.item.component.CustomData;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.CropBlock;
import net.minecraft.world.level.block.NetherWartBlock;
import net.minecraft.world.phys.AABB;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class RatProbe {
    private static final BlockPos WORKSTATION = new BlockPos(9, 92, 5);
    private static final BlockPos WHEAT = new BlockPos(11, 92, 5);
    private static final BlockPos WART = new BlockPos(12, 92, 5);
    private static final AABB AREA = new AABB(3, 89, 2, 16, 100, 10);
    private static Rat worker;
    private static ServerPlayer owner;

    public RatProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("rat_fixture")
            .then(Commands.literal("setup").executes(ctx -> run(ctx.getSource(), true)))
            .then(Commands.literal("observe").executes(ctx -> run(ctx.getSource(), false))));
    }

    private int run(net.minecraft.commands.CommandSourceStack source, boolean setup) {
        try {
            Map<String, Object> sample = setup ? setup(source.getLevel()) : observe(source.getLevel());
            source.sendSuccess(() -> Component.literal("rat_fixture " + new Gson().toJson(sample)), false);
        } catch (Exception ex) {
            ex.printStackTrace();
            source.sendSuccess(() -> Component.literal("rat_fixture_error " + ex), false);
        }
        return 1;
    }

    private static Map<String, Object> setup(ServerLevel level) throws Exception {
        var out = new LinkedHashMap<String, Object>();
        for (var old : level.getEntitiesOfClass(Rat.class, AREA)) {
            old.discard();
        }
        var rat = NaturalistEntityTypes.RAT.get().create(level, EntitySpawnReason.COMMAND);
        rat.snapTo(6.5, 92, 5.5, 0, 0);
        rat.setNoAi(true);
        level.addFreshEntity(rat);
        out.put("adult_dims", size(rat));
        rat.setAge(-24000);
        out.put("baby_dims", size(rat));
        rat.setAge(0);
        out.put("attributes", new double[]{
            rat.getAttributeBaseValue(Attributes.MAX_HEALTH), rat.getAttributeBaseValue(Attributes.MOVEMENT_SPEED),
            rat.getAttributeBaseValue(Attributes.FOLLOW_RANGE),
            rat.getAttributeBaseValue(Attributes.STEP_HEIGHT)
        });
        out.put("attack_damage_exists", rat.getAttribute(Attributes.ATTACK_DAMAGE) != null);
        out.put("default_variant", rat.getVariantString());
        out.put("carry_slots", rat.getCarriedItems().getContainerSize());
        out.put("portal_allowed", rat.canUsePortal(false));
        var items = BuiltInRegistries.ITEM;
        var foods = items.stream().filter(item -> rat.isFood(new ItemStack(item)))
            .map(item -> items.getKey(item).toString()).sorted().toList();
        out.put("food_ids", foods);
        var food = items.stream().filter(item -> rat.isFood(new ItemStack(item))).findFirst().orElseThrow();
        if (owner != null) owner.discard();
        var profile = new GameProfile(UUID.randomUUID(), "RatFixture");
        owner = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        var connection = new Connection(PacketFlow.SERVERBOUND);
        owner.connection = new ServerGamePacketListenerImpl(level.getServer(), connection, owner,
                CommonListenerCookie.createInitial(profile, false));
        owner.snapTo(6.5, 92, 7.5, 0, 0);
        level.addNewPlayer(owner);
        owner.setGameMode(GameType.SURVIVAL);
        var tameRows = new ArrayList<List<Object>>();
        for (long seed : new long[]{1,42,100,4000,5000,9000,14000,18000,22000,36000}) {
            rat.setTame(false, false);
            rat.setOrderedToSit(false);
            rat.getRandom().setSeed(seed);
            var stack = new ItemStack(food, 2);
            owner.setItemInHand(InteractionHand.MAIN_HAND, stack);
            var result = rat.mobInteract(owner, InteractionHand.MAIN_HAND);
            tameRows.add(List.of(seed, result.consumesAction(), stack.getCount(), rat.isTame(), rat.isOrderedToSit()));
        }
        out.put("tame_rows", tameRows);
        rat.tame(owner);
        rat.setOrderedToSit(true);
        rat.setHealth(2);
        var foodStack = new ItemStack(food, 2);
        owner.setItemInHand(InteractionHand.MAIN_HAND, foodStack);
        rat.mobInteract(owner, InteractionHand.MAIN_HAND);
        out.put("heal", List.of(rat.getHealth(), foodStack.getCount()));

        var other = NaturalistEntityTypes.RAT.get().create(level, EntitySpawnReason.COMMAND);
        rat.setVariantString("naturalist:white");
        other.setVariantString("naturalist:brown");
        var baby = (Rat) rat.getBreedOffspring(level, other);
        out.put("offspring", List.of(baby.isTame(), baby.isOwnedBy(owner), baby.getVariantString()));

        rat.setFollowingOwner(true);
        rat.getCarriedItems().setItem(0, new ItemStack(Items.DIAMOND, 2));
        var hand = rat.getCaughtItemStack();
        rat.saveToHandTag(hand);
        var tag = hand.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
        out.put("short_tag", List.of(tag.getBooleanOr("Tame", false),
                tag.getBooleanOr("Sitting", false), tag.getBooleanOr("FollowingOwner", false),
                tag.getStringOr("Variant", ""), tag.getIntOr("Age", -1),
                tag.contains("Owner"), tag.contains("Workstation"), tag.contains("CarriedItems"), tag.contains("id")));

        level.setBlockAndUpdate(WORKSTATION, Blocks.CHEST.defaultBlockState());
        level.setBlockAndUpdate(WHEAT.below(), Blocks.FARMLAND.defaultBlockState());
        level.setBlockAndUpdate(WHEAT, Blocks.WHEAT.defaultBlockState().setValue(CropBlock.AGE, 7));
        level.setBlockAndUpdate(WART.below(), Blocks.SOUL_SAND.defaultBlockState());
        level.setBlockAndUpdate(WART, Blocks.NETHER_WART.defaultBlockState().setValue(NetherWartBlock.AGE, 3));
        tag.remove("NoAI");
        hand.set(DataComponents.CUSTOM_DATA, CustomData.of(tag));
        NaturalistRegistry.RAT.get().checkExtraContent(owner, level, hand, WORKSTATION);
        worker = level.getEntitiesOfClass(Rat.class, AREA, candidate -> candidate != rat).getFirst();
        out.put("released", List.of(worker.fromHand(), worker.isTame(), worker.isOwnedBy(owner),
                worker.isFollowingOwner(), worker.isOrderedToSit(), WORKSTATION.equals(worker.getWorkstationPos()),
                worker.getCarriedItems().isEmpty()));
        // Leave the item release test above on the real chest top. Start the
        // crop-to-container cycle on the neighboring walkable floor.
        worker.snapTo(10.5, 92, 5.5, 0, 0);
        owner.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
        rat.discard();
        return out;
    }

    private static Map<String, Object> observe(ServerLevel level) {
        var out = new LinkedHashMap<String, Object>();
        var chest = (Container) level.getBlockEntity(WORKSTATION);
        int wheat = 0;
        int wart = 0;
        for (int slot = 0; slot < chest.getContainerSize(); slot++) {
            var stack = chest.getItem(slot);
            if (stack.is(Items.WHEAT)) wheat += stack.getCount();
            if (stack.is(Items.NETHER_WART)) wart += stack.getCount();
        }
        out.put("crop_ages", List.of(level.getBlockState(WHEAT).getValue(CropBlock.AGE),
                level.getBlockState(WART).getValue(NetherWartBlock.AGE)));
        out.put("deposited", List.of(wheat, wart > 0));
        out.put("carried_empty", worker.getCarriedItems().isEmpty());
        out.put("near_workstation", worker.blockPosition().distSqr(WORKSTATION) <= 9.0);
        out.put("ticks", worker.tickCount);
        out.put("position", worker.position().toString());
        out.put("workstation_active", WORKSTATION.equals(worker.getWorkstationPos()));
        out.put("ai_enabled", !worker.isNoAi());
        out.put("can_sleep", worker.canSleep());
        out.put("sleeping", worker.isSleeping());
        out.put("sitting", worker.isOrderedToSit());
        out.put("on_ground", worker.onGround());
        return out;
    }

    private static int[] size(Rat rat) {
        return new int[]{Math.round(rat.getBbWidth() * 10000), Math.round(rat.getBbHeight() * 10000),
                Math.round(rat.getEyeHeight() * 10000)};
    }
}
