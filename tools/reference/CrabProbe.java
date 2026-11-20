package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.crab.Crab;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.LinkedHashMap;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.component.DataComponents;
import net.minecraft.network.Connection;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.EquipmentSlot;
import net.minecraft.world.entity.Pose;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.item.component.CustomData;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.JukeboxBlock;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class CrabProbe {
    private static final Gson GSON = new Gson();

    public CrabProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("crab_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            try {
                var player = spawnPlayer(level);
                var adult = add(level);
                var result = new LinkedHashMap<String, Object>();
                result.put("health", adult.getAttributeBaseValue(Attributes.MAX_HEALTH));
                result.put("speed", adult.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
                result.put("attack", adult.getAttributeBaseValue(Attributes.ATTACK_DAMAGE));
                result.put("variant", adult.getVariantString());
                result.put("adult_dims", dimensions(adult));
                result.put("cluster", adult.getMaxSpawnClusterSize());
                result.put("portal", adult.canUsePortal(false));
                result.put("adult_hides", adult.canHide());
                result.put("crab_food", adult.isFood(new ItemStack(Items.TROPICAL_FISH)));
                result.put("cod_food", adult.isFood(new ItemStack(Items.COD)));
                result.put("weapon_wanted", adult.wantsToPickUp(level, new ItemStack(Items.STONE_SWORD)));
                result.put("stick_wanted", adult.wantsToPickUp(level, new ItemStack(Items.STICK)));
                adult.hurtServer(level, level.damageSources().generic(), 4.0F);
                result.put("hidden_health", Math.round(adult.getHealth() * 100) / 100.0);
                adult.setDeltaMovement(Vec3.ZERO);
                adult.knockback(0.4, 1.0, 0.0, level.damageSources().generic(), 0.f, false);
                result.put("hidden_knockback", Math.round(Math.abs(adult.getDeltaMovement().x) * 1000));

                var baby = add(level);
                baby.setBaby(true);
                result.put("baby_hides", baby.canHide());
                result.put("baby_dims", dimensions(baby));
                baby.discard();
                var equipped = add(level);
                equipped.setItemSlot(EquipmentSlot.MAINHAND, new ItemStack(Items.STONE_SWORD));
                result.put("armed_hides", equipped.canHide());
                result.put("armed_wants_more", equipped.wantsToPickUp(level, new ItemStack(Items.IRON_AXE)));
                equipped.discard();

                player.setShiftKeyDown(true);
                player.setPose(Pose.CROUCHING);
                var sneaking = add(level);
                result.put("sneaker_hides", sneaking.canHide());
                sneaking.discard();
                player.setShiftKeyDown(false);
                player.setPose(Pose.STANDING);
                player.setItemInHand(InteractionHand.OFF_HAND, new ItemStack(Items.TROPICAL_FISH));
                var fed = add(level);
                result.put("food_holder_hides", fed.canHide());
                fed.discard();
                player.setItemInHand(InteractionHand.OFF_HAND, ItemStack.EMPTY);
                player.setGameMode(GameType.CREATIVE);
                var creative = add(level);
                result.put("creative_hides", creative.canHide());
                creative.discard();
                player.setGameMode(GameType.SURVIVAL);

                var item = new ItemEntity(level, 9.5, 92, 9.5, new ItemStack(Items.STONE_SWORD));
                item.setNoPickUpDelay();
                level.addFreshEntity(item);
                var armed_nearby = add(level);
                result.put("nearby_weapon_hides", armed_nearby.canHide());
                armed_nearby.discard();
                item.discard();
                adult.discard();

                var looter = add(level);
                looter.setNoAi(false);
                var droppedSword = new ItemEntity(level, 8.6, 92, 8.5, new ItemStack(Items.STONE_SWORD));
                droppedSword.setNoPickUpDelay();
                level.addFreshEntity(droppedSword);
                var droppedStick = new ItemEntity(level, 8.5, 92, 8.6, new ItemStack(Items.STICK));
                droppedStick.setNoPickUpDelay();
                level.addFreshEntity(droppedStick);
                looter.aiStep();
                result.put("weapon_picked", looter.getMainHandItem().is(Items.STONE_SWORD) && droppedSword.isRemoved());
                result.put("nonweapon_left", !droppedStick.isRemoved());
                var dummy = EntityTypes.ZOMBIE.create(level, EntitySpawnReason.COMMAND);
                dummy.snapTo(9.5, 92, 8.5, 0, 0);
                dummy.setNoAi(true);
                level.addFreshEntity(dummy);
                result.put("weapon_hit_landed", looter.doHurtTarget(level, dummy));
                result.put("weapon_damage_on_hit", looter.getMainHandItem().getDamageValue());
                dummy.discard();
                looter.discard();
                droppedStick.discard();

                var pet = add(level);
                var food = new ItemStack(Items.TROPICAL_FISH, 32);
                player.setItemInHand(InteractionHand.MAIN_HAND, food);
                for (int n = 0; n < 32 && !pet.isTame(); n++) {
                    pet.mobInteract(player, InteractionHand.MAIN_HAND);
                }
                result.put("can_tame", pet.isTame() && pet.isOwnedBy(player) && pet.isFollowingOwner());
                result.put("tame_hides", pet.canHide());
                pet.setHealth(4f);
                pet.mobInteract(player, InteractionHand.MAIN_HAND);
                result.put("food_heals", pet.getHealth() == 6f);
                player.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
                pet.mobInteract(player, InteractionHand.MAIN_HAND);
                result.put("empty_hand_sits", pet.isOrderedToSit());
                pet.mobInteract(player, InteractionHand.MAIN_HAND);
                player.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(NaturalistRegistry.WHISTLE.get()));
                pet.mobInteract(player, InteractionHand.MAIN_HAND);
                result.put("whistle_stops_following", !pet.isFollowingOwner());
                pet.setOrderedToSit(false);
                var music = new BlockPos(9, 92, 9);
                level.setBlockAndUpdate(music, Blocks.JUKEBOX.defaultBlockState().setValue(JukeboxBlock.HAS_RECORD, true));
                for (int i = 0; i < 20; i++) pet.tick();
                result.put("jukebox_dance", pet.isDancing());
                level.setBlockAndUpdate(music, Blocks.AIR.defaultBlockState());
                for (int i = 0; i < 20; i++) pet.tick();
                result.put("no_record_stops_dance", !pet.isDancing());

                pet.setVariantString("naturalist:yellow");
                pet.setAge(-900);
                pet.setOrderedToSit(true);
                var weapon = new ItemStack(Items.IRON_AXE);
                weapon.setDamageValue(7);
                pet.setItemSlot(EquipmentSlot.MAINHAND, weapon);
                var caught = pet.getCaughtItemStack();
                pet.saveToHandTag(caught);
                var data = caught.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
                result.put("caught_item", caught.getItem().builtInRegistryHolder().key().identifier().toString());
                result.put("caught_variant", data.getStringOr("Variant", ""));
                result.put("caught_age", data.getIntOr("Age", 0));
                result.put("caught_following", data.getBooleanOr("FollowingOwner", true));
                result.put("caught_sitting", data.getBooleanOr("Sitting", false));
                var released = add(level);
                released.loadFromHandTag(data);
                result.put("released_variant", released.getVariantString());
                result.put("released_age", released.getAge());
                result.put("released_owner", released.isTame() && released.isOwnedBy(player));
                result.put("released_max_health", released.getAttributeBaseValue(Attributes.MAX_HEALTH));
                result.put("released_weapon", released.getMainHandItem().is(Items.IRON_AXE) && released.getMainHandItem().getDamageValue() == 7);
                data.putInt("Variant", -1);
                released.loadFromHandTag(data);
                result.put("legacy_variant", released.getVariantString());
                var net = new ItemStack(NaturalistRegistry.CAPTURE_NET.get());
                player.setItemInHand(InteractionHand.MAIN_HAND, net);
                pet.mobInteract(player, InteractionHand.MAIN_HAND);
                result.put("capture_removes_crab", pet.isRemoved());
                result.put("capture_uses_one_durability", net.getDamageValue() == 1);
                result.put("capture_gives_crab", player.getInventory().countItem(NaturalistRegistry.CRAB.get()) == 1);
                released.discard();
                pet.discard();
                player.discard();
                source.sendSuccess(() -> Component.literal("crab_fixture " + GSON.toJson(result)), false);
            } catch (Exception error) {
                error.printStackTrace();
                source.sendSuccess(() -> Component.literal("crab_fixture_error " + error), false);
            }
            return 1;
        }));
    }

    private static int[] dimensions(Crab crab) {
        return new int[]{Math.round(crab.getBbWidth()*10000), Math.round(crab.getBbHeight()*10000),
            Math.round(crab.getEyeHeight()*10000)};
    }

    private static Crab add(ServerLevel level) {
        var mob = NaturalistEntityTypes.CRAB.get().create(level, EntitySpawnReason.COMMAND);
        mob.snapTo(8.5, 92, 8.5, 0, 0);
        mob.setNoAi(true);
        mob.setNoGravity(true);
        level.addFreshEntity(mob);
        return mob;
    }

    private static ServerPlayer spawnPlayer(ServerLevel level) {
        var profile = new GameProfile(UUID.randomUUID(), "CrabFixture");
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        var connection = new Connection(PacketFlow.SERVERBOUND);
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), connection, player,
            CommonListenerCookie.createInitial(profile, false));
        player.snapTo(8.5, 92, 7.5, 0, 0);
        level.addNewPlayer(player);
        return player;
    }
}
