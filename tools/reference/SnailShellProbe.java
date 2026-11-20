package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.level.block.SnailShellBlock;
import com.crispytwig.naturalist.world.level.block.entity.SnailShellBlockEntity;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.DyeColor;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.component.CustomData;
import net.minecraft.core.component.DataComponents;
import net.minecraft.world.item.context.BlockPlaceContext;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class SnailShellProbe {
    public SnailShellProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("snail_shell_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var player = FakePlayerFactory.getMinecraft(level);
            var support = new BlockPos(8, 92, 8);
            var at = support.above();
            var hitSupport = new BlockHitResult(new Vec3(8.5, 93.0, 8.5), Direction.UP, support, false);
            var shellItem = SnailShellBlock.getShellStack(DyeColor.PURPLE);
            player.setYRot(45.0F);
            var block = NaturalistRegistry.SNAIL_SHELL_BLOCK.get();
            var shellState = block.getStateForPlacement(new BlockPlaceContext(player, InteractionHand.MAIN_HAND, shellItem, hitSupport));
            if (shellState == null || !level.setBlockAndUpdate(at, shellState)) {
                throw new IllegalStateException("Could not place shell");
            }
            if (!(level.getBlockEntity(at) instanceof SnailShellBlockEntity shell)) {
                throw new IllegalStateException("Missing Snail shell block entity");
            }

            var flowerItem = BuiltInRegistries.ITEM.getValue(Identifier.withDefaultNamespace("dandelion"));
            var cactusItem = BuiltInRegistries.ITEM.getValue(Identifier.withDefaultNamespace("cactus"));
            var flower = new ItemStack(flowerItem, 2);
            var cactus = new ItemStack(cactusItem, 2);
            var hitShell = new BlockHitResult(new Vec3(8.5, 93.5, 8.5), Direction.UP, at, false);
            shellState.useItemOn(cactus, level, player, InteractionHand.MAIN_HAND, hitShell);
            boolean refusesCactus = cactus.getCount() == 2 && shell.getFlower().isEmpty();
            shellState.useItemOn(flower, level, player, InteractionHand.MAIN_HAND, hitShell);
            boolean oneFlower = flower.getCount() == 1 && shell.getFlower().is(flowerItem) && shell.getFlower().getCount() == 1;
            shellState.useItemOn(flower, level, player, InteractionHand.MAIN_HAND, hitShell);
            boolean refusesSecond = flower.getCount() == 1 && shell.getFlower().getCount() == 1;
            boolean savedFlower = shell.getUpdateTag(level.registryAccess()).contains("Flower");
            level.destroyBlock(at, true);
            var drops = level.getEntitiesOfClass(ItemEntity.class, new AABB(at).inflate(2.0));
            boolean coloredDrop = drops.stream().anyMatch(entity -> {
                var stack = entity.getItem();
                var data = stack.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
                return stack.is(NaturalistRegistry.SNAIL_SHELL.get()) && data.getIntOr("Color", -1) == 10;
            });
            boolean flowerDrop = drops.stream().anyMatch(entity -> entity.getItem().is(flowerItem) && entity.getItem().getCount() == 1);
            String result = """
                {"color":"%s","rotation":%d,"refuses_cactus":%s,"one_flower":%s,"refuses_second":%s,"saves_flower":%s,"colored_drop":%s,"flower_drop":%s}
                """.strip().formatted(
                    shellState.getValue(SnailShellBlock.COLOR).getName(),
                    shellState.getValue(SnailShellBlock.ROTATION), refusesCactus, oneFlower,
                    refusesSecond, savedFlower, coloredDrop, flowerDrop);
            for (var drop : drops) drop.discard();
            source.sendSuccess(() -> Component.literal("snail_shell_fixture " + result), false);
            return 1;
        }));
    }
}
