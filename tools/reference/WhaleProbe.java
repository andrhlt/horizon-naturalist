package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.whale.Whale;
import com.crispytwig.naturalist.world.entity.ai.goal.WhaleDiveGoal;
import com.crispytwig.naturalist.world.entity.ai.goal.WhaleSeekDeeperWaterGoal;
import com.crispytwig.naturalist.world.entity.ai.goal.WhaleSurfaceGoal;
import com.crispytwig.naturalist.world.entity.ai.goal.WhaleSwimGoal;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.Arrays;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.Connection;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class WhaleProbe {
    public WhaleProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("whale_fixture").executes(ctx -> {
            var src = ctx.getSource();
            try {
                var level = src.getLevel();
                var dry = new BlockPos(8, 92, 6);
                var wet = new BlockPos(8, 93, 22);
                var contact = Entity.class.getDeclaredMethod("updateFluidInteraction");
                contact.setAccessible(true);

                var whale = create(level, dry);
                var out = new LinkedHashMap<String, Object>();
                out.put("variant", whale.getVariantString());
                out.put("adult_dims", dimensions(whale));
                out.put("max_health", whale.getAttributeBaseValue(Attributes.MAX_HEALTH));
                out.put("speed", whale.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
                out.put("range", whale.getAttributeBaseValue(Attributes.FOLLOW_RANGE));
                out.put("knockback_resistance", whale.getAttributeBaseValue(Attributes.KNOCKBACK_RESISTANCE));
                out.put("step", whale.getAttributeBaseValue(Attributes.STEP_HEIGHT));
                out.put("cluster", whale.getMaxSpawnClusterSize());
                out.put("pushable", whale.isPushable());
                out.put("current_push", whale.isPushedByFluid());
                whale.tick();
                out.put("adult_part_dims", partSizes(whale));
                out.put("adult_part_offsets", partOffsets(whale));
                whale.setAge(-24000);
                whale.tick();
                out.put("baby_dims", dimensions(whale));
                out.put("baby_part_dims", partSizes(whale));
                out.put("baby_part_offsets", partOffsets(whale));
                out.put("baby_max_health", whale.getMaxHealth());
                whale.setAge(0);
                out.put("regrowth_before_tick_max_health", whale.getMaxHealth());
                whale.tick();
                out.put("regrowth_after_tick_max_health", whale.getMaxHealth());

                var player = player(level);
                var food = List.of(Items.COD, Items.COOKED_COD, Items.SALMON, Items.COOKED_SALMON,
                    NaturalistRegistry.CRAB_MEAT.get(), NaturalistRegistry.COOKED_CRAB_MEAT.get(),
                    Items.TROPICAL_FISH, Items.PUFFERFISH, Items.BEEF);
                out.put("food", food.stream().map(item -> whale.isFood(new ItemStack(item))).toList());
                out.put("healing", food.stream().map(item -> {
                    whale.setHealth(60);
                    player.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(item, 2));
                    whale.mobInteract(player, InteractionHand.MAIN_HAND);
                    return List.of(Math.round(whale.getHealth() - 60), player.getMainHandItem().getCount());
                }).toList());

                var jellyfish = NaturalistEntityTypes.JELLYFISH.get().create(level, EntitySpawnReason.COMMAND);
                var puffer = EntityTypes.PUFFERFISH.create(level, EntitySpawnReason.COMMAND);
                var cod = EntityTypes.COD.create(level, EntitySpawnReason.COMMAND);
                whale.setHealth(80);
                out.put("jellyfish_immune", !whale.hurtServer(level, level.damageSources().mobAttack(jellyfish), 4));
                out.put("pufferfish_immune", !whale.hurtServer(level, level.damageSources().mobAttack(puffer), 4));
                out.put("cod_hit", whale.hurtServer(level, level.damageSources().mobAttack(cod), 4));
                out.put("cod_health", whale.getHealth());

                whale.setAirSupply(0);
                contact.invoke(whale);
                whale.baseTick();
                out.put("dry_air", whale.getAirSupply());
                out.put("dry_spray", whale.canSpray());
                var mate = create(level, dry.offset(3, 0, 0));
                whale.setInLove(player);
                mate.setInLove(player);
                out.put("mate_dry", whale.canMate(mate));
                whale.snapTo(8.5, 93.0, 22.5, 0, 0);
                contact.invoke(whale);
                out.put("mate_one_wet", whale.canMate(mate));
                mate.snapTo(10.5, 93.0, 23.5, 0, 0);
                contact.invoke(mate);
                out.put("mate_both_wet", whale.canMate(mate));
                out.put("offspring", ((Whale) whale.getBreedOffspring(level, mate)).getVariantString());
                whale.baseTick();
                out.put("wet_air", whale.getAirSupply());
                out.put("deep_spray", whale.canSpray());
                whale.snapTo(8.5, 96.0, 22.5, 0, 0);
                contact.invoke(whale);
                out.put("surface_spray", whale.canSpray());

                var type = NaturalistEntityTypes.WHALE.get();
                var first = type.create(level, EntitySpawnReason.NATURAL);
                var second = type.create(level, EntitySpawnReason.NATURAL);
                var third = type.create(level, EntitySpawnReason.NATURAL);
                var difficulty = level.getCurrentDifficultyAt(wet);
                var group = first.finalizeSpawn(level, difficulty, EntitySpawnReason.NATURAL, null);
                group = second.finalizeSpawn(level, difficulty, EntitySpawnReason.NATURAL, group);
                third.finalizeSpawn(level, difficulty, EntitySpawnReason.NATURAL, group);
                out.put("natural_group_ages", List.of(first.getAge(), second.getAge(), third.getAge()));
                out.put("spawn_three_water", Whale.checkWhaleSpawnRules(type, level, EntitySpawnReason.NATURAL, wet, level.getRandom()));
                level.setBlockAndUpdate(wet.above(2), Blocks.AIR.defaultBlockState());
                out.put("spawn_two_water", Whale.checkWhaleSpawnRules(type, level, EntitySpawnReason.NATURAL, wet, level.getRandom()));
                level.setBlockAndUpdate(wet.above(), Blocks.AIR.defaultBlockState());
                out.put("spawn_one_water", Whale.checkWhaleSpawnRules(type, level, EntitySpawnReason.NATURAL, wet, level.getRandom()));
                out.put("spawn_dry", Whale.checkWhaleSpawnRules(type, level, EntitySpawnReason.NATURAL, dry, level.getRandom()));
                captureGoals(level, out);

                whale.discard();
                mate.discard();
                player.discard();
                src.sendSuccess(() -> Component.literal("whale_fixture " + new Gson().toJson(out)), false);
            } catch (Exception error) {
                error.printStackTrace();
                src.sendSuccess(() -> Component.literal("whale_fixture_error " + error), false);
            }
            return 1;
        }));
    }

    private static void captureGoals(ServerLevel level, Map<String, Object> out) throws Exception {
        var contact = Entity.class.getDeclaredMethod("updateFluidInteraction");
        contact.setAccessible(true);
        var whale = create(level, new BlockPos(8, 116, 5));
        contact.invoke(whale);
        var surface = new WhaleSurfaceGoal(whale);
        var dive = new WhaleDiveGoal(whale);
        var seek = new WhaleSeekDeeperWaterGoal(whale);
        out.put("surface_before_cooldown", surface.canUse());
        out.put("dive_before_cooldown", dive.canUse());
        whale.tickCount = 1000;
        out.put("surface_open", surface.canUse());
        var roof = new BlockPos(8, 124, 5);
        level.setBlockAndUpdate(roof, Blocks.STONE.defaultBlockState());
        out.put("surface_closed", surface.canUse());
        level.setBlockAndUpdate(roof, Blocks.AIR.defaultBlockState());
        surface.canUse();
        surface.start();
        var breach = WhaleSurfaceGoal.class.getDeclaredField("breach");
        breach.setAccessible(true);
        breach.setBoolean(surface, false);
        surface.tick();
        out.put("surface_wanted", wanted(whale));
        whale.snapTo(8.5, 121.25, 5.5, 0, 0);
        contact.invoke(whale);
        surface.canContinueToUse();
        surface.tick();
        out.put("surface_skim_velocity_y", whale.getDeltaMovement().y);
        whale.snapTo(8.5, 121.5, 5.5, 0, 0);
        contact.invoke(whale);
        surface.canContinueToUse();
        breach.setBoolean(surface, true);
        surface.tick();
        var velocity = whale.getDeltaMovement();
        out.put("surface_forced_breach_velocity", List.of(velocity.x, velocity.y, velocity.z));
        surface.stop();
        out.put("surface_after_stop", surface.canUse());

        whale.snapTo(8.5, 112.9, 5.5, 0, 0);
        contact.invoke(whale);
        out.put("dive_below_six_depth", dive.canUse());
        whale.snapTo(8.5, 113, 5.5, 0, 0);
        contact.invoke(whale);
        out.put("dive_at_six_depth", dive.canUse());
        whale.snapTo(8.5, 116, 5.5, 0, 0);
        contact.invoke(whale);
        dive.start();
        out.put("dive_state_started", whale.isDiving());
        dive.tick();
        out.put("dive_wanted", wanted(whale));
        dive.stop();
        out.put("dive_state_stopped", whale.isDiving());
        out.put("dive_after_stop", dive.canUse());

        whale.setDeltaMovement(Vec3.ZERO);
        out.put("seek_deep", seek.canUse());
        whale.snapTo(8.5, 122.75, 5.5, 0, 0);
        contact.invoke(whale);
        out.put("seek_shallow_still", seek.canUse());
        whale.setDeltaMovement(0.11, 0, 0);
        out.put("seek_shallow_moving", seek.canUse());
        whale.setDeltaMovement(Vec3.ZERO);
        seek.start();
        seek.tick();
        out.put("seek_wanted_depth_speed", List.of(whale.getMoveControl().getWantedY(),
            whale.getMoveControl().getSpeedModifier()));
        out.put("seek_shallow_continue", seek.canContinueToUse());
        var swim = new WhaleSwimGoal(whale, 1.0, 40);
        var water = WhaleSwimGoal.class.getDeclaredMethod("isOpenWater", Vec3.class);
        water.setAccessible(true);
        var center = new Vec3(8.5, 116, 5.5);
        out.put("swim_open", water.invoke(swim, center));
        level.setBlockAndUpdate(new BlockPos(10, 116, 5), Blocks.STONE.defaultBlockState());
        out.put("swim_blocked_lateral", water.invoke(swim, center));
        whale.discard();
    }

    private static List<Double> wanted(Whale whale) {
        var control = whale.getMoveControl();
        return List.of(control.getWantedX(), control.getWantedY(), control.getWantedZ(), control.getSpeedModifier());
    }

    private static List<Integer> dimensions(Whale whale) {
        return List.of(Math.round(whale.getBbWidth() * 10000), Math.round(whale.getBbHeight() * 10000),
            Math.round(whale.getEyeHeight() * 10000));
    }

    private static List<int[]> partSizes(Whale whale) {
        return Arrays.stream(whale.getMobParts())
            .map(part -> new int[]{Math.round(part.getBbWidth() * 100), Math.round(part.getBbHeight() * 100)})
            .toList();
    }

    private static List<Long> partOffsets(Whale whale) {
        return Arrays.stream(whale.getMobParts())
            .map(part -> Math.round((part.getZ() - whale.getZ()) * 100))
            .toList();
    }

    private static Whale create(ServerLevel level, BlockPos pos) {
        var whale = NaturalistEntityTypes.WHALE.get().create(level, EntitySpawnReason.COMMAND);
        whale.snapTo(pos.getX() + 0.5, pos.getY(), pos.getZ() + 0.5, 0, 0);
        whale.setNoAi(true);
        whale.setNoGravity(true);
        level.addFreshEntity(whale);
        return whale;
    }

    private static ServerPlayer player(ServerLevel level) {
        var profile = new GameProfile(UUID.randomUUID(), "WhaleFixture");
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), new Connection(PacketFlow.SERVERBOUND), player,
            CommonListenerCookie.createInitial(profile, false));
        player.snapTo(14.5, 92, 6.5, 0, 0);
        level.addNewPlayer(player);
        player.connection.markClientLoaded();
        player.setGameMode(GameType.SURVIVAL);
        return player;
    }
}
