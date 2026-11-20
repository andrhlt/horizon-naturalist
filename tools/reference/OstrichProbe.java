package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.ai.goal.EggLayingBreedGoal;
import com.crispytwig.naturalist.world.entity.ai.goal.LayEggGoal;
import com.crispytwig.naturalist.world.entity.animal.ostrich.Ostrich;
import com.crispytwig.naturalist.world.level.block.OstrichEggBlock;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Proxy;
import java.util.ArrayList;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.registries.Registries;
import net.minecraft.network.Connection;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.tags.BlockTags;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.Pose;
import net.minecraft.world.entity.SpawnPlacements;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.ai.goal.BreedGoal;
import net.minecraft.world.entity.ai.goal.MoveToBlockGoal;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.item.context.BlockPlaceContext;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.ServerLevelAccessor;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class OstrichProbe {
    public OstrichProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("ostrich_fixture").executes(ctx -> {
            var source = ctx.getSource();
            try {
                var result = capture(source.getLevel());
                source.sendSuccess(() -> Component.literal("ostrich_fixture " + new Gson().toJson(result)), false);
            } catch (Exception failure) {
                throw new IllegalStateException(failure);
            }
            return 1;
        }));
    }

    private static Ostrich add(ServerLevel level, double x, double z, boolean baby) {
        var actor = NaturalistEntityTypes.OSTRICH.get().create(level, EntitySpawnReason.COMMAND);
        actor.snapTo(x, 93, z, 0, 0);
        actor.setAge(baby ? -24000 : 0);
        level.addFreshEntity(actor);
        return actor;
    }

    private static double[] dimensions(Ostrich ostrich) {
        return new double[] {ostrich.getBbWidth(), ostrich.getBbHeight(), ostrich.getEyeHeight()};
    }

    private static Map<String, Object> spawnPlacement(ServerLevel level) {
        var registry = level.registryAccess().lookupOrThrow(Registries.BLOCK);
        var tagged = registry.get(BlockTags.SUBSTRATE_OVERWORLD).orElseThrow();
        var loadedTag = tagged.stream().map(holder -> holder.unwrapKey().orElseThrow().identifier().toString())
            .sorted().toList();
        var floors = new ArrayList<Block>();
        tagged.forEach(holder -> floors.add(holder.value()));
        floors.addAll(List.of(Blocks.SAND, Blocks.RED_SAND, Blocks.STONE));
        var brightness = new AtomicInteger();
        var world = (ServerLevelAccessor) Proxy.newProxyInstance(OstrichProbe.class.getClassLoader(),
            new Class<?>[] {ServerLevelAccessor.class}, (proxy, method, arguments) -> {
                if (method.getName().equals("getRawBrightness")) {
                    return brightness.get();
                }
                try {
                    return method.invoke(level, arguments);
                } catch (InvocationTargetException wrapped) {
                    throw wrapped.getCause();
                }
            });
        var rows = new LinkedHashMap<String, Object>();
        var floor = new BlockPos(4, 92, 4);
        var position = floor.above();
        var previous = level.getBlockState(floor);
        try {
            for (var block : floors) {
                level.setBlockAndUpdate(floor, block.defaultBlockState());
                var values = new ArrayList<Boolean>();
                for (int light : new int[] {8, 9, 15}) {
                    brightness.set(light);
                    values.add(SpawnPlacements.checkSpawnRules(NaturalistEntityTypes.OSTRICH.get(), world,
                        EntitySpawnReason.NATURAL, position, level.getRandom()));
                }
                rows.put(block.builtInRegistryHolder().key().identifier().toString(), values);
            }
        } finally {
            level.setBlockAndUpdate(floor, previous);
        }
        return Map.of("brightness", List.of(8, 9, 15), "tag", loadedTag, "outcomes", rows);
    }

    private static Map<String, Object> capture(ServerLevel level) throws Exception {
        var result = new LinkedHashMap<String, Object>();
        level.getEntitiesOfClass(Ostrich.class, new AABB(0, 90, 0, 16, 98, 16)).forEach(Entity::discard);
        var profile = new GameProfile(UUID.randomUUID(), "OstrichFixture");
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        var connection = new Connection(PacketFlow.SERVERBOUND);
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), connection, player,
            CommonListenerCookie.createInitial(profile, false));
        level.addNewPlayer(player);
        player.setGameMode(GameType.SURVIVAL);
        player.snapTo(8.5, 93, 6.5, 0, 0);
        try {
            var adult = add(level, 8.5, 8.5, false);
            var baby = add(level, 8.5, 9.5, true);
            result.put("adult_dimensions", dimensions(adult));
            result.put("baby_dimensions", dimensions(baby));
            result.put("variant", adult.getVariantString());
            result.put("attributes", new double[] {
                adult.getAttributeBaseValue(Attributes.MAX_HEALTH), adult.getAttributeBaseValue(Attributes.MOVEMENT_SPEED),
                adult.getAttributeBaseValue(Attributes.ATTACK_DAMAGE), adult.getAttributeBaseValue(Attributes.JUMP_STRENGTH),
                adult.getAttributeBaseValue(Attributes.STEP_HEIGHT), adult.getAttributeBaseValue(Attributes.FOLLOW_RANGE)
            });
            result.put("adult_hides", adult.canHide());
            result.put("baby_hides", baby.canHide());
            player.setPose(Pose.CROUCHING);
            player.setShiftKeyDown(true);
            result.put("sneak_hides", add(level, 9.5, 8.5, false).canHide());
            player.setPose(Pose.STANDING);
            player.setShiftKeyDown(false);
            var seeds = new ItemStack(Items.WHEAT_SEEDS, 10);
            player.setItemInHand(InteractionHand.MAIN_HAND, seeds);
            result.put("seeds_hide", add(level, 7.5, 8.5, false).canHide());
            adult.mobInteract(player, InteractionHand.MAIN_HAND);
            result.put("adult_seed_outcome", List.of(adult.isTame(), adult.isInLove(), seeds.getCount()));
            baby.mobInteract(player, InteractionHand.MAIN_HAND);
            result.put("juvenile_seed_outcome", List.of(baby.isOwnedBy(player), baby.isOrderedToSit(), seeds.getCount()));
            int previousAge = baby.getAge();
            baby.mobInteract(player, InteractionHand.MAIN_HAND);
            result.put("tame_baby_accelerates", baby.getAge() - previousAge);
            baby.setAge(0);
            baby.setOrderedToSit(false);
            baby.setInSittingPose(false);
            baby.setHealth(20);
            baby.mobInteract(player, InteractionHand.MAIN_HAND);
            result.put("owner_heals", List.of(baby.getHealth(), seeds.getCount()));
            var saddle = new ItemStack(Items.SADDLE);
            player.setItemInHand(InteractionHand.MAIN_HAND, saddle);
            baby.mobInteract(player, InteractionHand.MAIN_HAND);
            result.put("saddles_and_consumes", baby.isSaddled() && saddle.isEmpty());
            player.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
            baby.mobInteract(player, InteractionHand.MAIN_HAND);
            result.put("owner_rides", player.getVehicle() == baby && baby.getControllingPassenger() == player);
            baby.positionRider(player);
            result.put("rider_seat_offset", List.of(player.getX() - baby.getX(),
                player.getY() - baby.getY(), player.getZ() - baby.getZ()));
            player.stopRiding();
            var shears = new ItemStack(Items.SHEARS);
            player.setItemInHand(InteractionHand.MAIN_HAND, shears);
            baby.mobInteract(player, InteractionHand.MAIN_HAND);
            result.put("shears_remove_saddle", !baby.isSaddled() && shears.getDamageValue() == 1);
            var dye = new ItemStack(Items.DYE.purple());
            player.setItemInHand(InteractionHand.MAIN_HAND, dye);
            baby.mobInteract(player, InteractionHand.MAIN_HAND);
            result.put("owner_dyes", List.of(baby.getDyeColor().getId(), dye.getCount()));
            player.setItemInHand(InteractionHand.MAIN_HAND, shears);
            baby.mobInteract(player, InteractionHand.MAIN_HAND);
            result.put("owner_clears_dye", baby.getDyeColor() == null && shears.getDamageValue() == 2);
            var whistle = new ItemStack(NaturalistRegistry.WHISTLE.get());
            baby.setOrderedToSit(true);
            player.setItemInHand(InteractionHand.MAIN_HAND, whistle);
            baby.mobInteract(player, InteractionHand.MAIN_HAND);
            result.put("whistle_wander", List.of(baby.isFollowingOwner(), baby.isOrderedToSit(), whistle.getCount()));


            var mate = add(level, 10.5, 8.5, false);
            adult.setInLoveTime(600);
            mate.setInLoveTime(600);
            var breed = new EggLayingBreedGoal<>(adult, 1.0);
            var partner = BreedGoal.class.getDeclaredField("partner");
            partner.setAccessible(true);
            partner.set(breed, mate);
            var breedMethod = EggLayingBreedGoal.class.getDeclaredMethod("breed");
            breedMethod.setAccessible(true);
            breedMethod.invoke(breed);
            result.put("breed_sets_egg", adult.hasEgg() && !mate.hasEgg() && !adult.isInLove() && !mate.isInLove());
            result.put("egg_blocks_further_love", !adult.canFallInLove());
            var egg = (OstrichEggBlock) NaturalistRegistry.OSTRICH_EGG.get();
            BlockPos base = new BlockPos(8, 92, 8);
            BlockPos laid = base.above();
            level.setBlockAndUpdate(base, Blocks.SAND.defaultBlockState());
            var goal = new LayEggGoal<>(adult, 1.0);
            var target = MoveToBlockGoal.class.getDeclaredField("blockPos");
            target.setAccessible(true);
            target.set(goal, base);
            adult.setDeltaMovement(Vec3.ZERO);
            int layCalls = 0;
            while (adult.hasEgg() && layCalls < 100) {
                goal.tick();
                layCalls++;
            }
            var placed = level.getBlockState(laid);
            result.put("lay_calls", layCalls);
            result.put("owns_laid_egg", placed.is(egg) && adult.owns(laid));
            result.put("laid_egg_fresh", placed.is(egg) && placed.getValue(OstrichEggBlock.HATCH) == 0);
            result.put("can_attack_player", adult.canAttack(player));
            result.put("visible_egg_owners", level.getEntitiesOfClass(Ostrich.class, new AABB(laid).inflate(16, 8, 16), actor -> actor.owns(laid)).size());
            egg.playerWillDestroy(level, laid, placed, player);
            result.put("owned_egg_consumed", !adult.owns(laid));
            result.put("owned_egg_break_targets_player", adult.getTarget() == player && !adult.owns(laid));

            BlockPos hatch = new BlockPos(13, 93, 11);
            level.setBlockAndUpdate(hatch.below(), Blocks.STONE.defaultBlockState());
            level.setBlockAndUpdate(hatch, egg.defaultBlockState());
            var stages = new ArrayList<Integer>();
            int last = 0;
            for (int attempt = 0; attempt < 3000; attempt++) {
                var state = level.getBlockState(hatch);
                int progress = state.is(egg) ? state.getValue(OstrichEggBlock.HATCH) : -1;
                if (progress != last) {
                    stages.add(progress);
                    last = progress;
                }
                if (progress < 0) break;
                egg.randomTick(state, level, hatch, level.getRandom());
            }
            result.put("hatch_stages_on_stone", stages);
            var young = level.getEntitiesOfClass(Ostrich.class, new AABB(hatch).inflate(0.2), Ostrich::isAlive).getFirst();
            result.put("hatchling_data", List.of(young.getAge(), young.isTame(), young.getX(), young.getY(), young.getZ()));
            result.put("fall_hurts", young.causeFallDamage(20, 1, level.damageSources().fall()));
            var placementContext = new BlockPlaceContext(player, InteractionHand.MAIN_HAND,
                new ItemStack(egg.asItem()), new BlockHitResult(Vec3.atCenterOf(laid), Direction.UP, laid, false));
            result.put("physical_adjacent_egg_placement", !egg.canBeReplaced(placed, placementContext)
                && placementContext.getClickedPos().equals(laid.above()));

            int firstBabies = 0;
            int laterBabies = 0;
            var localDifficulty = level.getCurrentDifficultyAt(laid);
            for (int index = 0; index < 512; index++) {
                var first = NaturalistEntityTypes.OSTRICH.get().create(level, EntitySpawnReason.NATURAL);
                var later = NaturalistEntityTypes.OSTRICH.get().create(level, EntitySpawnReason.NATURAL);
                first.snapTo(6.5, 93, 8.5, 0, 0);
                later.snapTo(7.5, 93, 8.5, 0, 0);
                var group = first.finalizeSpawn(level, localDifficulty, EntitySpawnReason.NATURAL, null);
                later.finalizeSpawn(level, localDifficulty, EntitySpawnReason.NATURAL, group);
                if (first.isBaby()) firstBabies++;
                if (later.isBaby()) laterBabies++;
            }
            result.put("natural_group_age_counts", List.of(firstBabies, laterBabies, 512));
            result.put("natural_spawn_placement", spawnPlacement(level));
            return result;
        } finally {
            level.removePlayerImmediately(player, Entity.RemovalReason.DISCARDED);
        }
    }
}
