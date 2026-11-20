package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.level.block.GlowGoopBlock;
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
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.Connection;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.Entity;
import net.minecraft.resources.Identifier;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.item.crafting.CraftingInput;
import net.minecraft.world.item.crafting.RecipeManager;
import net.minecraft.world.item.crafting.RecipeType;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.NaturalSpawner;
import net.minecraft.world.level.pathfinder.PathComputationType;
import net.minecraft.world.level.pathfinder.WalkNodeEvaluator;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.CampfireBlock;
import net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity;
import net.minecraft.world.level.block.entity.CampfireBlockEntity;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.Vec3;
import net.minecraft.world.phys.shapes.CollisionContext;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class GlowGoopProbe {
    public GlowGoopProbe() { NeoForge.EVENT_BUS.addListener(this::register); }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("glow_goop_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var result = capture(source.getLevel());
            source.sendSuccess(() -> Component.literal("glow_goop_fixture " + new Gson().toJson(result)), false);
            return 1;
        }));
    }

    private static ItemStack goop(int count) {
        return new ItemStack(NaturalistRegistry.GLOW_GOOP.get(), count);
    }

    private static String stack(ItemStack stack) {
        return stack.isEmpty() ? "empty" : BuiltInRegistries.ITEM.getKey(stack.getItem()) + "*" + stack.getCount();
    }

    private static String state(ServerLevel level, BlockPos at) {
        var state = level.getBlockState(at);
        if (!state.is(NaturalistRegistry.GLOW_GOOP_BLOCK.get())) {
            return BuiltInRegistries.BLOCK.getKey(state.getBlock()).toString();
        }
        return state.getValue(GlowGoopBlock.GOOP) + ":" + state.getValue(GlowGoopBlock.WATERLOGGED)
            + ":" + state.getLightEmission();
    }

    private static List<String> drops(ServerLevel level, BlockPos at) {
        var items = level.getEntitiesOfClass(ItemEntity.class, new AABB(at).inflate(2));
        var result = items.stream().map(item -> stack(item.getItem())).sorted().toList();
        items.forEach(Entity::discard);
        return result;
    }

    private static boolean place(ServerLevel level, ServerPlayer player, BlockPos hit) {
        return place(level, player, hit, InteractionHand.MAIN_HAND);
    }

    private static boolean place(ServerLevel level, ServerPlayer player, BlockPos hit, InteractionHand hand) {
        var contact = new BlockHitResult(Vec3.atCenterOf(hit).add(0, 0.5, 0), Direction.UP, hit, false);
        return player.gameMode.useItemOn(player, level, player.getItemInHand(hand), hand, contact).consumesAction();
    }

    private static final class PathProbe extends WalkNodeEvaluator {
        static String rawPath(ServerLevel level, BlockPos pos) {
            return getPathTypeFromState(level, pos).name();
        }
    }

    private static Map<String, List<Object>> pathRows(ServerLevel level, BlockPos at) {
        var result = new LinkedHashMap<String, List<Object>>();
        for (boolean wet : List.of(false, true)) {
            var goop = NaturalistRegistry.GLOW_GOOP_BLOCK.get().defaultBlockState()
                .setValue(GlowGoopBlock.WATERLOGGED, wet);
            level.setBlockAndUpdate(at, goop);
            result.put(Boolean.toString(wet), List.of(PathProbe.rawPath(level, at),
                goop.isPathfindable(PathComputationType.LAND), goop.isPathfindable(PathComputationType.WATER),
                goop.isPathfindable(PathComputationType.AIR),
                NaturalSpawner.isValidEmptySpawnBlock(level, at, goop, goop.getFluidState(), BuiltInRegistries.ENTITY_TYPE.getValue(Identifier.withDefaultNamespace("zombie")))));
        }
        level.removeBlock(at, false);
        return result;
    }

    private static Map<String, Object> capture(ServerLevel level) {
        var result = new LinkedHashMap<String, Object>();
        level.getEntitiesOfClass(ItemEntity.class, new AABB(0, 90, 0, 16, 98, 16)).forEach(Entity::discard);
        var profile = new GameProfile(UUID.randomUUID(), "GoopFixture");
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), new Connection(PacketFlow.SERVERBOUND),
            player, CommonListenerCookie.createInitial(profile, false));
        level.addNewPlayer(player);
        player.setGameMode(GameType.SURVIVAL);
        player.snapTo(8.5, 93, 6.5, 0, 0);
        try {
            result.put("ai_paths", pathRows(level, new BlockPos(14, 93, 14)));
            var at = new BlockPos(8, 93, 8);
            var layers = new ArrayList<String>();
            player.setItemInHand(InteractionHand.MAIN_HAND, goop(5));
            for (var hit : List.of(at.below(), at, at, at)) {
                layers.add(place(level, player, hit) + "/" + state(level, at) + "/" + state(level, at.above()));
            }
            result.put("stacking", layers);
            result.put("stack_remaining", stack(player.getMainHandItem()));
            var sneak = new BlockPos(4, 93, 8);
            level.setBlockAndUpdate(sneak, NaturalistRegistry.GLOW_GOOP_BLOCK.get().defaultBlockState());
            player.setShiftKeyDown(true);
            result.put("sneak_placement", List.of(place(level, player, sneak), state(level, sneak), state(level, sneak.above())));
            player.setShiftKeyDown(false);
            player.setItemInHand(InteractionHand.MAIN_HAND, goop(1));
            var layered = level.getBlockState(at);
            result.put("main_outline", !layered.getShape(level, at, CollisionContext.of(player)).isEmpty());
            result.put("collision", !layered.getCollisionShape(level, at, CollisionContext.of(player)).isEmpty());
            player.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
            result.put("empty_outline", !layered.getShape(level, at, CollisionContext.of(player)).isEmpty());
            player.setItemInHand(InteractionHand.OFF_HAND, goop(1));
            result.put("off_outline", !layered.getShape(level, at, CollisionContext.of(player)).isEmpty());
            player.setItemInHand(InteractionHand.OFF_HAND, ItemStack.EMPTY);
            level.removeBlock(at.above(), false);
            var mined = new ArrayList<String>();
            for (int i = 0; i < 3; ++i) {
                final boolean ok = player.gameMode.destroyBlock(at);
                mined.add(ok + "/" + state(level, at) + "/" + drops(level, at));
            }
            result.put("mine", mined);
            level.setBlockAndUpdate(at, layered);
            player.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(Items.STONE, 1));
            result.put("replace", List.of(place(level, player, at.below()), state(level, at), drops(level, at)));
            level.setBlockAndUpdate(at, layered);
            player.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(Items.CACTUS, 1));
            result.put("failed_replace", List.of(place(level, player, at.below()), state(level, at),
                drops(level, at), stack(player.getMainHandItem())));
            level.setBlockAndUpdate(at, Blocks.WATER.defaultBlockState());
            player.setItemInHand(InteractionHand.MAIN_HAND, goop(3));
            result.put("water", List.of(place(level, player, at.below()), state(level, at),
                place(level, player, at), state(level, at), player.gameMode.destroyBlock(at),
                state(level, at), drops(level, at), player.gameMode.destroyBlock(at), state(level, at), drops(level, at)));
            level.setBlockAndUpdate(at, layered);
            player.setGameMode(GameType.CREATIVE);
            result.put("creative", List.of(player.gameMode.destroyBlock(at), state(level, at), drops(level, at)));
            player.setGameMode(GameType.SURVIVAL);
            level.setBlockAndUpdate(at, NaturalistRegistry.GLOW_GOOP_BLOCK.get().defaultBlockState());
            player.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
            player.setItemInHand(InteractionHand.OFF_HAND, goop(1));
            result.put("offhand_stack_on_floor", List.of(place(level, player, at.below(), InteractionHand.OFF_HAND),
                state(level, at), stack(player.getOffhandItem())));

            var furnace = new BlockPos(6, 93, 8);
            var cooked = new LinkedHashMap<String, List<String>>();
            for (var oven : List.of(Blocks.FURNACE, Blocks.SMOKER)) {
                level.setBlockAndUpdate(furnace, oven.defaultBlockState());
                var be = (AbstractFurnaceBlockEntity) level.getBlockEntity(furnace);
                be.setItem(0, new ItemStack(Items.GLOW_BERRIES));
                be.setItem(1, new ItemStack(Items.COAL));
                int beforeOutput = oven == Blocks.SMOKER ? 99 : 199;
                for (int i = 0; i < beforeOutput; ++i) {
                    AbstractFurnaceBlockEntity.serverTick(level, furnace, level.getBlockState(furnace), be);
                }
                var before = stack(be.getItem(2));
                AbstractFurnaceBlockEntity.serverTick(level, furnace, level.getBlockState(furnace), be);
                cooked.put(BuiltInRegistries.BLOCK.getKey(oven).toString(), List.of(before, stack(be.getItem(2))));
            }
            result.put("cooking", cooked);
            var fire = new BlockPos(11, 93, 8);
            level.setBlockAndUpdate(fire, Blocks.CAMPFIRE.defaultBlockState().setValue(CampfireBlock.LIT, true));
            var campfire = (CampfireBlockEntity) level.getBlockEntity(fire);
            var berries = new ItemStack(Items.GLOW_BERRIES);
            var check = RecipeManager.createCheck(RecipeType.CAMPFIRE_COOKING);
            var accepted = campfire.placeFood(level, player, berries);
            for (int i = 0; i < 199; ++i) {
                CampfireBlockEntity.cookTick(level, fire, level.getBlockState(fire), campfire, check);
            }
            var before = drops(level, fire);
            CampfireBlockEntity.cookTick(level, fire, level.getBlockState(fire), campfire, check);
            result.put("campfire", List.of(accepted, stack(berries), before, drops(level, fire)));

            var slots = new ArrayList<ItemStack>();
            for (int i = 0; i < 9; ++i) slots.add(i == 4 ? new ItemStack(Items.ARROW) : goop(1));
            var grid = CraftingInput.of(3, 3, slots);
            var recipes = level.getServer().getRecipeManager();
            var crafted = recipes.getRecipeFor(RecipeType.CRAFTING, grid, level).orElseThrow();
            result.put("craft", List.of(crafted.id().identifier().toString(), stack(crafted.value().assemble(grid))));
            return result;
        } finally {
            level.removePlayerImmediately(player, Entity.RemovalReason.DISCARDED);
        }
    }
}
