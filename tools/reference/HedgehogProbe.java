package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.mojang.authlib.GameProfile;
import java.util.UUID;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.hedgehog.Hedgehog;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.Pose;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.item.component.CustomData;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.phys.AABB;
import net.minecraft.core.BlockPos;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayer;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class HedgehogProbe {
    public HedgehogProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private static Hedgehog make(ServerLevel level, double x, double z) {
        var hedgehog = NaturalistEntityTypes.HEDGEHOG.get().create(level, EntitySpawnReason.COMMAND);
        hedgehog.snapTo(x, 93, z, 0, 0);
        level.addFreshEntity(hedgehog);
        return hedgehog;
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("hedgehog_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var player = FakePlayerFactory.getMinecraft(level);
            level.addNewPlayer(player);
            player.setGameMode(GameType.SURVIVAL);
            player.snapTo(8.5, 93, 6.5, 0, 0);
            var guest = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "HedgehogGuest"));
            level.addNewPlayer(guest);
            guest.setGameMode(GameType.SURVIVAL);
            guest.snapTo(14.5, 93, 14.5, 0, 0);
            try {
                var pet = make(level, 8.5, 8.5);
                double width = pet.getBbWidth(), height = pet.getBbHeight();
                double eye = pet.getEyeHeight(), base = pet.getMaxHealth();
                double speed = pet.getAttributeValue(Attributes.MOVEMENT_SPEED);
                boolean hides = pet.canHide();
                player.setShiftKeyDown(true);
                player.setPose(Pose.CROUCHING);
                boolean sneak_safe = !make(level, 8.5, 8.5).canHide();
                player.setShiftKeyDown(false);
                player.setPose(Pose.STANDING);
                var berries = new ItemStack(Items.SWEET_BERRIES, 20);
                player.setItemInHand(InteractionHand.MAIN_HAND, berries);
                boolean holds_food = !make(level, 8.5, 8.5).canHide();
                guest.snapTo(8.5, 93, 5.5, 0, 0);
                boolean otherPlayerVisible = make(level, 8.5, 8.5).canHide();
                guest.snapTo(10.5, 93, 8.5, 0, 0);
                BlockPos occluder = new BlockPos(9, 93, 8);
                level.setBlockAndUpdate(occluder, Blocks.STONE.defaultBlockState());
                level.setBlockAndUpdate(occluder.above(), Blocks.STONE.defaultBlockState());
                boolean otherPlayerOccluded = !make(level, 8.5, 8.5).canHide();
                level.setBlockAndUpdate(occluder, Blocks.AIR.defaultBlockState());
                level.setBlockAndUpdate(occluder.above(), Blocks.AIR.defaultBlockState());
                guest.snapTo(14.5, 93, 14.5, 0, 0);
                for (int trial = 0; trial < 20 && !pet.isTame(); trial++) {
                    pet.mobInteract(player, InteractionHand.MAIN_HAND);
                }
                boolean tamed = pet.isTame() && pet.isOwnedBy(player) && pet.getDyeColor().getId() == 14;
                pet.setHealth(5);
                int beforeHeal = berries.getCount();
                pet.mobInteract(player, InteractionHand.MAIN_HAND);
                boolean healed = pet.getHealth() == 7 && berries.getCount() == beforeHeal - 1;
                var purple = new ItemStack(Items.DYE.purple());
                player.setItemInHand(InteractionHand.MAIN_HAND, purple);
                pet.mobInteract(player, InteractionHand.MAIN_HAND);
                boolean dyed = pet.getDyeColor().getId() == 10 && purple.isEmpty();
                var mate = make(level, 8.5, 9.5);
                mate.tame(player);
                pet.setInLove(player);
                mate.setInLove(player);
                boolean canMate = pet.canMate(mate);
                var infant = pet.getBreedOffspring(level, mate);
                boolean inherited = infant != null && infant.isOwnedBy(player) && infant.isTame()
                    && infant.getDyeColor() != null && infant.getDyeColor().getId() == 10;
                var shears = new ItemStack(Items.SHEARS);
                player.setItemInHand(InteractionHand.MAIN_HAND, shears);
                pet.mobInteract(player, InteractionHand.MAIN_HAND);
                boolean cleared = pet.getDyeColor() == null && shears.getDamageValue() == 1;
                player.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
                pet.mobInteract(player, InteractionHand.MAIN_HAND);
                boolean sat = pet.isOrderedToSit();
                var whistle = new ItemStack(NaturalistRegistry.WHISTLE.get());
                player.setItemInHand(InteractionHand.MAIN_HAND, whistle);
                pet.mobInteract(player, InteractionHand.MAIN_HAND);
                boolean switched = !pet.isFollowingOwner() && !pet.isOrderedToSit();
                var head = pet.getCaughtItemStack();
                pet.saveToHandTag(head);
                var hand = head.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
                boolean stored = hand.getFloatOr("Health", 0) == 7 && hand.getIntOr("DyeColor", 0) == -1
                    && hand.getIntOr("Age", -1) == 0 && !hand.getBooleanOr("FollowingOwner", true);
                boolean portal = pet.canUsePortal(false);
                boolean cactus = pet.isInvulnerableTo(level, level.damageSources().cactus());
                boolean bushes = pet.isInvulnerableTo(level, level.damageSources().sweetBerryBush());
                boolean fall = pet.causeFallDamage(20, 1, level.damageSources().fall());
                pet.setAge(-24000);
                double babyWidth = pet.getBbWidth(), babyHeight = pet.getBbHeight(), babyEye = pet.getEyeHeight();
                pet.setAge(0);
                var net = new ItemStack(NaturalistRegistry.CAPTURE_NET.get());
                player.setItemInHand(InteractionHand.MAIN_HAND, net);
                var wild = make(level, 8.5, 7.5);
                wild.mobInteract(player, InteractionHand.MAIN_HAND);
                boolean wildRejected = !wild.isRemoved() && net.getDamageValue() == 0;
                pet.mobInteract(player, InteractionHand.MAIN_HAND);
                boolean tameCaptured = pet.isRemoved() && net.getDamageValue() == 1;
                var caught = ItemStack.EMPTY;
                for (int slot = 0; slot < player.getInventory().getContainerSize(); slot++) {
                    var value = player.getInventory().getItem(slot);
                    if (value.is(NaturalistRegistry.HEDGEHOG.get())) caught = value;
                }
                boolean netData = caught.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag().contains("Owner");
                NaturalistRegistry.HEDGEHOG.get().checkExtraContent(player, level, caught, new BlockPos(6, 92, 8));
                var released = level.getEntitiesOfClass(Hedgehog.class, new AABB(6, 92, 8, 7, 94, 9)).getFirst();
                boolean releaseFields = released.isOwnedBy(player) && released.getHealth() == 7
                    && released.getDyeColor() == null && !released.isFollowingOwner() && released.fromHand()
                    && !released.isNoGravity() && !released.isRolling();
                boolean centerOnGround = released.getX() == 6.5 && released.getY() == 93.0 && released.getZ() == 8.5;
                var result = """
                    {"base_health":%s,"speed":%s,"width":%s,"height":%s,"eye":%s,"baby_width":%s,"baby_height":%s,"baby_eye":%s,"visible_player_hides":%s,"sneaking_prevents_hide":%s,"held_berries_prevents_hide":%s,"tamed_default_red":%s,"tamed_health_limit":%s,"owner_heals_two":%s,"purple_dye":%s,"shears_clear":%s,"empty_hand_sits":%s,"whistle_wanders_and_stands":%s,"stored_fields":%s,"health_bar":%s,"bar_width":%s,"fall_hurt":%s,"cactus_immune":%s,"bush_immune":%s,"can_portal":%s,"nearer_food_holder_other_visible":%s,"other_player_occluded":%s,"can_mate":%s,"offspring_owner_and_dye":%s,"wild_net_rejected":%s,"tame_net_captured":%s,"net_owns_caught_item":%s,"ground_release_fields":%s,"release_center_on_ground":%s}
                    """.strip().formatted(base, speed, width, height, eye, babyWidth, babyHeight, babyEye, hides,
                        sneak_safe, holds_food, tamed, pet.getMaxHealth(), healed, dyed, cleared, sat, switched,
                        stored, head.isBarVisible(), head.getBarWidth(), fall, cactus, bushes, portal, otherPlayerVisible, otherPlayerOccluded, canMate, inherited,
                        wildRejected, tameCaptured, netData, releaseFields, centerOnGround);
                source.sendSuccess(() -> Component.literal("hedgehog_fixture " + result), false);
                return 1;
            } finally {
                level.removePlayerImmediately(guest, Entity.RemovalReason.DISCARDED);
                level.removePlayerImmediately(player, Entity.RemovalReason.DISCARDED);
            }
        }));
    }
}
