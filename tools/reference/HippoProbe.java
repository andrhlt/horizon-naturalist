package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.hippo.Hippo;
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
public final class HippoProbe {
    private static final Gson GSON = new Gson();

    public HippoProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {

        event.getDispatcher().register(Commands.literal("hippo_fixture").executes(ctx -> {
            var source = ctx.getSource();
            try {
                var output = new LinkedHashMap<String, Object>();
                var level = source.getLevel();
                var adult = add(level, 6.5, 92, 5.5);
                var baby = add(level, 8.5, 92, 5.5);
                baby.setAge(-24000);
                output.put("variant", adult.getVariantString());
                output.put("adult_dims", dimensions(adult));
                output.put("baby_dims", dimensions(baby));
                output.put("health", adult.getAttributeBaseValue(Attributes.MAX_HEALTH));
                output.put("follow_range", adult.getAttributeBaseValue(Attributes.FOLLOW_RANGE));
                output.put("speed", adult.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
                output.put("attack", adult.getAttributeBaseValue(Attributes.ATTACK_DAMAGE));
                output.put("knockback_resistance", adult.getAttributeBaseValue(Attributes.KNOCKBACK_RESISTANCE));
                output.put("step", adult.getAttributeBaseValue(Attributes.STEP_HEIGHT));
                output.put("knockback", new int[]{knockback(adult, level), knockback(baby, level)});
                output.put("adult_float_threshold", Math.round(adult.getFluidJumpThreshold() * 100));
                output.put("baby_float_threshold", Math.round(baby.getFluidJumpThreshold() * 100));
                output.put("portal", adult.canUsePortal(false));
                output.put("cluster", adult.getMaxSpawnClusterSize());
                output.put("food", new boolean[]{
                    adult.isFood(new ItemStack(Items.MELON)),
                    adult.isFood(new ItemStack(Items.MELON_SLICE)),
                    adult.isFood(new ItemStack(Items.GLISTERING_MELON_SLICE)),
                    adult.isFood(new ItemStack(Items.CARROT)),
                });

                var owner = spawnPlayer(level);
                var food = new ItemStack(Items.MELON, 8);
                owner.setItemInHand(InteractionHand.MAIN_HAND, food);
                adult.mobInteract(owner, InteractionHand.MAIN_HAND);
                output.put("adult_tame", adult.isTame());
                output.put("adult_in_love", adult.getInLoveTime());
                output.put("adult_carrying_melon", adult.getMainHandItem().is(Items.MELON));
                output.put("adult_carried_count", adult.getMainHandItem().getCount());
                for (int i = 0; i < 10; i++) adult.aiStep();
                output.put("melon_cleared_at_10", adult.getMainHandItem().isEmpty());
                adult.aiStep();
                output.put("melon_cleared_at_11", adult.getMainHandItem().isEmpty());

                baby.mobInteract(owner, InteractionHand.MAIN_HAND);
                output.put("baby_tamed", baby.isTame() && baby.isOwnedBy(owner));
                output.put("baby_ordered_to_sit", baby.isOrderedToSit());
                output.put("taming_did_not_age", baby.getAge() == -24000);
                output.put("following_at_tame", baby.isFollowingOwner());
                baby.hurtServer(level, level.damageSources().generic(), 7f);
                float hurtHealth = baby.getHealth();
                baby.mobInteract(owner, InteractionHand.MAIN_HAND);
                output.put("melon_heal", Math.round(baby.getHealth() - hurtHealth));
                baby.setHealth(baby.getMaxHealth());
                baby.mobInteract(owner, InteractionHand.MAIN_HAND);
                output.put("healthy_baby_age", baby.getAge());

                owner.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(NaturalistRegistry.WHISTLE.get()));
                baby.mobInteract(owner, InteractionHand.MAIN_HAND);
                output.put("whistle_unseats_and_stops_follow", !baby.isOrderedToSit() && !baby.isFollowingOwner());
                owner.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
                baby.mobInteract(owner, InteractionHand.MAIN_HAND);
                output.put("plain_empty_sits", baby.isOrderedToSit());
                owner.setShiftKeyDown(true);
                owner.setPose(Pose.CROUCHING);
                baby.mobInteract(owner, InteractionHand.MAIN_HAND);
                output.put("crouch_empty_sits", baby.isOrderedToSit());
                owner.setShiftKeyDown(false);
                owner.setPose(Pose.STANDING);

                var mate = add(level, 7.5, 92, 5.5);
                adult.setInLoveTime(600);
                mate.setInLoveTime(600);
                output.put("can_mate_dry", adult.canMate(mate));
                adult.snapTo(7.5, 92.5, 10.5, 0, 0);
                var refresh = net.minecraft.world.entity.Entity.class.getDeclaredMethod("updateFluidInteraction");
                refresh.setAccessible(true);
                refresh.invoke(adult);
                output.put("can_mate_one_wet", adult.canMate(mate));
                mate.snapTo(9.5, 92.5, 10.5, 0, 0);
                refresh.invoke(mate);
                output.put("both_in_water", adult.isInWater() && mate.isInWater());
                output.put("can_mate_both_wet", adult.canMate(mate));
                adult.discard();
                mate.discard();
                baby.discard();
                owner.discard();

                var normalBoat = EntityTypes.OAK_BOAT.create(level, EntitySpawnReason.COMMAND);
                var chestBoat = EntityTypes.OAK_CHEST_BOAT.create(level, EntitySpawnReason.COMMAND);
                var raft = EntityTypes.BAMBOO_RAFT.create(level, EntitySpawnReason.COMMAND);
                output.put("boat_adult_goal", boat(level, normalBoat, false));
                output.put("boat_destroyed", normalBoat.getRemovalReason() == net.minecraft.world.entity.Entity.RemovalReason.KILLED);
                output.put("chest_boat_adult_goal", boat(level, chestBoat, false));
                output.put("raft_adult_goal", boat(level, raft, false));
                var childBoat = EntityTypes.OAK_BOAT.create(level, EntitySpawnReason.COMMAND);
                output.put("boat_baby_goal", boat(level, childBoat, true));

                source.sendSuccess(() -> Component.literal("hippo_fixture " + GSON.toJson(output)), false);
            } catch (Exception error) {
                error.printStackTrace();
                source.sendSuccess(() -> Component.literal("hippo_fixture_error " + error), false);
            }
            return 1;
        }));
    }

