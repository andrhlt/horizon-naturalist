package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.ant.Ant;
import com.crispytwig.naturalist.world.level.block.AntHillBlock;
import com.crispytwig.naturalist.world.level.block.entity.AntHillBlockEntity;
import com.crispytwig.naturalist.world.level.levelgen.feature.AntHillFeature;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.network.chat.Component;
import net.minecraft.util.RandomSource;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.InteractionResult;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.item.context.UseOnContext;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;
import java.util.ArrayList;
import java.util.UUID;

@Mod("naturalist_fixture")
public final class AntProbe {
    public AntProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("ant_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var player = FakePlayerFactory.getMinecraft(level);
            level.addNewPlayer(player);
            player.setGameMode(GameType.SURVIVAL);
            try {
                var base = new BlockPos(8, 93, 8);
                var support = base.below();
                var queen = new ItemStack(NaturalistRegistry.QUEEN_ANT.get());
                player.setItemInHand(InteractionHand.MAIN_HAND, queen);
                var hit = new BlockHitResult(new Vec3(8.5, 93, 8.5), Direction.UP, support, false);
                var use = new UseOnContext(player, InteractionHand.MAIN_HAND, hit);
                level.setBlockAndUpdate(base.above(), Blocks.STONE.defaultBlockState());
                boolean blocked = queen.getItem().useOn(use) == InteractionResult.FAIL && queen.getCount() == 1;
                level.setBlockAndUpdate(base.above(), Blocks.AIR.defaultBlockState());
                var queenItem = NaturalistRegistry.QUEEN_ANT.get();
                boolean consumed = queenItem.useOn(use) == InteractionResult.CONSUME && queen.isEmpty();

                int blocks = 0, open = 0, queens = 0, linked = 0;
                BlockPos entrance = null;
                for (var pos : BlockPos.betweenClosed(base.offset(-1, 0, -1), base.offset(1, 2, 1))) {
                    var state = level.getBlockState(pos);
                    if (!(state.getBlock() instanceof AntHillBlock)) continue;
                    blocks++;
                    if (state.getValue(AntHillBlock.OPEN)) {
                        open++;
                        if (entrance == null && state.getValue(AntHillBlock.WORKERS) == 0) {
                            entrance = pos.immutable();
                        }
                    }
                    if (state.getValue(AntHillBlock.HAS_QUEEN)) queens++;
                    if (level.getBlockEntity(pos) instanceof AntHillBlockEntity hill
                            && player.getUUID().equals(hill.getOwner())) linked++;
                }
                if (entrance == null) throw new IllegalStateException("Missing open entrance");
                var hill = (AntHillBlockEntity) level.getBlockEntity(entrance);

                var worker = NaturalistEntityTypes.ANT.get().create(level, EntitySpawnReason.COMMAND);
                worker.snapTo(5.5, 93, 8.5, 0, 0);
                level.addFreshEntity(worker);
                boolean owner_check = !AntHillBlock.tryEnter(level, entrance, worker);
                worker.setTame(true, false);
                worker.setOwnerUUID(player.getUUID());
                worker.setHealth(3);
                var sugar = new ItemStack(Items.SUGAR, 2);
                player.setItemInHand(InteractionHand.MAIN_HAND, sugar);
                var heal = worker.mobInteract(player, InteractionHand.MAIN_HAND);
                boolean heals = heal == InteractionResult.SUCCESS && worker.getHealth() == 4 && sugar.getCount() == 1;

                var apples = new ItemEntity(level, 5.5, 93, 8.5, new ItemStack(Items.APPLE, 2));
                level.addFreshEntity(apples);
                var pickup = Ant.class.getDeclaredMethods();
                for (var method : pickup) {
                    if (!method.getName().equals("pickUpOneFood")) continue;
                    try {
                        method.setAccessible(true);
                        method.invoke(worker, apples);
                    } catch (ReflectiveOperationException error) {
                        throw new IllegalStateException(error);
                    }
                }
                var carried = worker.getCarriedFood();
                boolean food_rules = Ant.isFoodItem(new ItemStack(Items.APPLE))
                        && !Ant.isFoodItem(new ItemStack(Items.SUGAR));
                boolean carried_one = carried != null && carried.getItem().getCount() == 1
                        && apples.getItem().getCount() == 1 && carried.resolveAnt() == worker
                        && Math.abs(carried.getY() - worker.getY() - worker.getBbHeight() - 0.05) < 0.00001;
                AntHillBlock.storeFood(level, entrance, worker);
                boolean stored = worker.getCarriedFood() == null && hill.getStorage().countItem(Items.APPLE) == 1;
                worker.onCarriedFoodTaken(player);
                boolean owner_safe = worker.getTarget() == null;
                boolean entered = AntHillBlock.tryEnter(level, entrance, worker) && worker.isRemoved()
                        && level.getBlockState(entrance).getValue(AntHillBlock.WORKERS) == 1;
                var released = AntHillBlock.releaseAnt(level, entrance, level.getRandom(), player.getUUID());
                var wild = AntHillBlock.releaseAnt(level, entrance, level.getRandom(), null);
                wild.onCarriedFoodTaken(player);
                var fixed = new BlockPos(4, 93, 4);
                AntHillFeature.placeSmallHill(level, fixed, RandomSource.create(42L), null);
                var layout = new ArrayList<String>();
                for (var pos : BlockPos.betweenClosed(fixed.offset(-1, 0, -1), fixed.offset(1, 2, 1))) {
                    var state = level.getBlockState(pos);
                    if (!(state.getBlock() instanceof AntHillBlock)) continue;
                    layout.add("[%d,%d,%d,%s,%d,%s]".formatted(pos.getX() - fixed.getX(),
                            pos.getY() - fixed.getY(), pos.getZ() - fixed.getZ(),
                            state.getValue(AntHillBlock.OPEN), state.getValue(AntHillBlock.WORKERS),
                            state.getValue(AntHillBlock.HAS_QUEEN)));
                }
                var gatherers = new ArrayList<Ant>();
                for (int lane = 0; lane < 4; lane++) {
                    var gatherer = NaturalistEntityTypes.ANT.get().create(level, EntitySpawnReason.COMMAND);
                    gatherer.snapTo(2.5 + lane * 3, 93, 13.5, 0, 0);
                    gatherer.startHillCooldown();
                    level.addFreshEntity(gatherer);
                    gatherers.add(gatherer);
                }
                for (var gatherer : gatherers) {
                    var nearby = new ItemEntity(level, gatherer.getX() + 0.18, 93, 13.5, new ItemStack(Items.APPLE));
                    nearby.setNoPickUpDelay();
                    nearby.setNoGravity(true);
                    level.addFreshEntity(nearby);
                }
                for (int tick = 0; tick < 40; tick++) {
                    for (var gatherer : gatherers) level.tickNonPassenger(gatherer);
                }
                var naturalGather = new ArrayList<String>();
                for (var gatherer : gatherers) {
                    naturalGather.add("[%d,%s]".formatted(gatherer.getId() & 1, gatherer.isCarryingFood()));
                }
                var breakChecks = new ArrayList<String>();
                for (boolean sameOwner : new boolean[]{true, false}) {
                    var pos = new BlockPos(sameOwner ? 1 : 13, 93, 3);
                    var bounds = new AABB(pos).inflate(1.5);
                    var owner = sameOwner ? player.getUUID() : new UUID(17, 31);
                    var existing = level.getEntitiesOfClass(Ant.class, bounds).stream()
                            .map(Ant::getId).collect(java.util.stream.Collectors.toSet());
                    var existingItems = level.getEntitiesOfClass(ItemEntity.class, bounds).stream()
                            .map(ItemEntity::getId).collect(java.util.stream.Collectors.toSet());
                    var hillState = NaturalistRegistry.ANT_HILL.get().defaultBlockState()
                            .setValue(AntHillBlock.OPEN, true).setValue(AntHillBlock.WORKERS, 2);
                    level.setBlockAndUpdate(pos, hillState);
                    var heldHill = (AntHillBlockEntity) level.getBlockEntity(pos);
                    heldHill.setOwner(owner);
                    heldHill.getStorage().setItem(0, new ItemStack(Items.APPLE, 2));
                    hillState.getBlock().playerWillDestroy(level, pos, hillState, player);
                    level.removeBlock(pos, false);
                    var ants = level.getEntitiesOfClass(Ant.class, bounds).stream()
                            .filter(ant -> !existing.contains(ant.getId())).toList();
                    int tamed = 0, angry = 0;
                    for (var ant : ants) {
                        if (ant.isTame() && owner.equals(ant.getOwnerUUID())) tamed++;
                        if (ant.getPersistentAngerTarget() != null && player.getUUID().equals(ant.getPersistentAngerTarget().getUUID())
                                && ant.getPersistentAngerEndTime() - level.getGameTime() == 500) angry++;
                    }
                    int applesDropped = level.getEntitiesOfClass(ItemEntity.class, bounds).stream()
                            .filter(item -> !existingItems.contains(item.getId()) && item.getItem().is(Items.APPLE)).mapToInt(item -> item.getItem().getCount()).sum();
                    breakChecks.add("{\"same_owner\":%s,\"released\":%d,\"tamed\":%d,\"angry\":%d,\"apples\":%d}"
                            .formatted(sameOwner, ants.size(), tamed, angry, applesDropped));
                }
                var result = """
                    {"blocked":%s,"consumed":%s,"blocks":%d,"open_in_range":%s,"queens":%d,"owner_links":%d,"owner_check":%s,"sugar_heals":%s,"food_rules":%s,"carried_one":%s,"stored":%s,"owner_safe":%s,"entered":%s,"released_tamed":%s,"released_owner":%s,"released_owned_by_actor":%s,"wild_angry_ticks":%d,"portals":%s,"variant":"%s","width":%s,"height":%s,"max_health":%s,"movement_speed":%s,"attack_damage":%s,"follow_range":%s,"seed_42_layout":[%s],"live_gather":[%s],"hill_breaks":[%s]}
                    """.strip().formatted(blocked, consumed, blocks, open >= 4 && open <= 6, queens, linked,
                        owner_check, heals, food_rules, carried_one, stored, owner_safe, entered,
                        released.isTame(), player.getUUID().equals(released.getOwnerUUID()), released.isOwnedBy(player), wild.getPersistentAngerEndTime() - level.getGameTime(),
                        wild.canUsePortal(false), wild.getVariantString(), wild.getBbWidth(), wild.getBbHeight(),
                        wild.getMaxHealth(), wild.getAttributeValue(Attributes.MOVEMENT_SPEED),
                        wild.getAttributeValue(Attributes.ATTACK_DAMAGE), wild.getAttributeValue(Attributes.FOLLOW_RANGE),
                        String.join(",", layout), String.join(",", naturalGather), String.join(",", breakChecks));
                source.sendSuccess(() -> Component.literal("ant_fixture " + result), false);
                return 1;
            } finally {
                level.removePlayerImmediately(player, Entity.RemovalReason.DISCARDED);
            }
        }));
    }
}