    private static boolean boat(ServerLevel level, net.minecraft.world.entity.Entity target, boolean baby) throws Exception {
        var mob = add(level, 6.5, 92, 5.5);
        if (baby) mob.setAge(-24000);
        target.snapTo(7.4, 92, 6.4, 0, 0);
        target.setNoGravity(true);
        level.addFreshEntity(target);
        var clazz = Class.forName(Hippo.class.getName() + "$HippoAttackBoatsGoal");
        var factory = clazz.getDeclaredConstructor(net.minecraft.world.entity.PathfinderMob.class, double.class);
        factory.setAccessible(true);
        var goal = (net.minecraft.world.entity.ai.goal.Goal) factory.newInstance(mob, 1.25);
        boolean wanted = goal.canUse();
        if (wanted) {
            goal.start();
            goal.tick();
        }
        mob.discard();
        boolean alreadyDestroyed = target.isRemoved();
        if (!alreadyDestroyed) target.remove(net.minecraft.world.entity.Entity.RemovalReason.UNLOADED_TO_CHUNK);
        return wanted;
    }

    private static int[] dimensions(Hippo hippo) {
        return new int[]{ Math.round(hippo.getBbWidth() * 10000), Math.round(hippo.getBbHeight() * 10000),
            Math.round(hippo.getEyeHeight() * 10000) };
    }

    private static int knockback(Hippo hippo, ServerLevel level) {
        hippo.setDeltaMovement(Vec3.ZERO);
        hippo.knockback(0.4, 1.0, 0.0, level.damageSources().generic(), 0.0f, false);
        return (int) Math.round(Math.abs(hippo.getDeltaMovement().x) * 1000);
    }

    private static Hippo add(ServerLevel level, double x, double y, double z) {
        var mob = NaturalistEntityTypes.HIPPO.get().create(level, EntitySpawnReason.COMMAND);
        mob.snapTo(x, y, z, 0, 0);
        mob.setNoAi(true);
        mob.setNoGravity(true);
        level.addFreshEntity(mob);
        return mob;
    }
    private static ServerPlayer spawnPlayer(ServerLevel level) {
        var profile = new GameProfile(UUID.randomUUID(), "HippoFixture");
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        var connection = new Connection(PacketFlow.SERVERBOUND);
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), connection, player,
            CommonListenerCookie.createInitial(profile, false));
        player.snapTo(8.5, 92, 7.5, 0, 0);
        level.addNewPlayer(player);
        return player;
    }
}
